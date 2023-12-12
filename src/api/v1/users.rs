

use actix_web::{get, post, error,web, HttpResponse, 
};
use serde::{Serialize,Deserialize};
use sea_orm::{DatabaseConnection, EntityTrait,Set,ActiveModelTrait};
use entity::{
    user::{Entity as User,ActiveModel}
};
use chrono::{Utc};

use crate::common::AppData;


// use sqlx::{MySqlPool};
// use super::entity::User;
// use entity::prelude::User;

#[get("/get_users")]
pub async fn get_users(app_data: web::Data<AppData>) -> Result<HttpResponse,error::Error> {


    let db= &app_data.db;
    let recs: Vec<entity::user::Model> = User::find()
    .all(db)
    .await.unwrap();

    // 如果一切正常，将结果转换为 JSON 并返回
    Ok(HttpResponse::Ok().json(recs))
}

#[derive(Serialize,Deserialize)]
struct AddUserDto{
    pub id: i32,
    pub username: String,
}

#[post("/add_users")]
pub async fn add_users(info:web::Json<AddUserDto>,app_data: web::Data::<AppData>) -> Result<HttpResponse,error::Error> {


    let db= &app_data.db;

    let mut counter = app_data.counter.lock().unwrap();
    
    *counter +=1;
    if let Some(_user) = User::find_by_id(info.id).one(db).await.unwrap(){

        eprintln!("Duplicate entry '{}' for key 'PRIMARY'", info.id);
        return     Ok(HttpResponse::Ok().finish());
    }else{
        let new_user= ActiveModel {
            id: Set(info.id.to_owned()),
            username: Set(Some(info.username.to_owned())),
           lastmodified: Set(Some(Utc::now().naive_local()))
        };
        let pear = new_user.insert(db).await.unwrap();
        // 如果一切正常，将结果转换为 JSON 并返回
    Ok(HttpResponse::Ok().json(pear))

    }
}

