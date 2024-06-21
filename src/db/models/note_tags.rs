use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::note_tags, Conn},
    error::BlogError,
};

use super::{notes::Note, tags::Tag};

#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Note))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = note_tags)]
#[diesel(primary_key(note_id, tag_id))]
pub struct NoteTag {
    pub note_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = note_tags)]
pub struct NewNoteTag<'a> {
    note_id: &'a Uuid,
    tag_id: &'a Uuid,
}

impl NoteTag {
    pub async fn add_tag_to_note(
        note_id: &Uuid,
        tag_id: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::note_tags;

        let new_note_tag = NewNoteTag { note_id, tag_id };

        diesel::insert_into(note_tags::table)
            .values(&new_note_tag)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn remove_tag_from_note(
        note_id: &Uuid,
        tag_id: &Uuid,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::note_tags;

        diesel::delete(
            note_tags::table.filter(
                note_tags::note_id
                    .eq(note_id)
                    .and(note_tags::tag_id.eq(tag_id)),
            ),
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get_tags_by_note_id(
        note_id: &Uuid,
        conn: &mut Conn,
    ) -> Result<Vec<Tag>, BlogError> {
        use crate::db::schema::{note_tags, tags};

        let tags = note_tags::table
            .inner_join(tags::table)
            .filter(note_tags::note_id.eq(note_id))
            .select(Tag::as_select())
            .load::<Tag>(conn)
            .await?;

        Ok(tags)
    }

    pub async fn get_note_tags_and_tags_by_notes(
        notes: &Vec<Note>,
        conn: &mut Conn,
    ) -> Result<Vec<(NoteTag, Tag)>, BlogError> {
        use crate::db::schema::tags;

        let note_tags_tags: Vec<(NoteTag, Tag)> = NoteTag::belonging_to(notes)
            .inner_join(tags::table)
            .select((NoteTag::as_select(), Tag::as_select()))
            .load::<(NoteTag, Tag)>(conn)
            .await?;

        Ok(note_tags_tags)
    }
}
