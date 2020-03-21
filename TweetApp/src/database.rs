// persistent data manager module
extern crate mysql;

use std::default::Default;
use mysql::*;
use mysql::prelude;

//public structs
#[derive(Debug)]
pub struct UserData {
	pub name : String,
	pub login: String,
	pub password : String
}

// public methods
pub fn create_database() {
	//TODO: Code here all initialization for mysql
	let url = "mysql://mZ1Vu6T31b:3oWn6b41Df@remotemysql.com:3306/mZ1Vu6T31b";
	let pool = mysql::Pool::new_manual(1,1,url).unwrap();
	
	println!("Begin creating default tables .....\n");
	
	
	pool.prep_exec(r"CREATE TABLE IF NOT EXISTS USER_TABLE (
						user_id int AUTO_INCREMENT PRIMARY KEY,
						user_name text,
						user_login text,
						user_password text
					)",()).unwrap();

	pool.prep_exec(r"CREATE TABLE IF NOT EXISTS TWEET_TABLE (
						user_id int,
						tweet_id int not null,
						tweet_data text,
						FOREIGN KEY (user_id)
							REFERENCES USER_TABLE(user_id)
					)",()).unwrap();
}

pub fn add_user( userInfo : &mut UserData) {
	println!("\nUserName 21 :{}\n", userInfo.name);
}

pub fn close_database() {
	//TODO: Code here the free database
}

/*
fn get_opts() -> Opts {
	let builder = OptsBuilder::new()
}
*/


