use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::comms_closure;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = comms_closure)]
pub struct CommsClosure {
    pub ancestor_id: Uuid,
    pub descendant_id: Uuid,
    pub distance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comms_closure)]
pub struct NewCommsClosure<'a> {
    ancestor_id: &'a Uuid,
    descendant_id: &'a Uuid,
    distance: i64,
}
