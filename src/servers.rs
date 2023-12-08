

use actix_web::{get, post, error,web, App, HttpResponse, HttpServer, Responder, body::BoxBody,
    http::{
        header::ContentType,
        StatusCode
    } ,
    guard,
    middleware::Logger, cookie::time::Duration, 
};

use super::errors::MyError;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[post("/vaildtest")]
async fn vaildtest(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[post("/index")]
async fn index(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/{user_id}/{job}")]
async fn get_user_info(info: web::Path<(u32,String)>)->impl Responder{
    let (user_id,job)=info.into_inner();
    format!("user_id:{},job:{}",user_id,job)
}



use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Deserialize, Serialize)]
struct Info {
    user_id: i32,
    job: String,
}

impl Responder for Info{
    type Body = BoxBody;
    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Info{
    Info{
        user_id:info.user_id,
        job:info.job.clone()
    }
}






use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        text::Text,
        MultipartForm,
    },
    Multipart,
};
#[derive(Debug,MultipartForm)]
struct UploadForm{
    #[multipart(rename="file")]
    files:Vec<TempFile>,
    name:Text<String>,
    id:Text<i32>,
    crates:Text<String>
}


#[post("/form")]
async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder,Box::<dyn std::error::Error>> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        f.file.persist(path).unwrap();
    }
    let res = format!("id:{},name:{}", form.id.0, form.name.0);
    let crates = form.crates.0.as_str();
    let crates: Value = serde_json::from_str(crates)?; //将合法json字符串转json
    println!("crates:{:#?}", crates);
    Ok(HttpResponse::Ok().body(res))
}



#[get("/test")]
async fn test()->Result<&'static str,MyError>{
    Err(MyError::BadClientData)
}

#[get("/vailderrortest")]
async fn vailderrortest() ->Result<&'static str,MyError>{
    Err(MyError::ValidationError { filed: "input is invalid".to_string() })
}

// use sqlx::{MySqlPool};
// use super::entity::User;
// use entity::prelude::User;
use sea_orm::{DatabaseConnection, EntityTrait,Set,ActiveModelTrait};
use entity::{
    user::{Entity as User,ActiveModel}
};
#[get("/get_users")]
pub async fn get_users(db: web::Data<DatabaseConnection>) -> Result<HttpResponse,error::Error> {
    let recs: Vec<entity::user::Model> = User::find()
    .all(db.as_ref())
    .await.unwrap();

    // 如果一切正常，将结果转换为 JSON 并返回
    Ok(HttpResponse::Ok().json(recs))
}

#[derive(Serialize,Deserialize)]
struct AddUserDto{
    pub id: i32,
    pub username: String,
}
use chrono::{Local};

#[post("/add_users")]
pub async fn add_users(info:web::Json<AddUserDto>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse,error::Error> {

    let new_user= ActiveModel {
        id: Set(info.id.to_owned()),
        username: Set(Some(info.username.to_owned())),
        ..Default::default()
    };
    let pear = new_user.insert(db.as_ref()).await.unwrap();

    // 如果一切正常，将结果转换为 JSON 并返回
    Ok(HttpResponse::Ok().json(pear))
}