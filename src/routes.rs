extern crate base64;
extern crate rocket_client_addr;

use rocket::Request;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{Json, JsonValue};
use rocket_client_addr::ClientRealAddr;


use diesel::{self, prelude::*, sql_query};
use unicode_segmentation::UnicodeSegmentation;
use base64::{encode, decode};
use rand::prelude::*;

use crate::models::{NewFace, NewAccount};
use crate::schema;
use crate::DbConn;
use crate::schema::{owo_faces, accounts};

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&request).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Deserialize)]
pub struct NewFaceValues {
    pub face: String,
    pub style: String,
    pub emotion: String,
    pub token: String
}

#[derive(Deserialize)]
pub struct NewAccountValues {
    pub ip_addr: String,
    pub token: String
}

#[derive(Queryable, Serialize, Debug, QueryableByName)]
#[table_name = "owo_faces"]
pub struct DBFace {
    id: i32,
    face: String,
    face_size: i32,
    style: String,
    emotion: String
}

#[derive(Queryable, Serialize, Debug, QueryableByName)]
#[table_name = "accounts"]
pub struct DBAccount {
    id: i32,
    ip_addr: String,
    token: String
}

// Database
pub fn create_face(conn: DbConn, face: &str, style: &str, emotion: &str) -> i32 {
    use schema::owo_faces::dsl::owo_faces;

    let face_size_temp: i32 = face.graphemes(true).count() as i32;
    let face_size: &i32 = &face_size_temp;
    let new_face = NewFace {
        face,
        face_size,
        style,
        emotion
    };

    diesel::insert_into(owo_faces)
        .values(&new_face)
        .execute(&conn.0)
        .expect("Error saving new face");

    let inserted_id: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE face = '{}' LIMIT 1", face))
        .load(&conn.0)
        .expect("An error has occurred");

    return inserted_id[0].id
}

pub fn create_account(conn: DbConn, ip_addr: &str, token: &str) {
    use schema::accounts::dsl::accounts;

    let new_account = NewAccount {
        ip_addr,
        token
    };

    diesel::insert_into(accounts)
        .values(&new_account)
        .execute(&conn.0)
        .expect("Error saving new account");
}

pub fn get_faces(conn: DbConn) -> Vec<DBFace> {
    use crate::schema::owo_faces::dsl::*;
    let connection = conn;

    return owo_faces.load::<DBFace>(&connection.0).ok().unwrap();
}

// Routes
#[get("/faces")]
pub fn faces(conn: DbConn) -> ApiResponse {
    let data = get_faces(conn);
    
    ApiResponse {
        json: json!({"success": true,"data": data}),
        status: Status::Ok
    }
}

#[delete("/face")]
pub fn delete_face() -> JsonValue {
    json!({
        "success": true
    })
}

#[get("/face?<id>&<face>&<face_size>&<style>&<emotion>")]
pub fn search_face(conn: DbConn, id: Option<usize>, face: Option<String>, face_size: Option<usize>, style: Option<String>, emotion: Option<String>) -> ApiResponse {
    // direct searches
    // NOTE very repetitive code
    if let Some(id) = id {
        let id_search_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE id = {} LIMIT 1", id))
            .load(&conn.0)
            .expect("An error has occurred");

        return ApiResponse {
            json: json!({ "success": true, "data": id_search_face[0] }),
            status: Status::Ok
        }
    }
    if let Some(face) = face {
        let face_search_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE face = '{}' LIMIT 1", face))
            .load(&conn.0)
            .expect("An error has occurred");

        return ApiResponse {
            json: json!({ "success": true, "data": face_search_face[0] }),
            status: Status::Ok
        }
    }

    // returns list of simple ones (face_size, emotion, style)
    if let Some(face_size) = face_size {
        let face_size_search_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE face_size = {}", face_size))
            .load(&conn.0)
            .expect("An error has occurred");

        return ApiResponse {
            json: json!({ "success": true, "data": face_size_search_face }),
            status: Status::Ok
        }
    }
    if let Some(style) = style {
        let style_search_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE style = '{}'", style))
            .load(&conn.0)
            .expect("An error has occurred");

        return ApiResponse {
            json: json!({ "success": true, "data": style_search_face }),
            status: Status::Ok
        }
    }
    if let Some(emotion) = emotion {
        let emotion_search_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE emotion = '{}'", emotion))
            .load(&conn.0)
            .expect("An error has occurred");

        return ApiResponse {
            json: json!({ "success": true, "data": emotion_search_face }),
            status: Status::Ok
        }
    }
    return ApiResponse {
        json: json!({ "success": false, "reason": "No results came back" }),
        status: Status::Ok
    }
}

