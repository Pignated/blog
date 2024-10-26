
use std::hash::{DefaultHasher, Hash, Hasher};

use actix_web::{post, web, HttpResponse, Responder};
use sqlite::State;

use crate::{models::{EmailUser, PartialUser, Response}, password_handler::{hash_password, salt_password}};

#[post("/auth/signup")]
async fn sign_up(body: web::Json<EmailUser>) -> impl Responder {
    let username = body.username.to_lowercase();
    let password = &body.password;
    let email = body.email.to_lowercase();
    let connection = sqlite::open(".\\database\\web.db").unwrap();
    if !username_available(username.clone()) {
        let already_exists = Response {
            status: "fail".to_string(),
            message: format!("Username {} already exists!!", username)
        };
        return HttpResponse::Conflict().json(already_exists)
    }
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    let mut id:i64 = i64::from_ne_bytes(hasher.finish().to_ne_bytes());
    loop {
        if user_id_available(id) {
            break;
        }
        id+=1;
    }
    let (salted_password, salt) = salt_password(password.to_string());
    let hashed_password;
    let hashing_failed = Response {
        status: "fail".to_string(),
        message: "Hashing failed, try again".to_string()
    };

    match hash_password(salted_password) {
        Ok(hashed) => hashed_password = hashed,
        Err(_) => return HttpResponse::InternalServerError().json(hashing_failed)
    }
    let failed_to_add = Response {
        status: "fail".to_string(),
        message: "failed to add to database".to_string()
    };
    let completed =Response {
        status: "success".to_string(),
        message: format!("User {} created!!", &username)
    };
    let query = format!("insert into users values ({}, \'{}\',\'{}\',\'{}\', \'{}\')",
    id, username, hashed_password, salt, email);
    let _ = match connection.execute(query) {
        Ok(_) => return HttpResponse::Ok().json(completed),
        Err(_) => return HttpResponse::InternalServerError().json(failed_to_add)
    };
}
#[post("auth/signin")]
async fn sign_in(body: web::Json<PartialUser>) -> impl Responder {
    let username = body.username.to_lowercase();
    let password = &body.password;
    let connection = sqlite::open(".\\database\\web.db").unwrap();
    let query1 = "select password_hash, password_salt from users where username=?";
    let mut reply = connection.prepare(query1).unwrap();
    reply.bind((1,username.as_str())).unwrap();
    let mut hash:String = "".into();
    let mut salt:String = "".into();
    while let Ok(State::Row) = reply.next() {
        hash = reply.read::<String, _>("password_hash").unwrap();
        salt = reply.read::<String,_>("password_salt").unwrap();
    }
    let login_success = Response {
        status: "success".to_string(),
        message: format!("User {} signed in!!", username)
    };
    let login_failure = Response {
        status: "fail".to_string(),
        message: "incorrect password".to_string()
    };
    let salted = format!("{}{}",password, salt).to_string();
    if bcrypt::verify(salted, &hash).unwrap() {
        return HttpResponse::Accepted().json(login_success);
    }

    return HttpResponse::Unauthorized().json(login_failure);
}

fn username_available(username:String) -> bool {
    let mut returned_value:u64 = 1;
    let connection = sqlite::open(".\\database\\web.db").unwrap();
    eprintln!("{username}");
    let query = "select exists(select 1 from users where username =?) as row_exists";
    let mut reply = connection.prepare(query).unwrap();
    reply.bind((1,username.as_str())).unwrap();
    while let Ok(State::Row) = reply.next() {
        returned_value = reply.read::<String, _>("row_exists").unwrap().parse().unwrap();
    }
    return returned_value == 0;
}
fn user_id_available(user_id:i64) -> bool {
    let mut returned_value:u64 = 1;
    let connection = sqlite::open(".\\database\\web.db").unwrap();
    let id_string = user_id.to_string();
    let query = "select exists(select 1 from users where user_id =?) as row_exists";
    let mut reply = connection.prepare(query).unwrap();
    reply.bind((1,id_string.as_str())).unwrap();
    while let Ok(State::Row) = reply.next() {
        returned_value = reply.read::<String, _>("row_exists").unwrap().parse().unwrap();
    }
    return returned_value == 0;
}