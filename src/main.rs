


use actix_web::{get, post, error,web, App, HttpResponse, HttpServer, Responder, body::BoxBody,
    http::{
        header::ContentType,
        StatusCode
    } ,
    guard,
    middleware::Logger, 
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::io::Write;
use std::{env, io};
use log::info;

use actix_01::configs;
use chrono::Local;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "error");// 设置日志级别
    std::env::set_var("RUST_BACKTRACE", "1"); //此处暂时不明白，望懂的人解惑
    // env_logger::init();
    init_logger();
    HttpServer::new(|| {
        let logger=Logger::default();
        App::new()
            .wrap(logger)
            .configure(configs::config)
           
         
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



fn init_logger() {
    use env_logger::fmt::Color;
    use env_logger::Env;
    use log::LevelFilter;

    let env = Env::default().filter_or("MY_LOG_LEVEL", "debug");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);

            writeln!(
                buf,
                "{} {} [ {} ] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_style.value(record.level()),
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
    info!("env_logger initialized.");
}











