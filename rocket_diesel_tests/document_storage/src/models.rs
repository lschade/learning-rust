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
    pub struct UserNew {
        pub username: String,
        pub password: String,
    }
    
    #[derive(AsChangeset, Deserialize)]
    #[table_name="app_user"]
    pub struct UserUpdate {
        pub username: Option<String>,
        pub password: Option<String>,
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
use rocket::request::{ Request, FromRequest, Outcome };
use rocket::http::{ Status };
use crate::diesel::{ QueryDsl, ExpressionMethods, RunQueryDsl };
use crate::database::DbConn;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for user::User {
    type Error = RequestError;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_id: Option<Result<i32, rocket::form::Errors>> = request.query_value("user");
                
        match user_id {
            None => Outcome::Failure((Status::BadRequest, RequestError::UserArgumentMissing)),
            Some(x) => {
                match x {
                    Err(_) => Outcome::Failure((Status::BadRequest, RequestError::BadRequest)),
                    Ok(id) => {
                        let db = request.guard::<DbConn>().await.unwrap();

                        db.run(move |c| {
                            let query_result = app_user::table.filter(app_user::id.eq(id)).get_result(c);

                            match query_result {
                                Ok(user) => Outcome::Success(user),
                                Err(_) => Outcome::Failure((Status::NotFound, RequestError::NotFound))
                            }
                        }).await
                    }
                }
            }
        }
    }
}
