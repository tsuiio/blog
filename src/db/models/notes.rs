use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{
        schema::{notes, sql_types::PublishStatus},
        Conn, DbConn, DbPool,
    },
    error::BlogError,
    utils::generate_random_string,
};

#[derive(Debug, Queryable, Selectable, Insertable, PartialEq, Identifiable)]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub status: Status,
    pub summary: String,
    pub content: String,
    pub views: i64,
    pub comm: bool,
    pub user_id: Uuid,
    pub short_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    id: Uuid,
    title: &'a str,
    status: &'a Status,
    summary: &'a str,
    content: &'a str,
    views: i64,
    comm: bool,
    user_id: &'a Uuid,
    short_id: Uuid,
}

#[derive(Debug, PartialEq, DbEnum, Deserialize, Serialize, utoipa::ToSchema)]
#[ExistingTypePath = "PublishStatus"]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Public,
    Draft,
    Recycle,
}

impl Note {
    pub async fn create_note(
        subname: Option<&str>,
        status: &Status,
        title: &str,
        summary: &str,
        content: &str,
        comm: bool,
        user_id: &Uuid,
        pool: DbPool,
    ) -> Result<(), BlogError> {
        use crate::db::schema::{notes, short_ids};

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

                let new_note = NewNote {
                    id: Uuid::new_v4(),
                    title,
                    status,
                    summary,
                    content,
                    views: 0,
                    comm,
                    user_id,
                    short_id,
                };

                diesel::insert_into(notes::table)
                    .values(&new_note)
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }

    pub async fn find_note_by_uuid(id: &Uuid, conn: &mut Conn) -> Result<Option<Self>, BlogError> {
        use crate::db::schema::notes;

        let note = notes::table
            .filter(notes::id.eq(id))
            .select(Note::as_select())
            .first::<Self>(conn)
            .await
            .optional()?;

        Ok(note)
    }

    pub async fn find_note_by_short_id(
        short_id: &str,
        conn: &mut Conn,
    ) -> Result<Option<Self>, BlogError> {
        use crate::db::schema::{notes, short_ids};

        let note = notes::table
            .inner_join(short_ids::table)
            .filter(
                short_ids::short_name
                    .eq(short_id)
                    .or(short_ids::subname.eq(short_id)),
            )
            .select(Note::as_select())
            .first::<Self>(conn)
            .await
            .optional()?;

        Ok(note)
    }

    pub async fn get_notes_short_total(
        limit: i64,
        offset: i64,
        conn: &mut Conn,
    ) -> Result<(Vec<Note>, Vec<(String, Option<String>)>, u64), BlogError> {
        use crate::db::schema::{notes, short_ids};

        let notes_with_short_name: Vec<(Note, (String, Option<String>))> = notes::table
            .inner_join(short_ids::table)
            .select((
                Note::as_select(),
                (short_ids::short_name, short_ids::subname),
            ))
            .limit(limit)
            .offset(offset)
            .load::<(Note, (String, Option<String>))>(conn)
            .await?;

        let (notes, short_names): (Vec<Note>, Vec<(String, Option<String>)>) =
            notes_with_short_name.into_iter().unzip();

        let total = notes::table.count().get_result::<i64>(conn).await?;

        Ok((notes, short_names, total as u64))
    }

    pub async fn get_notes_by_tag(
        tag_id: Uuid,
        limit: i64,
        offset: i64,
        DbConn(mut conn): DbConn,
    ) -> Result<Vec<Note>, BlogError> {
        use crate::db::schema::{note_tags, notes};

        let notes = notes::table
            .inner_join(note_tags::table)
            .filter(note_tags::tag_id.eq(tag_id))
            .select(Note::as_select())
            .order(notes::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Self>(&mut conn)
            .await?;

        Ok(notes)
    }

    pub async fn get_notes_by_sort(
        sort_id: Uuid,
        limit: i64,
        offset: i64,
        conn: &mut Conn,
    ) -> Result<Vec<Note>, BlogError> {
        use crate::db::schema::{note_sorts, notes};

        let notes = notes::table
            .inner_join(note_sorts::table)
            .filter(note_sorts::sort_id.eq(sort_id))
            .select(Note::as_select())
            .order(notes::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Self>(conn)
            .await?;

        Ok(notes)
    }

    pub async fn update_note_by_uuid(
        id: &Uuid,
        title: &str,
        subname: Option<&str>,
        summary: &str,
        content: &str,
        status: &Status,
        comm: bool,
        pool: DbPool,
    ) -> Result<(), BlogError> {
        use crate::db::schema::{notes, short_ids};

        let mut conn = pool.get_owned().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let now = Utc::now().naive_utc();
                diesel::update(notes::table.find(id))
                    .set((
                        notes::content.eq(content),
                        notes::summary.eq(summary),
                        notes::title.eq(title),
                        notes::status.eq(status),
                        notes::updated_at.eq(now),
                        notes::comm.eq(comm),
                    ))
                    .execute(conn)
                    .await?;

                let short_id: Uuid = notes::table
                    .inner_join(short_ids::table)
                    .filter(notes::id.eq(id))
                    .select(short_ids::id)
                    .first::<Uuid>(conn)
                    .await?;

                diesel::update(short_ids::table.filter(short_ids::id.eq(short_id)))
                    .set((
                        short_ids::subname.eq(subname),
                        short_ids::updated_at.eq(now),
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

    pub async fn delete_note_by_uuid(id: &Uuid, pool: DbPool) -> Result<(), BlogError> {
        use crate::db::schema::{comms, note_sorts, note_tags, notes, short_ids};

        let mut conn = pool.get_owned().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let short_id: Uuid = notes::table
                    .filter(notes::id.eq(id))
                    .select(notes::short_id)
                    .first::<Uuid>(conn)
                    .await?;

                diesel::delete(comms::table.filter(comms::note_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(note_tags::table.filter(note_tags::note_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(note_sorts::table.filter(note_sorts::note_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(notes::table.find(id)).execute(conn).await?;

                diesel::delete(short_ids::table.filter(short_ids::id.eq(short_id)))
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
