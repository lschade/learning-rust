use document_storage::models::user::{User, UserNew, UserUpdate};
use rocket::Route;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::user_repo;

use bcrypt::{verify, DEFAULT_COST, hash};

pub fn routes() -> Vec<Route> {
    routes![get, get_all, update, create, login]
}

#[get("/")]
async fn get_all(db_conn: DbConn) -> Result<Json<Vec<User>>, String> {
    db_conn.run(|c| {
        user_repo::get_all(&c)
                    .map(|x| Json(x))
                    .map_err(|e| format!("{}", e))
    }).await
}

#[get("/<id>")]
async fn get(id: i32, db_conn: DbConn) -> Result<Json<User>, String> {
    db_conn.run(move |c| {
        user_repo::get(id, &c)
                    .map(|x| Json(x))
                    .map_err(|e| format!("{}", e))
    }).await
}

#[put("/<id>", data = "<update>")]
async fn update(db_conn: DbConn, id: i32, update: Json<UserUpdate>) -> Result<Json<User>, String> {
    let mut update = update.0;

    update.password = update.password.map(|pw| hash(pw, DEFAULT_COST).unwrap());

    db_conn.run(move |c| {
        user_repo::update(id, update, &c)
                    .map(|x| Json(x))
                    .map_err(|e| format!("{}", e))
    }).await
}

#[post("/", data = "<new_user>")]
async fn create<'a>(db_conn: DbConn, new_user: Json<UserNew>) -> Result<Json<User>, String> {
    let mut new_user = new_user.0;
    
    let hashed = hash(new_user.password, DEFAULT_COST).unwrap();
    new_user.password = hashed;

    db_conn.run(move |c| {
        user_repo::create(new_user, &c)
                    .map(|x| Json(x))
                    .map_err(|e| format!("{}", e))
    }).await
}

#[post("/login", data="<login_info>")]
async fn login(db_conn: DbConn, login_info: Json<UserNew>, cookies: &CookieJar<'_>) -> Result<Json<User>, String> {
    let res = db_conn.run(|c| {
        let maybe_user = user_repo::get_by_username(login_info.0.username.as_str(), &c);

        match maybe_user {
            Err(err) => Result::Err(format!("{}", err)),
            Ok(user) => {
                let valid = verify(login_info.0.password, user.password.as_str()).unwrap_or(false);
                if valid {
                    Result::Ok(Json(user))
                } else {
                    Result::Err(String::from("err"))
                }
            }
        }
    }).await;


    match res {
        Ok(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Ok(user)
        },
        Err(e) => Err(e) 
    }
}