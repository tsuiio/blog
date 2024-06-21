use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::comms, Conn},
    error::BlogError,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = comms)]
#[diesel(belongs_to(Note))]
pub struct Comm {
    pub id: Uuid,
    pub content: String,
    pub comm_user_id: Option<Uuid>,
    pub blog_user_id: Option<Uuid>,
    pub note_id: Option<Uuid>,
    pub page_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comms)]
pub struct NewComm<'a> {
    id: Uuid,
    content: &'a str,
    comm_user_id: &'a Uuid,
    note_id: Option<&'a Uuid>,
    page_id: Option<&'a Uuid>,
}

#[derive(PartialEq)]
pub enum CommType {
    Note,
    Page,
}

impl Comm {
    pub async fn create_comm(
        content_: &str,
        comm_user_id_: &Uuid,
        type_id_: Option<&Uuid>,
        comm_type_: CommType,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::comms::dsl::*;

        let id_ = Uuid::new_v4();
        let new_comm = NewComm {
            id: id_,
            content: content_,
            comm_user_id: comm_user_id_,
            note_id: if comm_type_ == CommType::Note {
                type_id_
            } else {
                None
            },
            page_id: if comm_type_ == CommType::Page {
                type_id_
            } else {
                None
            },
        };

        diesel::insert_into(comms)
            .values(&new_comm)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_comm_by_uuid(id: &Uuid, conn: &mut Conn) -> Result<(), BlogError> {
        use crate::db::schema::comms;

        diesel::delete(comms::table.find(id)).execute(conn).await?;

        Ok(())
    }

    pub async fn get_comms_by_short_id(
        short_name: &str,
        conn: &mut Conn,
    ) -> Result<Vec<Self>, BlogError> {
        use crate::db::schema::{comms, notes, pages, short_ids};

        let results = comms::table
            .left_join(notes::table)
            .left_join(pages::table)
            .inner_join(
                short_ids::table.on(notes::short_id
                    .eq(short_ids::id)
                    .or(pages::short_id.eq(short_ids::id))),
            )
            .filter(short_ids::short_name.eq(short_name))
            .select(Self::as_select())
            .load::<Self>(conn)
            .await?;

        Ok(results)
    }

    pub async fn delete_comms_by_type_id(
        type_id: &Uuid,
        comm_type: CommType,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::comms;

        match comm_type {
            CommType::Note => {
                diesel::delete(comms::table.filter(comms::note_id.eq(type_id)))
                    .execute(conn)
                    .await?;
            }
            CommType::Page => {
                diesel::delete(comms::table.filter(comms::page_id.eq(type_id)))
                    .execute(conn)
                    .await?;
            }
        }

        Ok(())
    }
}
