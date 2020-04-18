//module responsible for manager all data the tweet user
extern crate clap;

use crate::lib::routines::len_vec_str;
use crate::lib::routines::vec_str_to_string;
use crate::lib::routines::vec_str_to_i32;

use crate::lib::database::add_tweet_user;
use crate::lib::database::get_tweet_by_id;
use crate::lib::database::insert_like_tweet;
use crate::lib::database::remove_like_tweet;
use crate::lib::database::get_any_tweet;
use crate::lib::database::get_user_id;
use crate::lib::database::get_all_tweets_by_user;
use crate::lib::database::update_status_like_tweet;
use crate::lib::database::get_total_like_tweet;
use crate::lib::database::TweetUserData;

use std::string::ToString;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeManagerTweet {
    CodeOk,
    InvalidTweetData,
    InvalidLogin,
    InvalidPassword,
    InvalidTweetId,
    NotFoundTweets
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
        add_tweet_user(&mut tweet_info);

        println!("\nUser tweet created with success\n");
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
        println!("\nInvalid login!\n");
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        println!("\nInvalid password!\n");
        result = CodeManagerTweet::InvalidPassword;
    }
    else if len_vec_str(tweet_id.to_vec()) == 0  
    {
        println!("\nInvalid tweet id!\n");
        result = CodeManagerTweet::InvalidTweetId;
    }

    if result != CodeManagerTweet::CodeOk
    {
        return result;
	}

    let (check_user, _user_id, user_name) = get_user_id(vec_str_to_string(login.to_vec()),
                                            vec_str_to_string(password.to_vec())
                                            );
    if check_user == false 
    {
        println!("\nInvalid Login or Password!\n");
        return CodeManagerTweet::InvalidLogin;
	}

    let (check_tweet_id, tweet_id_i32) = vec_str_to_i32(tweet_id.to_vec());
    
    if check_tweet_id == false
    {
        println!("\nInvalid tweet id!\n");
        return CodeManagerTweet::InvalidLogin;
    }

    if let CodeManagerTweet::CodeOk = result 
    {
        let mut tweet_info = TweetUserData {tweet:"".to_string(), 
                                           login:vec_str_to_string(login.to_vec()), 
                                           password:vec_str_to_string(password.to_vec()) };

        let (status, list_tweets) = get_tweet_by_id(&mut tweet_info, 
                                                        tweet_id_i32);
        
        if status 
        {
            println!("\nListing one tweet for user specified\n");

            let title_usename  = format!("{:-^1$}", "UserName".to_string(), 30);
            let title_tw_id    = format!("{:-^1$}", "Tweet Id".to_string(), 10);
            let title_like_tw  = format!("{:-^1$}", "Like".to_string(), 10);
            let title_tw_body  = format!("{:-^1$}", "Tweet Body".to_string(), 70);

            println!("\n{} {} {} {}\n", 
                    title_usename,
                    title_tw_id,
                    title_like_tw,
                    title_tw_body);
            
            let mut txt_like_tw : String = String ::new();

            for t in list_tweets.iter()
            {
                let txt_usename  = format!("{: <1$}", user_name.to_string(), 30);
                let txt_tw_id    = format!("{: ^1$}", t.tweet_id.to_string(), 10);

                txt_like_tw.clear();
                txt_like_tw.push_str(&format!("{: ^1$}", t.liked.to_string(), 10));

                let txt_tw_body  = format!("{: <1$}", t.tweet_data.to_string(), 70);

                println!("\n{} {} {} {}\n", 
                    txt_usename,
                    txt_tw_id,
                    txt_like_tw,
                    txt_tw_body);
            }             
        }
        else 
        { 
            println!("\nNot found tweets for user specified\n"); 
        }
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
        println!("\nInvalid login\n");
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        println!("\nInvalid password\n");
        result = CodeManagerTweet::InvalidPassword;
    }

    if result != CodeManagerTweet::CodeOk
    {
        return result;
	}
 
    let (check_user, _user_id, user_name) = get_user_id(vec_str_to_string(login.to_vec()),
                                        vec_str_to_string(password.to_vec())
                                        );
    if check_user == false 
    {
        println!("\nInvalid Login or Password\n");
        return CodeManagerTweet::InvalidLogin;
    }

    let mut tweet_info = TweetUserData { tweet:"".to_string() , 
                                            login:vec_str_to_string(login.to_vec()), 
                                            password:vec_str_to_string(password.to_vec()),
                                        };

    let (status,list_tweets) = get_all_tweets_by_user(&mut tweet_info);

    if !status
    {
        println!("\nNot found tweets created by user\n");  
        return CodeManagerTweet::NotFoundTweets;
	}

    println!("\nListing All Tweets for user specified\n");
            
    let title_usename  = format!("{:-^1$}", "UserName".to_string(), 30);
    let title_tw_id    = format!("{:-^1$}", "Tweet Id".to_string(), 10);
    let title_like_tw  = format!("{:-^1$}", "Like".to_string(), 10);
    let title_tw_body  = format!("{:-^1$}", "Tweet Body".to_string(), 70);

    println!("\n{} {} {} {}\n", 
        title_usename,
        title_tw_id,
        title_like_tw,
        title_tw_body);
            
    let mut txt_like_tw : String = String ::new();

    for t in list_tweets.iter()
    {
        let txt_usename  = format!("{: <1$}", user_name.to_string(), 30);
        let txt_tw_id    = format!("{: ^1$}", t.tweet_id.to_string(), 10);

        txt_like_tw.clear();
        let (_status_like, amount_like) = get_total_like_tweet( t.tweet_id, true);

        txt_like_tw.push_str(&format!("{: ^1$}", amount_like.to_string(), 10));

        let txt_tw_body  = format!("{: <1$}", t.tweet_data.to_string(), 70);

        println!("\n{} {} {} {}\n", 
            txt_usename,
            txt_tw_id,
            txt_like_tw,
            txt_tw_body);
    }

    return CodeManagerTweet::CodeOk;
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
        println!("\nInvalid login\n");
        result = CodeManagerTweet::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        println!("\nInvalid password\n");
        result = CodeManagerTweet::InvalidPassword;
    }
    else if len_vec_str(tweet_id.to_vec()) == 0  
    {
        println!("\nInvalid tweet id!\n");
        result = CodeManagerTweet::InvalidTweetId;
    }

    let (check_user, user_id, _user_name) = get_user_id(vec_str_to_string(login.to_vec()),
                                                       vec_str_to_string(password.to_vec()));
    if check_user == false
    {
        println!("\nInvalid Login or Password!\n");
        return CodeManagerTweet::InvalidLogin;
	}

    let (check_tweet_id, tweet_id_i32) = vec_str_to_i32(tweet_id.to_vec());
    
    if check_tweet_id == false
    {
        println!("\nInvalid tweet id!\n");
        return CodeManagerTweet::InvalidLogin;
    }

    if let CodeManagerTweet::CodeOk = result 
    {        
        let (status, _) = get_any_tweet(tweet_id_i32);        
        if status
        {
            //Here we have a valid user_id and tweet_id
            if remove_like
            {
                remove_like_tweet(user_id, tweet_id_i32);
 			}
            else
            {
                insert_like_tweet(user_id, tweet_id_i32);
			}

            //For here exist at least one tweet of user specifieds
            //So we will update the status's like of tweet
            let (status_like, amount_like) = get_total_like_tweet(tweet_id_i32, true);
            if status_like 
            {
                update_status_like_tweet(amount_like != 0,user_id, tweet_id_i32 );
            }

            println!("\nLike inserted or removed with success!\n");
        }
        else
        {
            println!("\nNot found tweets created by user\n");}
    }
    return result;
}
