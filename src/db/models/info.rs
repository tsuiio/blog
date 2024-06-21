use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::info, Conn},
    error::BlogError,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = info)]
pub struct Info {
    pub id: Uuid,
    pub bio: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = info)]
pub struct NewInfo<'a> {
    id: Uuid,
    bio: &'a str,
    title: &'a str,
}

impl Info {
    pub async fn create_info(bio_: &str, title_: &str, conn: &mut Conn) -> Result<(), BlogError> {
        use crate::db::schema::info::dsl::*;

        let id_ = uuid::Uuid::new_v4();
        let info_ = NewInfo {
            id: id_,
            bio: bio_,
            title: title_,
        };

        diesel::insert_into(info)
            .values(&info_)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn get_info(conn: &mut Conn) -> Result<Option<Info>, BlogError> {
        use crate::db::schema::info::dsl::*;

        let info_ = info
            .select(Info::as_select())
            .first(conn)
            .await
            .optional()?;

        Ok(info_)
    }

    pub async fn update_info(bio_: &str, title_: &str, conn: &mut Conn) -> Result<(), BlogError> {
        use crate::db::schema::info::dsl::*;

        let now = Utc::now().naive_utc();
        diesel::update(info)
            .set((
                bio.eq(bio_.to_owned()),
                title.eq(title_.to_owned()),
                updated_at.eq(now),
            ))
            .execute(conn)
            .await?;

        Ok(())
    }
}
