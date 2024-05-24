use crate::error::BlogError;

mod smtp;
mod telegram;
mod webhook;

pub enum Notification {}

impl Notification {
    pub async fn new() -> Result<(), BlogError> {
        todo!()
    }

    pub async fn push(&self) -> Result<(), BlogError> {
        todo!()
    }
}
