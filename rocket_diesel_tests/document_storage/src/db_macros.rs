#[macro_export]
macro_rules! db_insert {
    ($conn:expr, $table:path, $aggregate:expr) => {
        diesel::insert_into($table)
            .values(&$aggregate.0)
            .get_result(&*$conn)
            .map(|x| Json(x))
    };
}

#[macro_export]
macro_rules! db_get_all {
    ($conn:expr, $table:expr) => {
        $table.get_results(&*$conn)
            .map(|x| Json(x))
    };
}

#[macro_export]
macro_rules! db_get_all_u {
    ($conn:expr, $module:path, $user_id:expr) => {
        schema_module::table.filter(schema_module::owner.eq($user_id))
            .get_results(&*$conn)
            .map(|x| Json(x))
    };
}

#[macro_export]
macro_rules! db_get {
    ($conn:expr, $table:expr, $filter_by:expr, $id:expr) => {
        $table.filter($filter_by.eq($id))
            .get_result(&*$conn)
            .map(|x| Json(x))
    };
}

#[macro_export]
macro_rules! db_delete {
    ($conn:expr, $table:expr, $filter_by:expr, $id:expr) => {
        diesel::delete($table.filter($filter_by.eq($id)))
            .execute(&*$conn)
            .expect("err");
    };
}
