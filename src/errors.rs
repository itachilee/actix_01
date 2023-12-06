
use actix_web::{get, post, error,web, App, HttpResponse, HttpServer, Responder, body::BoxBody,
    http::{
        header::ContentType,
        StatusCode
    } ,
    guard,
    middleware::Logger, 
};


use derive_more::{Display,Error};
#[derive(Debug,Display,Error)]
pub enum MyError {
    #[display(fmt="internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
    #[display(fmt="vaildation error on filed:{}",filed)]
    ValidationError{filed:String}
}


impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match  *self {
            MyError::InternalError=>StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData=>StatusCode::BAD_REQUEST,
            MyError::Timeout=>StatusCode::GATEWAY_TIMEOUT,
            MyError::ValidationError { ..}=>StatusCode::BAD_REQUEST
        }
    }
}