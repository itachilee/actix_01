

use actix_web::{get, post,web, HttpResponse, Responder, body::BoxBody,
    http::
        header::ContentType
};
use actix_multipart::
    form::{
        tempfile::TempFile,
        text::Text,
        MultipartForm,
    }
;
use super::super:: errors::MyError;
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
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
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



