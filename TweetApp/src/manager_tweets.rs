//module responsible for manager all data the tweet user
extern crate clap;

use crate::routines::len_vec_str;
use crate::routines::vec_str_to_string;
use crate::routines::vec_str_to_i32;

use crate::database::create_database;
use crate::database::add_tweet_user;
use crate::database::get_all_tweets_by_user;
use crate::database::get_tweet_by_id;
use crate::database::insert_like_tweet;
use crate::database::remove_like_tweet;
use crate::database::get_any_tweet;
use crate::database::get_user_id;

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
        let mut tweet_info = TweetUserData {tweet:vec_str_to_string(tweetdata.to_vec()), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };
        create_database();
        add_tweet_user(&mut tweet_info);
    }
    else 
    {
        println!("\nTweet User don't created with success!\n");
    }

    return result;
}

pub fn view_user_tweet(cmd : &clap::ArgMatches) -> CodeManagerTweet
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
        let mut tweet_info = TweetUserData {tweet:"".to_string(), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };
        create_database();
        
        let tweet_id_i32 = vec_str_to_i32(tweet_id.to_vec());

        let (status, list_tweets) = get_tweet_by_id(&mut tweet_info, 
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
            println!("\nNot found tweets for user specified\n"); 
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
        let mut tweet_info = TweetUserData {tweet:"".to_string(), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };
        create_database();
        
        let (status,list_tweets) = get_all_tweets_by_user(&mut tweet_info);
        
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
            println!("\nNot found tweets for user specified\n"); 
        }
    }
    else
    {
        println!("\nInvalid User or Invalid Login!\n");
    }

    return result;
}

pub fn like_user_tweet(cmd : &clap::ArgMatches) -> CodeManagerTweet
{    
    let mut result : self::CodeManagerTweet = CodeManagerTweet::CodeOk;

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    let ref tweet_id: Vec<&str> = cmd.values_of("TweetID").unwrap().collect();

    //let ref unlike_option: Vec<&str> = cmd.values_of("-u").unwrap().collect();

    let remove_like : bool = cmd.values_of("u").is_none() == false;

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

    create_database();

    let (check_user, user_id) = get_user_id(vec_str_to_string(login.to_vec()),
                                            vec_str_to_string(password.to_vec())
                                            );
    if check_user == false 
    {
        println!("\nInvalid User!!\n");
        result = CodeManagerTweet::InvalidLogin;
	}

    if let CodeManagerTweet::CodeOk = result 
    {        
        let tweet_id_i32 = vec_str_to_i32(tweet_id.to_vec());

        let (status, _) = get_any_tweet(tweet_id_i32);        
        if status
        {
            //Here we have a valid user_id and tweet_id
            if remove_like
            {
                if remove_like_tweet(user_id, tweet_id_i32)
                {
                    println!("\nLike removed with success!\n");                
				}
                else
                {
                    println!("\nLike don't removed (maybe don't exist any like...)!\n");
				}
			}
            else
            {
                if insert_like_tweet(user_id, tweet_id_i32)
                {
                    println!("\nLike inserted with success!\n");
				}
                else
                {
                    println!("\nLike don't inserted with success!\n");                
				}
			}
        }
    }
    return result;
}
