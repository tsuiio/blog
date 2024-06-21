use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        models::{notes::Status, pages::PageTy},
        schema::page_about,
        DbPool,
    },
    error::BlogError,
    utils::generate_random_string,
};

use super::Page;

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = page_about)]
pub struct AboutPage {
    pub id: Uuid,
    pub avatar_url: String,
    pub content: String,
}

impl Page for AboutPage {
    async fn create_page(
        &self,
        status: &Status,
        subname: &str,
        comm: bool,
        user_id: &Uuid,
        pool: DbPool,
    ) -> Result<(), BlogError> {
        use crate::db::schema::{page_about, pages, short_ids};

        let mut conn = pool.get().await?;
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let short_id = Uuid::new_v4();
                let short_name = generate_random_string(16);

                diesel::insert_into(short_ids::table)
                    .values((
                        short_ids::id.eq(short_id),
                        short_ids::short_name.eq(short_name),
                        short_ids::subname.eq(subname),
                    ))
                    .execute(conn)
                    .await?;

                let new_page_id = Uuid::new_v4();

                diesel::insert_into(pages::table)
                    .values((
                        pages::id.eq(new_page_id),
                        pages::page_type.eq(PageTy::About),
                        pages::status.eq(status),
                        pages::comm.eq(comm),
                        pages::user_id.eq(user_id),
                        pages::short_id.eq(short_id),
                    ))
                    .execute(conn)
                    .await?;

                let new_about_id = Uuid::new_v4();

                diesel::insert_into(page_about::table)
                    .values((
                        page_about::id.eq(new_about_id),
                        page_about::page_id.eq(new_page_id),
                        page_about::avatar_url.eq(&self.avatar_url),
                        page_about::content.eq(&self.content),
                    ))
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }

    async fn update_page(
        &self,
        id: &Uuid,
        status: &Status,
        subname: &str,
        comm: bool,
        pool: DbPool,
    ) -> Result<(), BlogError> {
        use crate::db::schema::{page_about, pages, short_ids};

        let mut conn = pool.get().await?;
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let now = Utc::now().naive_utc();
                diesel::update(pages::table.find(id))
                    .set((
                        pages::status.eq(status),
                        pages::comm.eq(comm),
                        pages::updated_at.eq(now),
                    ))
                    .execute(conn)
                    .await?;

                let short_id: Uuid = pages::table
                    .filter(pages::id.eq(id))
                    .select(pages::short_id)
                    .first(conn)
                    .await?;

                diesel::update(short_ids::table.filter(short_ids::id.eq(short_id)))
                    .set((
                        short_ids::subname.eq(subname),
                        short_ids::updated_at.eq(now),
                    ))
                    .execute(conn)
                    .await?;

                diesel::update(page_about::table.filter(page_about::page_id.eq(id)))
                    .set((
                        page_about::avatar_url.eq(&self.avatar_url),
                        page_about::content.eq(&self.content),
                    ))
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;
        Ok(())
    }
}
