pub mod user;
pub mod document;
pub mod collection;

pub mod repo_macros {
    #[macro_export]
    macro_rules! crud_create {
        ($obj_type:ty, $new_type:ty, $module:path) => {
            #[post("/", data = "<new_collection>")]
            pub fn create(db_conn: Database, new_collection: Json<$new_type>) -> Result<Json<$obj_type>, Error> {
                db_insert!(db_conn, base::table, new_collection)
            }
        }
    }
    
    #[macro_export]
    macro_rules! crud_get {
        ($obj_type:ty, $new_type:ty, $module:path) => {
            #[get("/<id>")]
            pub fn get(db_conn: Database, id: i32) -> Result<Json<$obj_type>, Error> {
                db_get!(db_conn, base::table, base::id, id)
            }
        }
    }
    
    #[macro_export]
    macro_rules! crud_get_all {
        ($obj_type:ty, $new_type:ty, $module:path) => {
            #[get("/")]
            pub fn get_all(db_conn: Database) -> Result<Json<Vec<$obj_type>>, Error> {
                db_get_all!(db_conn, base::table)
            }  
        }
    }
    
    #[macro_export]
    macro_rules! crud_delete {
        ($obj_type:ty, $new_type:ty, $module:path) => {
            #[delete("/<id>")]
            pub fn delete(db_conn: Database, id: i32) {
                db_delete!(db_conn, base::table, base::id, id);
            }  
        }
    }
    
    #[macro_export]
    macro_rules! crud_operations {
        ($obj_type:ty, $new_type:ty, $module:path) => 
        {
            use $module as base;
    
            crate::crud_create!($obj_type, $new_type, $module);
            crate::crud_get!($obj_type, $new_type, $module);
            crate::crud_get_all!($obj_type, $new_type, $module);
            crate::crud_delete!($obj_type, $new_type, $module);
        }
    }
}