#[post("/new", format = "json", data = "<entry>")]
pub fn new(conn: DbConn, entry: Json<NewFaceValues>) -> ApiResponse {
    let input_face = &entry.face;
    let input_style = &entry.style;
    let input_emotion = &entry.emotion;
    let input_token = &entry.token;

    let styles: Vec<String> = vec!["regular".to_owned()];
    let emotions: Vec<String> = vec!["happy".to_owned(), "sad".to_owned(), "stress".to_owned(), "angry".to_owned(), "weird".to_owned()];

    // check if token is valid
    let inputted_token: Vec<DBAccount> = sql_query(format!("SELECt * FROM accounts WHERE token = '{}' LIMIT 1", input_token))
        .load(&conn.0)
        .expect("An error has occurred");

    if inputted_token.is_empty() {
        return ApiResponse {
            json: json!({ "success": false, "reason": format!("Token '{}' doesn't exist", input_token) }),
            status: Status::Unauthorized
        }
    }

    // check to see if face exists
    let inputted_face: Vec<DBFace> = sql_query(format!("SELECT * FROM owo_faces WHERE face = '{}' LIMIT 1", input_face))
        .load(&conn.0)
        .expect("An error has occurred");

    if !inputted_face.is_empty() {
        return ApiResponse {
            json: json!({ "success": false, "reason": format!("Face '{}' already exists at id '{}'", inputted_face[0].face, inputted_face[0].id) }),
            status: Status::Forbidden
        }
    }

    // check to see if user entered valid style & emotion
    if !styles.contains(input_style) {
        return ApiResponse {
            json: json!({ "success": false, "reason": format!("Style '{}' is not a valid style type", input_style) }),
            status: Status::UnprocessableEntity
        }
    }
    if !emotions.contains(input_emotion) {
        return ApiResponse {
            json: json!({ "success": false, "reason": format!("Emotion '{}' is not a valid emotion type", input_emotion) }),
            status: Status::UnprocessableEntity
        }
    }

    let id_of_post = create_face(conn, input_face, input_style, input_emotion);
    return ApiResponse {
        json: json!({ "success": true, "id": id_of_post }),
        status: Status::Ok
    }
}

#[get("/account")]
pub fn account(conn: DbConn, client_addr: &ClientRealAddr) -> ApiResponse {
    let mut token = String::from("");
    let ip_addr = client_addr.get_ipv4_string().unwrap();

    // check to see if face exists
    let sent_ip: Vec<DBAccount> = sql_query(format!("SELECT * FROM accounts WHERE ip_addr = '{}' LIMIT 1", ip_addr))
        .load(&conn.0)
        .expect("An error has occurred");

    if !sent_ip.is_empty() {
        return ApiResponse {
            json: json!({ "success": false, "reason": format!("An account with the ip address '{}' already exists", ip_addr) }),
            status: Status::Forbidden
        }
    }

    // token is made by
    // base64(1000-9999) . base64(random 12 characters)
    let y: f64 = rand::thread_rng().gen();
    let random_number_fh: f64 = y * (9999.0 - 1000.0) + 1000.0;
    let first_half = encode(random_number_fh.floor().to_string());

    let mut char_bytes: Vec<u8> = Vec::new();
    for _n in 0..12 {
        // 97 - 122 (a-z)
        let x: f64 = rand::thread_rng().gen();
        let random_number_sh: f64 = x * (122.0 - 97.0) + 97.0;
        char_bytes.push(random_number_sh.floor() as u8)
    }
    let last_half = encode(String::from_utf8(char_bytes).unwrap());

    token.push_str(first_half.as_str());
    token.push_str(".");
    token.push_str(last_half.as_str());

    create_account(conn, &ip_addr, &token);

    return ApiResponse {
        json: json!({ "success": true, "token": token }),
        status: Status::Ok
    }
}

// Catchers
#[catch(404)]
pub fn not_found(req: &Request) -> JsonValue {
    json!({
        "success": false,
        "reason": format!("Path Not Found: {}", req.uri().path())
    })
}  

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> JsonValue {
    json!({
        "success": false,
        "reason": format!("Unprocessable Entity")
    })
}

#[catch(500)]
pub fn internal_server(_req: &Request) -> JsonValue {
    json!({
        "success": false,
        "reason": "500 Internal Server Error"
    })
}