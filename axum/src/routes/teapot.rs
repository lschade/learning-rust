use axum::http::StatusCode;

pub async fn im_a_teapot() -> Result<(), StatusCode> {
    return Err(StatusCode::IM_A_TEAPOT);
}
