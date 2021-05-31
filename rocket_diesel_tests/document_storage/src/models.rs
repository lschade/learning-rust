pub mod user {
    use serde::{ Serialize, Deserialize };
    use crate::schema::{ app_user };

    #[derive(Queryable, Serialize, Debug)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password: String
    }
    
    #[derive(Insertable, Deserialize)]
    #[table_name="app_user"]
    pub struct UserNew<'a> {
        pub username: &'a str,
        pub password: &'a str,
    }
    
    #[derive(AsChangeset, Deserialize)]
    #[table_name="app_user"]
    pub struct UserUpdate<'a> {
        pub username: Option<&'a str>,
        pub password: Option<&'a str>,
    }
}

pub mod collection {
    use serde::{ Serialize, Deserialize };
    use crate::schema::{ collection };

    #[derive(Queryable, Serialize)]
    pub struct Collection {
        id: i32,
        owner: i32,
        name: String,
    }

    #[derive(Insertable, Deserialize)]
    #[table_name="collection"]
    pub struct CollectionNew {
        pub owner: i32,
        pub name: String,
    }

    #[derive(AsChangeset, Deserialize)]
    #[table_name="collection"]
    pub struct CollectionUpdate {
        pub name: Option<String>,
    }
}

pub mod document {
    use serde::{ Serialize, Deserialize };
    use crate::schema::{ document };

    #[derive(Queryable, Serialize)]
    pub struct Document {
        id: i32,
        owner: i32,
        name: String,
        filename: String,
        collection: Option<i32>,
        document_type: Option<String>,
    }
    
    #[derive(Insertable, Deserialize)]
    #[table_name="document"]
    pub struct DocumentNew {
        pub owner: i32,
        pub name: String,
        pub filename: String,
        pub collection: Option<i32>,
        pub document_type: Option<String>,
    }
    
    #[derive(AsChangeset, Deserialize)]
    #[table_name="document"]
    pub struct DocumentUpdate {
        pub name: Option<String>,
        pub filename: Option<String>,
        pub collection: Option<Option<i32>>,
        pub document_type: Option<Option<String>>,
    }
}


#[derive(Debug)]
pub enum RequestError {
    DbError,
    BadRequest,
    UserArgumentMissing,
    NotFound,
}

use crate::schema::{ app_user };
use rocket::request::{self, Request, FromRequest};
use rocket::http::{ Status, RawStr };
use rocket::State;
use crate::diesel::{ QueryDsl, ExpressionMethods, RunQueryDsl };
use crate::database::PgPool;

impl<'a, 'r> FromRequest<'a, 'r> for user::User {
    type Error = RequestError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pg_pool = request.guard::<State<PgPool>>().unwrap();
        let db_conn = pg_pool.get().unwrap();

        let user_id: Option<Result<i32, &RawStr>> = request.get_query_value("user");
        
        match user_id {
            None => rocket::Outcome::Failure((Status::BadRequest, RequestError::UserArgumentMissing)),
            Some(x) => {
                match x {
                    Err(_) => rocket::Outcome::Failure((Status::BadRequest, RequestError::BadRequest)),
                    Ok(id) => {
                        let query_result = app_user::table.filter(app_user::id.eq(id)).get_result(&db_conn);
                        let query_result = query_result.map(|res| rocket::Outcome::Success(res));

                        query_result.unwrap_or(rocket::Outcome::Failure((Status::BadRequest, RequestError::NotFound)))
                    }   
                }
            }
        }


    }
}
