//module responsible for manager all data the tweet user
extern crate clap;

use crate::routines::len_vec_str;
use crate::routines::vec_str_to_string;
use crate::database::create_database;
use crate::database::add_tweet_user;

use crate::database::TweetUserData;

use std::string::ToString;

pub enum CodeManagerTweet {
    CodeOk,
    InvalidTweetData,
    InvalidLogin,
    InvalidPassword
}

pub fn create_tweet_user(cmd : &clap::ArgMatches) -> CodeManagerTweet
{    
    let mut result : self::CodeManagerTweet = CodeManagerTweet::CodeOk;
    let ref tweetdata: Vec<&str> = cmd.values_of("TweetText").unwrap().collect()  ;

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    if len_vec_str(tweetdata.to_vec()) == 0
    {
        println!("\nInvalid Tweet's Contents\n");
        result = CodeManagerTweet::InvalidTweetData;
    }
    else if len_vec_str(login.to_vec()) == 0 
    {
        println!("\nInvalid UserLogin\n");
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        println!("\nInvalid UserPassword\n");
        result = CodeManagerTweet::InvalidPassword;
    }

    if let CodeManagerTweet::CodeOk = result {
        //Code for include tweet user data in file here..
        let mut tweetInfo = TweetUserData {tweet:vec_str_to_string(tweetdata.to_vec()), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };

        create_database();
        add_tweet_user(&mut tweetInfo);
    }
    else 
    {
        println!("\nTweet User don't created with success!\n");
    }

    return result;
}
