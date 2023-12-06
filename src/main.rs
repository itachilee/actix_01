


use actix_web::{ App, HttpServer,
    middleware::Logger, 
    web::Data
};

use std::io::Write;
use log::info;
use actix_cors::Cors;
use actix_01::configs;
use chrono::Local;
use sqlx::{MySqlPool,Pool, MySql};
use std::env;
use urlencoding::encode;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {



    dotenv::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    std::env::set_var("RUST_LOG", "debug");// 设置日志级别
    std::env::set_var("RUST_BACKTRACE", "1"); 
    // env_logger::init();
    
    
    init_logger();
    
    
    // let encoded_password = encode("pllh@123");

    // // 设置数据库连接字符串，将密码中的@符号替换为%40
    // let database_url = format!(
    //     "mysql://root:{}@192.168.0.49/test",
    //     encoded_password
    // );
    println!("database_url: {}", database_url);
    let pool = MySqlPool::connect(&database_url)
    .await
    .expect("Failed to connect to MySQL.");

    // let p=Pool::connect(&database_url)
    // .await.unwrap()
 

    HttpServer::new(move || {
        let logger=Logger::default();
        let cors = Cors::permissive();

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
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











