#[macro_use] extern crate rocket;

mod database;
use database::connection;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::Selectable;
use diesel::SelectableHelper;
use rocket::{http::ContentType, serde::{json::Json,Deserialize,Serialize}};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins,CorsOptions};

use dotenv;

// Is the number of derivations normal for one struct
#[derive(Queryable,Insertable,Selectable)]
#[derive(Serialize,Deserialize)]
#[serde(crate="rocket::serde")]
#[diesel(table_name = crate::database::schema::user)]
pub struct User{
    pub name:String,
    pub password:String
}

#[get("/")]
fn index() -> (ContentType,&'static str){
    (ContentType::HTML,"<h2>Welcome to login backend</h2><p>you should be on the front end</p>")
}

#[get("/")]
fn get_all_user() -> Json<Vec<User>>{
    let conn = &mut connection::DatabaseConn::establish_connection();
    use crate::database::schema::user::dsl::*;
    let userjsons =  user 
        .select(User::as_select())
        .load(conn);
    rocket::serde::json::Json(userjsons.unwrap())
}

#[get("/<username>")]
fn get_user(username:&str) -> Json<User>{
    let conn = &mut connection::DatabaseConn::establish_connection();
    use crate::database::schema::user::dsl::*;
    let userjson = user
        .find(username.to_owned())
        .first(conn);
    rocket::serde::json::Json(userjson.unwrap())
}

#[post("/",data="<user_input>")]
fn create_user(user_input:Json<User>){
    let conn = &mut connection::DatabaseConn::establish_connection();
    use crate::database::schema::user::dsl::*;
    let _ = insert_into(user)
        .values((name.eq(&user_input.name),password.eq(&user_input.password)))
        .execute(conn);
}

#[launch]
fn rocket() -> _{
    // Enabling .env variables
    dotenv::dotenv().ok();
    // :s Something about allowing user of resources even if the origins are different
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![
                         Method::Get,Method::Post,Method::Patch
        ].into_iter().map(From::from).collect(),).allow_credentials(true);

    // Mount my pretties :)
    rocket::build()
        .mount("/",routes![index])
        .mount("/user",routes![get_all_user])
        .mount("/user",routes![get_user])
        .mount("/user", routes![create_user])
        .attach(cors.to_cors().unwrap())
}

