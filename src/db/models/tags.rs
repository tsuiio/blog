use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{schema::tags, Conn, DbPool},
    error::BlogError,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = tags)]
#[diesel(primary_key(id))]
pub struct Tag {
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    id: Uuid,
    content: &'a str,
}

impl Tag {
    pub async fn create_tag(content: &str, conn: &mut Conn) -> Result<(), BlogError> {
        use crate::db::schema::tags;

        let new_tag = NewTag {
            id: Uuid::new_v4(),
            content,
        };

        diesel::insert_into(tags::table)
            .values(&new_tag)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn update_tag_by_uuid(
        id: &Uuid,
        content: &str,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::tags;

        let now = Utc::now().naive_utc();

        diesel::update(tags::table.find(id))
            .set((tags::content.eq(content), tags::updated_at.eq(now)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn get_tags(
        limit: i64,
        offset: i64,
        conn: &mut Conn,
    ) -> Result<Vec<Self>, BlogError> {
        use crate::db::schema::tags;

        let tags = tags::table
            .select(Tag::as_select())
            .order(tags::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Self>(conn)
            .await?;

        Ok(tags)
    }

    pub async fn delete_tag_by_uuid(id: &Uuid, pool: DbPool) -> Result<(), BlogError> {
        use crate::db::schema::{note_tags, tags};
        let mut conn = pool.get_owned().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                diesel::delete(note_tags::table.filter(note_tags::tag_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(tags::table.find(id)).execute(conn).await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }
}
