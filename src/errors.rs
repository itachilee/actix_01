
use actix_web::{ error, HttpResponse,  body::BoxBody,
    http::{
        header::ContentType,
        StatusCode
    } ,

};


use derive_more::{Display};
#[derive(Debug,Display)]
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

impl std::error::Error for MyError {

    fn description(&self) -> &str {
        match self {
            MyError::BadClientData=> "bad client data",
            MyError::InternalError=> "internal error",
            MyError::Timeout=> "timeout",
            MyError::ValidationError{..}=> {
                "validation error on filed "
            }
        }
    }
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