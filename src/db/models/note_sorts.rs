use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::note_sorts, Conn},
    error::BlogError,
};

use super::{notes::Note, sorts::Sort};

#[derive(Debug, Queryable, Insertable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Note))]
#[diesel(belongs_to(Sort))]
#[diesel(table_name = note_sorts)]
#[diesel(primary_key(note_id, sort_id))]
pub struct NoteSort {
    pub note_id: Uuid,
    pub sort_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = note_sorts)]
pub struct NewNoteSort<'a> {
    note_id: &'a Uuid,
    sort_id: &'a Uuid,
}

impl NoteSort {
    pub async fn add_sort_to_note(
        note_id_: &Uuid,
        sort_id_: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::note_sorts::dsl::*;

        let now_note_sort = NewNoteSort {
            note_id: note_id_,
            sort_id: sort_id_,
        };

        diesel::insert_into(note_sorts)
            .values(&now_note_sort)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn remove_sort_from_note(
        note_id_: &Uuid,
        sort_id_: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::note_sorts::dsl::*;

        diesel::delete(note_sorts.filter(note_id.eq(note_id_).and(sort_id.eq(sort_id_))))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn get_sorts_by_note_id(
        note_id_: &Uuid,
        conn: &mut Conn,
    ) -> Result<Vec<Sort>, BlogError> {
        use crate::db::schema::{note_sorts, sorts};

        let sorts = note_sorts::table
            .inner_join(sorts::table)
            .filter(note_sorts::note_id.eq(note_id_))
            .select(Sort::as_select())
            .load::<Sort>(conn)
            .await?;

        Ok(sorts)
    }

    pub async fn get_note_sorts_and_tags_by_notes(
        notes: &Vec<Note>,
        conn: &mut Conn,
    ) -> Result<Vec<(NoteSort, Sort)>, BlogError> {
        use crate::db::schema::sorts;

        let note_sorts_tags: Vec<(NoteSort, Sort)> = NoteSort::belonging_to(notes)
            .inner_join(sorts::table)
            .select((NoteSort::as_select(), Sort::as_select()))
            .load::<(NoteSort, Sort)>(conn)
            .await?;

        Ok(note_sorts_tags)
    }
}
