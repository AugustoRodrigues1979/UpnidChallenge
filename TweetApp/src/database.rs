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

#[derive(Debug)]
pub struct TweetUserData {
	pub tweet : String,
	pub login: String,
	pub password : String
}

#[derive(Debug, PartialEq, Eq)]
pub struct TweetDataRecord {
	pub user_id : i32,
	pub tweet_id: i32,
	pub tweet_data: String
}

// public methods
pub fn create_database() {
	//TODO: Code here all initialization for mysql
	println!("Connecting remote database. Please Wait.....\n");

	let pool = create_pool();
	
	println!("Begin creating default tables (if it's necessary).....\n");
	
	
	pool.prep_exec(r"CREATE TABLE IF NOT EXISTS USER_TABLE (
						user_id int PRIMARY KEY AUTO_INCREMENT,
						user_name text,
						user_login text,
						user_password text
					)",()).unwrap();

	pool.prep_exec(r"CREATE TABLE IF NOT EXISTS TWEET_TABLE (
						user_id int,
						tweet_id int PRIMARY KEY AUTO_INCREMENT,
						tweet_data text,
						FOREIGN KEY (user_id)
							REFERENCES USER_TABLE(user_id)
					)",()).unwrap();
}

pub fn add_tweet_user( tweetInfo : &mut TweetUserData) {

    let mut user_id : &i32 = &0;
	let status : bool;

	let (status, user_id) = search_user_id (&tweetInfo.login, 
											&tweetInfo.password);
	if status 
	{
		let pool = create_pool();
		pool.prep_exec("INSERT INTO TWEET_TABLE (user_id,
												tweet_data)
								VALUES          (:a, :b)",
						params!("a" => &user_id, 
								"b" => &tweetInfo.tweet)).unwrap();
	}
	else
	{
		println!("\nInvalid User data for create user's tweet!\n");
	}
}

pub fn add_user( userInfo : &mut UserData) {
	println!("\nadd_user UserName:{}\n", userInfo.name);
	println!("\nadd_user UserLogin:{}\n", userInfo.login);
	println!("\nadd_user UserPassword:{}\n", userInfo.password);

	if search_user_login( &userInfo.login) 
	{
		let mut warning_login : String = "Login (".to_owned();
		warning_login.push_str(&userInfo.login);
		warning_login.push_str(") already exists!");
		println!("\n{}", warning_login);
	}
	else
	{
		let pool = create_pool();

		pool.prep_exec("INSERT INTO USER_TABLE (user_name,
												user_login,
												user_password) 
							   VALUES          (:a, :b, :c)",
						params!("a" => &userInfo.name, 
							    "b" => &userInfo.login,
							    "c" => &userInfo.password)).unwrap();
	}
}

pub fn close_database() {
	//TODO: Code here the free database
}

/*
fn get_opts() -> Opts {
	let builder = OptsBuilder::new()
}
*/

fn get_source_database() -> String
{
	return "mysql://mZ1Vu6T31b:3oWn6b41Df@remotemysql.com:3306/mZ1Vu6T31b".to_string();
}

fn create_pool() -> mysql::Pool
{
	let url = get_source_database();
	let pool = mysql::Pool::new_manual(1,1,url).unwrap();

	return pool;
}

fn get_one_tweet( user_id : i32, tweet_id : i32 ) -> (bool, Vec<TweetDataRecord>)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id, tweet_id, tweet_data
	                                    FROM TWEET_TABLE
									    WHERE user_id = '".to_owned();
	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("' AND tweet_id = '");
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("'");

	println!("Statement query by user id with tweet_id:{}", stmt_str);

	let mut list_tweets = vec![];

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return (false, list_tweets);
        }
    };

    for row in stmt.execute(()).unwrap() {
        let (user_info,tweet_info, tweet_body) = mysql::from_row::<(i32,i32,String)>(row.unwrap());

		list_tweets.push(TweetDataRecord{user_id    : user_info ,
										 tweet_id   : tweet_info,
										 tweet_data : tweet_body});
    }

	(list_tweets.is_empty(), list_tweets)
}

fn get_tweets_by_user_id( user_id : i32 ) -> (bool, Vec<TweetDataRecord>)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id, tweet_id, tweet_data
	                                    FROM TWEET_TABLE
									    WHERE user_id = '".to_owned();
	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("'");

	println!("Statement query user id:{}", stmt_str);

	let mut list_tweets = vec![];

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return (false, list_tweets);
        }
    };

    for row in stmt.execute(()).unwrap() {
        let (user_info,tweet_info, tweet_body) = mysql::from_row::<(i32,i32,String)>(row.unwrap());

		list_tweets.push(TweetDataRecord{user_id    : user_info ,
										 tweet_id   : tweet_info,
										 tweet_data : tweet_body});
    }

	(true, list_tweets)
}

fn search_user_id( login : &String, password : &String ) -> (bool, i32)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id 
	                                    FROM USER_TABLE
									    WHERE user_login = '".to_owned();
	stmt_str.push_str(login);
	stmt_str.push_str("' AND user_password = '");
	stmt_str.push_str(password);
	stmt_str.push_str("'");

	println!("Statement query user id:{}", stmt_str);

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return (false, 0);
        }
    };

    for row in stmt.execute(()).unwrap() {
        let (user,) = mysql::from_row::<(i32,)>(row.unwrap());
		return (true,user);
    }

	return (false, 0);
}

fn search_user_login( login : &String ) -> bool
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_login 
	                                    FROM USER_TABLE
									    WHERE user_login = '".to_owned();
	stmt_str.push_str(login);
	stmt_str.push_str("'");

	println!("Statement query user login:{}", stmt_str);

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return false;
        }
    };

    for row in stmt.execute(()).unwrap() {
        let (user,) =
            mysql::from_row::<(String,)>(row.unwrap());
        println!("rowContents: {}\n", user);
		return true;
    }
	
	return false;
}

pub fn get_all_tweets_by_user( tweetInfo : &mut TweetUserData) -> (bool, Vec<TweetDataRecord>)
{
    let mut user_id : &i32 = &0;
	let mut list_tweets_bk = vec![];

	let (mut status_bk, user_id) = search_user_id (&tweetInfo.login, 
	                                           &tweetInfo.password);
	
	println!("\nstatus_bk 1:{}\n", status_bk);  
	
	if status_bk
	{
		let (mut status,  mut list_tweets ) = get_tweets_by_user_id( user_id );
		list_tweets_bk.append(&mut list_tweets);
		status_bk = status;
		println!("\nstatus 1:{}\n", status);
		println!("\nstatus_bk 2:{}\n", status_bk);
	}
	
	println!("\nstatus_bk 3:{}\n", status_bk);

	return (status_bk, list_tweets_bk);
}

pub fn get_tweet_by_id( tweetInfo : &mut TweetUserData, tweet_id: i32) -> (bool, Vec<TweetDataRecord>)
{
    let mut user_id : &i32 = &0;
	let mut list_tweets_bk = vec![];

	let (mut status_bk, user_id) = search_user_id (&tweetInfo.login, 
											   &tweetInfo.password);
	if status_bk
	{
		let (mut status,  mut list_tweets ) = get_one_tweet( user_id, tweet_id  );
		list_tweets_bk.append(&mut list_tweets);
		status_bk = status;
	}
	
	return (status_bk, list_tweets_bk);
}
