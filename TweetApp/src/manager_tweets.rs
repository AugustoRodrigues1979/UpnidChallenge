//module responsible for manager all data the tweet user
extern crate clap;

use crate::routines::len_vec_str;
use crate::routines::vec_str_to_string;
use crate::routines::vec_str_to_i32;

use crate::database::create_database;
use crate::database::add_tweet_user;
use crate::database::get_all_tweets_by_user;
use crate::database::get_tweet_by_id;

use crate::database::TweetUserData;

use std::string::ToString;

pub enum CodeManagerTweet {
    CodeOk,
    InvalidTweetData,
    InvalidLogin,
    InvalidPassword,
    InvalidTweetId
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

    if let CodeManagerTweet::CodeOk = result 
    {
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

pub fn viewUserTweet(cmd : &clap::ArgMatches) -> CodeManagerTweet
{
    let mut result : self::CodeManagerTweet = CodeManagerTweet::CodeOk;

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    let ref tweet_id: Vec<&str> = cmd.values_of("TweetID").unwrap().collect();

    if len_vec_str(login.to_vec()) == 0 
    {
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        result = CodeManagerTweet::InvalidPassword;
    }
    else if len_vec_str(tweet_id.to_vec()) == 0  
    {
        result = CodeManagerTweet::InvalidTweetId;
    }

    if let CodeManagerTweet::CodeOk = result 
    {
        let mut tweetInfo = TweetUserData {tweet:"".to_string(), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };
        create_database();
        
        let tweet_id_i32 = vec_str_to_i32(tweet_id.to_vec());

        let (status, mut list_tweets) = get_tweet_by_id(&mut tweetInfo, 
                                                        tweet_id_i32);
        
        if status 
        {
            println!("\nListing one tweet for user specified\n");

            for t in list_tweets.iter()
            {
                println!("{:?}\n", t);  
			}             
        }
        else 
        { 
            println!("Not found tweets for user specified\n"); 
        }
    }
    else
    {
        println!("\nInvalid User or Invalid Login!\n");
    }

    return result;
}

pub fn show_all_tweets_by_user(cmd : &clap::ArgMatches) -> CodeManagerTweet
{    
    let mut result : self::CodeManagerTweet = CodeManagerTweet::CodeOk;

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    if len_vec_str(login.to_vec()) == 0 
    {
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        result = CodeManagerTweet::InvalidPassword;
    }

    if let CodeManagerTweet::CodeOk = result 
    {
        let mut tweetInfo = TweetUserData {tweet:"".to_string(), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };
        create_database();
        
        let (status, mut list_tweets) = get_all_tweets_by_user(&mut tweetInfo);
        
        if status 
        {
            println!("\nListing All Tweets for user specified\n");

            for t in list_tweets.iter() 
            {
                println!("{:?}\n", t);  
			}             
        }
        else 
        { 
            println!("Not found tweets for user specified\n"); 
        }
    }
    else
    {
        println!("\nInvalid User or Invalid Login!\n");
    }

    return result;
}
