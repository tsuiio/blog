use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{schema::sorts, Conn, DbPool},
    error::BlogError,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = sorts)]
pub struct Sort {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub sort_order: i32,
    pub parent_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sorts)]
pub struct NewSort<'a> {
    id: Uuid,
    name: &'a str,
    content: &'a str,
    sort_order: i32,
    parent_id: Option<&'a Uuid>,
}

impl Sort {
    pub async fn create_sort(
        name: &str,
        content: &str,
        sort_order: i32,
        parent_id: Option<&Uuid>,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::sorts;

        let new_sort = NewSort {
            id: Uuid::new_v4(),
            name,
            content,
            sort_order,
            parent_id,
        };

        diesel::insert_into(sorts::table)
            .values(&new_sort)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn update_sort_by_uuid(
        id: &Uuid,
        name: &str,
        content: &str,
        sort_order: i32,
        parent_id: Option<&Uuid>,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::sorts;

        diesel::update(sorts::table.find(id))
            .set((
                sorts::content.eq(content),
                sorts::sort_order.eq(sort_order),
                sorts::name.eq(name),
                sorts::parent_id.eq(parent_id),
            ))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn get_sorts(conn: &mut Conn) -> Result<Vec<Sort>, BlogError> {
        use crate::db::schema::sorts;

        let sorts = sorts::table.select(Sort::as_select()).load(conn).await?;

        Ok(sorts)
    }

    pub async fn delete_sort(id: &Uuid, pool: DbPool) -> Result<(), BlogError> {
        use crate::db::schema::{note_sorts, sorts};
        let mut conn = pool.get_owned().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                diesel::delete(note_sorts::table.filter(note_sorts::sort_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(sorts::table.find(id)).execute(conn).await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }
}
