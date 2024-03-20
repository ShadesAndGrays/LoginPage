use diesel::{mysql::MysqlConnection, Connection};

pub struct DatabaseConn();

impl DatabaseConn{
    pub fn establish_connection() -> MysqlConnection{
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        match MysqlConnection::establish(&database_url){
            Ok(mysql_conn) => {
                println!("Database Connection was successful");
                mysql_conn
            }
            Err(msg) => {
                panic!("Error: {} Failed to create database connection",msg);
            }
        }

    }

}
