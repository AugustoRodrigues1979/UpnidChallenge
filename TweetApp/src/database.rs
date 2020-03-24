// persistent data manager module
extern crate mysql;

use mysql::*;

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
	println!("\nConnecting remote database. Please Wait.....\n");

	let pool = create_pool();
	
	println!("\nBegin creating default tables (if it's necessary).....\n");
	
	
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

	pool.prep_exec(r"CREATE TABLE IF NOT EXISTS LIKE_TABLE (
						user_id int,
						tweet_id int,
						FOREIGN KEY (user_id)
							REFERENCES USER_TABLE(user_id),
						FOREIGN KEY (tweet_id)
							REFERENCES TWEET_TABLE(tweet_id)
					)",()).unwrap();
}

pub fn add_tweet_user( tweet_info : &mut TweetUserData) {
	let (status, user_id) = search_user_id (&tweet_info.login, 
											&tweet_info.password);
	if status 
	{
		let pool = create_pool();
		pool.prep_exec("INSERT INTO TWEET_TABLE (user_id,
												tweet_data)
								VALUES          (:a, :b)",
						params!("a" => &user_id, 
								"b" => &tweet_info.tweet)).unwrap();
	}
	else
	{
		println!("\nInvalid User data for create user's tweet!\n");
	}
}

pub fn add_user( user_info : &mut UserData) {

	if search_user_login( &user_info.login) 
	{
		let mut warning_login : String = "Login (".to_owned();
		warning_login.push_str(&user_info.login);
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
						params!("a" => &user_info.name, 
							    "b" => &user_info.login,
							    "c" => &user_info.password)).unwrap();
	}
}

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

fn get_one_tweet_user( user_id : i32, tweet_id : i32 ) -> (bool, Vec<TweetDataRecord>)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id, tweet_id, tweet_data
	                                    FROM TWEET_TABLE
									    WHERE user_id = '".to_owned();
	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("' AND tweet_id = '");
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("'");

	//println!("Statement query by user id with tweet_id:{}", stmt_str);

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

	(list_tweets.is_empty() == false, list_tweets)
}

pub fn get_any_tweet( tweet_id : i32 ) -> (bool, Vec<TweetDataRecord>)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id, tweet_id, tweet_data
	                                    FROM TWEET_TABLE
									    WHERE tweet_id = '".to_owned();
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("'");

	//println!("Statement query any tweet:{}", stmt_str);

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

	(list_tweets.is_empty() == false, list_tweets)
}

pub fn get_user_id(login : String, password : String) -> (bool, i32)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id
	                                    FROM USER_TABLE
									    WHERE user_login = '".to_owned();
	stmt_str.push_str(&login);
	stmt_str.push_str("' AND user_password = '");
	stmt_str.push_str(&password);
	stmt_str.push_str("'");

	//println!("Statement get user id:{}", stmt_str);

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return (false, 0);
        }
    };

	let mut user_id : i32 = 0;

    for row in stmt.execute(()).unwrap() {
        let user_info : i32 = mysql::from_row::<i32>(row.unwrap());
		user_id =   user_info;
	}

	(true, user_id)
}

fn get_tweets_by_user_id( user_id : i32 ) -> (bool, Vec<TweetDataRecord>)
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT user_id, tweet_id, tweet_data
	                                    FROM TWEET_TABLE
									    WHERE user_id = '".to_owned();
	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("'");

	//println!("Statement query user id:{}", stmt_str);

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

	//println!("Statement query user id:{}", stmt_str);

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

	//println!("Statement query user login:{}", stmt_str);

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return false;
        }
    };

    for _row in stmt.execute(()).unwrap() {
		return true;
    }
	
	return false;
}

pub fn get_all_tweets_by_user( tweet_info : &mut TweetUserData) -> (bool, Vec<TweetDataRecord>)
{
	let mut list_tweets_bk = vec![];

	let (mut status_bk, user_id) = search_user_id (&tweet_info.login, &tweet_info.password);

	if status_bk
	{
		let (status,  mut list_tweets ) = get_tweets_by_user_id( user_id );
		list_tweets_bk.append(&mut list_tweets);
		status_bk = status;
	}
	return (status_bk, list_tweets_bk);
}

pub fn get_tweet_by_id( tweet_info : &mut TweetUserData, tweet_id: i32) -> (bool, Vec<TweetDataRecord>)
{
	let mut list_tweets_bk = vec![];

	let (mut status_bk, user_id) = search_user_id (&tweet_info.login, 
											       &tweet_info.password);
	if status_bk
	{
		let (status,  mut list_tweets ) = get_one_tweet_user( user_id, tweet_id  );
		list_tweets_bk.append(&mut list_tweets);
		status_bk = status;
	}
	
	return (status_bk, list_tweets_bk);
}

fn search_like_tweet(user_id : i32, tweet_id : i32) -> bool
{
	let pool = create_pool();
	let mut stmt_str : String = "SELECT COUNT(tweet_id) 
	                                    FROM LIKE_TABLE
									    WHERE user_id = '".to_owned();

	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("' AND tweet_id = '");
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("'");

	//println!("Statement query like tweet:{}", stmt_str);

	let mut stmt = match pool.prepare(stmt_str)
	{
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("{}", e);
            return false;
        }
    };

    for row in stmt.execute(()).unwrap() {
       let (amount_tweets,) =
            mysql::from_row::<(i32,)>(row.unwrap());

		return amount_tweets != 0;
 	}
	
	return false;
}

pub fn remove_like_tweet(user_id : i32, tweet_id : i32) -> bool
{
	if search_like_tweet(user_id, tweet_id) == false
	{
		return false;
	}

	let pool = create_pool();
	let mut stmt_str : String = "DELETE FROM LIKE_TABLE
									    WHERE user_id = '".to_owned();
	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("' AND tweet_id = '");
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("'");

	//println!("Statement remove like tweet:{}", stmt_str);

	pool.prep_exec(stmt_str, ()).unwrap();
	
	return true
}

pub fn insert_like_tweet(user_id : i32, tweet_id : i32) -> bool
{
	if search_like_tweet(user_id, tweet_id) == true
	{
		return false;
	}

	let pool = create_pool();

	let mut stmt_str : String = "INSERT INTO LIKE_TABLE (user_id, tweet_id)
									             VALUES ('".to_owned();

	stmt_str.push_str(&user_id.to_string());
	stmt_str.push_str("','");
	stmt_str.push_str(&tweet_id.to_string());
	stmt_str.push_str("')");

	//println!("Statement insert like tweet:{}", stmt_str);

	pool.prep_exec(stmt_str, ()).unwrap();
	
	return true;
}
