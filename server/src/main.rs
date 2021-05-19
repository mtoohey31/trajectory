#[macro_use]
extern crate diesel;
extern crate base64;
extern crate crypto;
#[macro_use]
extern crate diesel_migrations;
mod models;
mod schema;

use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Result};
use crypto::hmac::Hmac;
use crypto::pbkdf2::pbkdf2;
use crypto::sha2::Sha256;
use crypto::util::fixed_time_eq;
use diesel::prelude::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use getrandom::getrandom;
use models::{User, UserCreds, UserData, UserDataUpdate};
// use rustc_serialize::base64::{FromBase64, ToBase64};
use serde::Serialize;

#[post("/users")]
async fn create_user(
    creds: web::Json<UserCreds>,
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    _req: web::HttpRequest,
) -> Result<HttpResponse> {
    let connection = pool.get().unwrap();
    use crate::schema::users::dsl::{username, users};
    match diesel::dsl::select(diesel::dsl::exists(
        users.filter(username.eq(&creds.username)),
    ))
    .get_result(&connection)
    {
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        Ok(true) => Ok(HttpResponse::Conflict().finish()),
        Ok(false) => {
            let mut mac = Hmac::new(Sha256::new(), &creds.hashed_passwd.as_bytes());

            let mut hash_salt = [0u8; 16];
            match getrandom(&mut hash_salt) {
                Ok(_) => (),
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            }
            let hash_salt_string = base64::encode(hash_salt);

            let mut vault_salt = [0u8; 16];
            match getrandom(&mut vault_salt) {
                Ok(_) => (),
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            }
            let vault_salt_string = base64::encode(vault_salt);

            let mut key_buf = [0u8; 32];
            pbkdf2(&mut mac, &hash_salt, 10001, &mut key_buf);
            let key = base64::encode(key_buf);

            let new_user = User {
                username: creds.username.to_string(),
                double_hashed_passwd: key,
                hash_salt: hash_salt_string,
                data: None,
                vault_salt: vault_salt_string,
            };

            use schema::users;
            match diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&connection)
            {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
    }
}

#[delete("/users")]
async fn delete_user(
    request: web::HttpRequest,
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse> {
    // TODO: Examine what in this section should constitute a BadRequest and what should constitute
    // an InternalServerError.
    // TODO: Refactor with other methods that parse auth headers into a separate function,
    // potentially into an actix guard?
    let mut iter = match request.headers().get("Authorization") {
        Some(val) => match val.to_str() {
            Ok(val) => val.split(' '),
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    match iter.next() {
        Some(val) => {
            if val != "Basic" {
                return Ok(HttpResponse::BadRequest().finish());
            }
        }
        None => return Ok(HttpResponse::BadRequest().finish()),
    }

    let auth_buf = match iter.next() {
        Some(val) => match base64::decode(val) {
            Ok(val) => val,
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    let mut auth_iter = match std::str::from_utf8(&auth_buf) {
        Ok(val) => val.split(':'),
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let creds_username = match auth_iter.next() {
        Some(val) => val.to_string(),
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    let creds_hashed_passwd = match auth_iter.next() {
        Some(val) => val.to_string(),
        None => return Ok(HttpResponse::BadRequest().finish()),
    };

    let creds = UserCreds {
        username: creds_username,
        hashed_passwd: creds_hashed_passwd,
    };

    let connection = pool.get().unwrap();
    use crate::schema::users::dsl::{username, users};
    match diesel::dsl::select(diesel::dsl::exists(
        users.filter(username.eq(&creds.username)),
    ))
    .get_result(&connection)
    {
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Ok(true) => {
            let user = match users
                .filter(username.eq(&creds.username))
                .get_result::<User>(&connection)
            {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut mac = Hmac::new(Sha256::new(), &creds.hashed_passwd.as_bytes());

            let hash_salt = match base64::decode(user.hash_salt) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut new_key = [0u8; 32];
            pbkdf2(&mut mac, &hash_salt, 10001, &mut new_key);

            let old_key = match base64::decode(user.double_hashed_passwd) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            if fixed_time_eq(&new_key, &old_key) {
                match diesel::delete(users.filter(username.eq(&creds.username)))
                    .execute(&connection)
                {
                    Ok(_) => Ok(HttpResponse::Ok().finish()),
                    Err(_) => Ok(HttpResponse::InternalServerError().finish()),
                }
            } else {
                Ok(HttpResponse::Forbidden().finish())
            }
        }
    }
}

// TODO: Add put("/users") method for changing password

#[derive(Serialize)]
struct Data {
    data: String,
}

#[get("/users/data")]
async fn get_data(
    request: web::HttpRequest,
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse> {
    // TODO: Examine what in this section should constitute a BadRequest and what should constitute
    // an InternalServerError.
    let mut iter = match request.headers().get("Authorization") {
        Some(val) => match val.to_str() {
            Ok(val) => val.split(' '),
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    match iter.next() {
        Some(val) => {
            if val != "Basic" {
                return Ok(HttpResponse::BadRequest().finish());
            }
        }
        None => return Ok(HttpResponse::BadRequest().finish()),
    }

    let auth_buf = match iter.next() {
        Some(val) => match base64::decode(val) {
            Ok(val) => val,
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    let mut auth_iter = match std::str::from_utf8(&auth_buf) {
        Ok(val) => val.split(':'),
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let creds_username = match auth_iter.next() {
        Some(val) => val.to_string(),
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    let creds_hashed_passwd = match auth_iter.next() {
        Some(val) => val.to_string(),
        None => return Ok(HttpResponse::BadRequest().finish()),
    };

    let creds = UserCreds {
        username: creds_username,
        hashed_passwd: creds_hashed_passwd,
    };

    let connection = pool.get().unwrap();
    use crate::schema::users::dsl::{username, users};
    match diesel::dsl::select(diesel::dsl::exists(
        users.filter(username.eq(&creds.username)),
    ))
    .get_result(&connection)
    {
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        Ok(false) => Ok(HttpResponse::Forbidden().finish()),
        Ok(true) => {
            let user = match users
                .filter(username.eq(&creds.username))
                .get_result::<User>(&connection)
            {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut mac = Hmac::new(Sha256::new(), &creds.hashed_passwd.as_bytes());

            let hash_salt = match base64::decode(user.hash_salt) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut new_key = [0u8; 32];
            pbkdf2(&mut mac, &hash_salt, 10001, &mut new_key);

            let old_key = match base64::decode(user.double_hashed_passwd) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            if fixed_time_eq(&new_key, &old_key) {
                Ok(HttpResponse::Ok().json(UserData {
                    data: user.data,
                    vault_salt: user.vault_salt,
                }))
            } else {
                Ok(HttpResponse::Forbidden().finish())
            }
        }
    }
}

#[put("/users/data")]
async fn update_data(
    update: web::Json<UserDataUpdate>,
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse> {
    let connection = pool.get().unwrap();
    use crate::schema::users::dsl::{data, username, users};
    match diesel::dsl::select(diesel::dsl::exists(
        users.filter(username.eq(&update.username)),
    ))
    .get_result(&connection)
    {
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Ok(true) => {
            let user = match users
                .filter(username.eq(&update.username))
                .get_result::<User>(&connection)
            {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut mac = Hmac::new(Sha256::new(), &update.hashed_passwd.as_bytes());

            let hash_salt = match base64::decode(user.hash_salt) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            let mut new_key = [0u8; 32];
            pbkdf2(&mut mac, &hash_salt, 10001, &mut new_key);

            let old_key = match base64::decode(user.double_hashed_passwd) {
                Ok(val) => val,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            if fixed_time_eq(&new_key, &old_key) {
                match diesel::update(users.filter(username.eq(&user.username)))
                    .set(data.eq(&update.new_data))
                    .execute(&connection)
                {
                    Ok(_) => Ok(HttpResponse::Ok().finish()),
                    Err(_) => Ok(HttpResponse::Ok().finish()),
                }
            } else {
                Ok(HttpResponse::Forbidden().finish())
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<PgConnection>::new(
        "postgresql://trajectory-server:changeme@db:5432/trajectory-server",
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    println!("Pool created");

    // TODO: Determine if it's safe to run migrations automatically, i.e. if they're idempotent
    embed_migrations!();
    let connection = pool.get().expect("Failed to get pool");
    embedded_migrations::run(&connection).expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_user)
            .service(delete_user)
            .service(get_data)
            .service(update_data)
            .service(hello_world)
    })
    .bind(&"0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn hello_world() -> Result<HttpResponse> {
    #[derive(Serialize)]
    struct Hello {
        hello: String,
    }
    Ok(HttpResponse::Ok().json(Hello {
        hello: "world".to_string(),
    }))
}
