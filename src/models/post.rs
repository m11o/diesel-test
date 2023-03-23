use diesel::prelude::*;
use chrono::{Datetime, Local};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: Datetime<Local>,
    pub updated_at: Datetime<Local>,
}