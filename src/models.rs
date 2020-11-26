use crate::schema::owo_faces;
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