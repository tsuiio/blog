use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::users, Conn},
    error::BlogError,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    id: Uuid,
    username: &'a str,
    nickname: &'a str,
    email: &'a str,
    password_hash: &'a str,
}

impl User {
    pub async fn created_user(
        username: &str,
        nickname: &str,
        email: &str,
        password_hash: &str,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::users;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            username,
            nickname,
            email,
            password_hash,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn find_user_by_name(
        username: &str,
        conn: &mut Conn,
    ) -> Result<Option<Self>, BlogError> {
        use crate::db::schema::users;

        let user = users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?;

        Ok(user)
    }

    pub async fn find_user_by_email(
        email: &str,
        conn: &mut Conn,
    ) -> Result<Option<Self>, BlogError> {
        use crate::db::schema::users;

        let user = users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?;

        Ok(user)
    }

    pub async fn login_by_email_or_name(
        identity: &str,
        conn: &mut Conn,
    ) -> Result<Option<User>, BlogError> {
        use crate::db::schema::users;

        let user = users::table
            .filter(users::username.eq(identity).or(users::email.eq(identity)))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?;

        Ok(user)
    }

    pub async fn update_user_info(
        id: &Uuid,
        nickname: &str,
        conn: &mut Conn,
    ) -> Result<(), BlogError> {
        use crate::db::schema::users;

        diesel::update(users::table.find(id))
            .set(users::nickname.eq(nickname))
            .execute(conn)
            .await?;

        Ok(())
    }
}
