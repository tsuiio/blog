pub mod about;

use about::AboutPage;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::notes::Status;
use crate::{
    db::{schema::sql_types::PageType, DbPool},
    error::BlogError,
};

pub trait Page {
    async fn create_page(
        &self,
        status: &Status,
        subname: &str,
        comm: bool,
        user_id: &Uuid,
        pool: DbPool,
    ) -> Result<(), BlogError>;

    async fn update_page(
        &self,
        id: &Uuid,
        status: &Status,
        subname: &str,
        comm: bool,
        pool: DbPool,
    ) -> Result<(), BlogError>;

    async fn find_page_by_short_id(
        short_id: &str,
        pool: DbPool,
    ) -> Result<Option<(Uuid, NaiveDateTime, NaiveDateTime, PageTi)>, BlogError> {
        use crate::db::schema::{page_about, pages, short_ids};

        let mut conn = pool.get().await?;
        let page = pages::table
            .inner_join(short_ids::table)
            .filter(
                short_ids::short_name
                    .eq(short_id)
                    .or(short_ids::subname.eq(short_id)),
            )
            .select((
                pages::id,
                pages::created_at,
                pages::updated_at,
                pages::page_type,
            ))
            .first::<(Uuid, NaiveDateTime, NaiveDateTime, PageTy)>(&mut conn)
            .await
            .optional()?;

        if let Some((page_id, created_at, updated_at, pty)) = page {
            match pty {
                PageTy::About => {
                    let about_page = page_about::table
                        .filter(page_about::page_id.eq(page_id))
                        .select(AboutPage::as_select())
                        .first::<AboutPage>(&mut conn)
                        .await
                        .optional()?;
                    if let Some(about_page) = about_page {
                        Ok(Some((
                            page_id,
                            created_at,
                            updated_at,
                            PageTi::About(about_page),
                        )))
                    } else {
                        Ok(None)
                    }
                }
            }
        } else {
            Ok(None)
        }
    }

    async fn delete_page(id: &Uuid, pool: DbPool) -> Result<(), BlogError> {
        use crate::db::schema::{comms, page_about, page_sorts, pages, short_ids};
        let mut conn = pool.get_owned().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let (short_id_id, pty) = pages::table
                    .inner_join(short_ids::table)
                    .filter(pages::id.eq(id))
                    .select((short_ids::id, pages::page_type))
                    .first::<(Uuid, PageTy)>(conn)
                    .await?;

                diesel::delete(comms::table.filter(comms::note_id.eq(id)))
                    .execute(conn)
                    .await?;

                diesel::delete(page_sorts::table.filter(page_sorts::page_id.eq(id)))
                    .execute(conn)
                    .await?;

                match pty {
                    PageTy::About => {
                        diesel::delete(page_about::table.filter(page_about::page_id.eq(id)))
                            .execute(conn)
                            .await?;
                    }
                }

                diesel::delete(pages::table.find(id)).execute(conn).await?;

                diesel::delete(short_ids::table.filter(short_ids::id.eq(short_id_id)))
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

#[derive(Debug, PartialEq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "PageType"]
pub enum PageTy {
    About,
}

pub enum PageTi {
    About(AboutPage),
}

pub struct PageImpl;
impl Page for PageImpl {
    async fn create_page(
        &self,
        _status: &Status,
        _subname: &str,
        _comm: bool,
        _user_id: &Uuid,
        _pool: DbPool,
    ) -> Result<(), BlogError> {
        Ok(())
    }

    async fn update_page(
        &self,
        _id: &Uuid,
        _status: &Status,
        _subname: &str,
        _comm: bool,
        _pool: DbPool,
    ) -> Result<(), BlogError> {
        Ok(())
    }
}
