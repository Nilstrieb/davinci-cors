use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::handlers::HttpResult;
use crate::models::conversion::IntoDao;
use actix_web::{web, HttpResponse};
use std::str::FromStr;

macro_rules! http_todo {
    () => {
        std::result::Result::Ok(actix_web::HttpResponse::Ok().body("Unimplemented"))
    };
    ($str:literal) => {
        std::result::Result::Ok(actix_web::HttpResponse::Ok().body($str))
    };
}

pub(super) fn class_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/classes", web::post().to(create_class)).service(
        web::scope("/classes/{uuid}")
            .route("", web::get().to(get_class))
            .route("", web::put().to(edit_class))
            .route("", web::delete().to(delete_class))
            .route("/members/{uuid}", web::put().to(edit_member))
            .route("/join", web::post().to(request_join))
            .route("/requests", web::get().to(get_join_requests))
            .route("/requests/{uuid}", web::post().to(accept_member))
            .route("/events", web::get().to(get_events))
            .route("/events", web::post().to(create_event))
            .route("/events/{uuid}", web::get().to(get_event))
            .route("/events/{uuid}", web::put().to(edit_event))
            .route("/events/{uuid}", web::delete().to(delete_event))
            .route("timetable", web::get().to(get_timetable))
            .route("timetable", web::put().to(edit_timetable)),
    );
}

async fn get_class(params: web::Path<String>, db: web::Data<Pool>, claims: Claims) -> HttpResult {
    let uuid = uuid::Uuid::from_str(&params)?;

    let class = web::block(move || actions::class::get_class(&db, uuid))
        .await?
        .ok_or(ServiceErr::NotFound)?
        .into_dao()?;

    if class.members.iter().any(|member| member.user == claims.uid) {
        Ok(HttpResponse::Ok().json(class))
    } else {
        Err(ServiceErr::Unauthorized("Cannot access other class"))
    }
}

async fn create_class() -> HttpResult {
    http_todo!()
}

async fn edit_class() -> HttpResult {
    http_todo!()
}

async fn delete_class() -> HttpResult {
    http_todo!()
}

async fn edit_member() -> HttpResult {
    http_todo!()
}

async fn request_join() -> HttpResult {
    http_todo!()
}

async fn get_join_requests() -> HttpResult {
    http_todo!()
}
async fn accept_member() -> HttpResult {
    http_todo!()
}

async fn get_event() -> HttpResult {
    http_todo!()
}

async fn get_events() -> HttpResult {
    http_todo!()
}

async fn create_event() -> HttpResult {
    http_todo!()
}

async fn edit_event() -> HttpResult {
    http_todo!()
}

async fn delete_event() -> HttpResult {
    http_todo!()
}

async fn get_timetable() -> HttpResult {
    http_todo!()
}

async fn edit_timetable() -> HttpResult {
    http_todo!()
}