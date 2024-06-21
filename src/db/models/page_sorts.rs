use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::page_sorts, Conn},
    error::BlogError,
};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = page_sorts)]
#[diesel(primary_key(page_id, sort_id))]
pub struct PageSort {
    pub page_id: Uuid,
    pub sort_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = page_sorts)]
pub struct NewNoteSort<'a> {
    page_id: &'a Uuid,
    sort_id: &'a Uuid,
}

impl PageSort {
    pub async fn add_sort_to_page(
        page_id: &Uuid,
        sort_id: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::page_sorts;

        let new_page_sort = NewNoteSort { page_id, sort_id };

        diesel::insert_into(page_sorts::table)
            .values(&new_page_sort)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn remove_page_from_sort(
        page_id: &Uuid,
        sort_id: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::page_sorts;

        diesel::delete(page_sorts::table)
            .filter(
                page_sorts::page_id
                    .eq(page_id)
                    .and(page_sorts::sort_id.eq(sort_id)),
            )
            .execute(conn)
            .await?;

        Ok(())
    }
}
