
use actix_web::{get, post, error,web, App, HttpResponse, HttpServer, Responder, body::BoxBody,
    guard,

};

use super::servers::{demo::*,api::*};
use actix_files::Files;

pub fn config(cfg:&mut web::ServiceConfig){
    config_demo(cfg);
    config_apiv1(cfg);
}


fn config_demo(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::scope("demo") //这里代表路由地址以user开头，例如/user/info。其次这里写”user“或者”/user“都可以actix会自动补上”/“
        .service(index)
        .service(vaildtest)

        
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(HttpResponse::Ok))
                .route(web::put().to(HttpResponse::Ok)),

        )
        .service(echo)

        .service(Files::new("/static","dist/static/").show_files_listing())
        .service(Files::new("/","dist/").index_file("index.html"))
    );

}


fn config_apiv1(cfg:&mut web::ServiceConfig){
    cfg.service( 
        web::scope("api")
        .service(   get_users)
    .service(    add_users));
}