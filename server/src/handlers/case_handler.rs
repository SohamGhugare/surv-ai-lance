use rocket::{http::Status, response::status::Custom, serde::json::Json};

use crate::models::case::{Case, CreateCase};

#[post("/new", format = "json", data = "<case>")]
pub async fn new_case_handler(case: Json<CreateCase>) -> Custom<Json<Case>> {
    let case = Case::new(case.into_inner());
    Custom(Status::Ok, Json(case))
}
