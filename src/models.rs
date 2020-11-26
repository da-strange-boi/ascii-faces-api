use crate::schema::{owo_faces, accounts};
#[derive(Queryable)]
pub struct Face {
    pub id: usize,
    pub face: String,
    pub face_size: usize,
    pub style: String,
    pub emotion: String
}

#[derive(Insertable)]
#[table_name="owo_faces"]
pub struct NewFace<'a> {
    pub face: &'a str,
    pub face_size: &'a i32,
    pub style: &'a str,
    pub emotion: &'a str
}

#[derive(Queryable)]
pub struct Account {
    pub id: usize,
    pub ip_addr: String,
    pub token: String
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub ip_addr: &'a str,
    pub token: &'a str
}