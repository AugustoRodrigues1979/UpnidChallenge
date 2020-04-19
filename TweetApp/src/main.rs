//main of the app

extern crate clap;
extern crate mysql;

mod lib;

use clap::{Arg, App};

use lib::database;
use lib::database::TypeDataBase;

use lib::manager_users;
use lib::manager_tweets;
use lib::manager_users::CodeManagerUser;
use lib::manager_tweets::CodeManagerTweet;

use lib::routines::enable_app_msg;


fn create_default_line_options<'b,'c>() -> clap::App<'b,'c>
{
  App::new("TweetApp")
            .version("1.0")
            .author("Augusto Rodrigues <augusto_mr@yahoo.com.br>")
            .about("Upnid Challenges")
            .subcommand(
                        App::new("createUser")
                        .about("Create a user")
                        .args(
                          &[Arg::from_usage("<UserName> 'User Name'"),
                          Arg::from_usage("<UserLogin> 'Login used by User'"),
                          Arg::from_usage("<UserPassword> 'PassWord used by User'")
                          ]
                        )
            )
            .subcommand(
                        App::new("createUserTweet")
                        .about("Create a Tweet for user")
                        .args(
                          &[Arg::from_usage("<UserLogin> 'Login used by User'"),
                          Arg::from_usage("<UserPassword> 'PassWord used by User'"),
                          Arg::from_usage("<TweetText> 'Contents of Tweet, delimited between double quote'"),
                          ]
                        )
            )
            .subcommand(
                        App::new("showAllUserTweet")
                        .about("Show all Tweets for specified user")
                        .args(
                          &[Arg::from_usage("<UserLogin> 'Login used by User'"),
                          Arg::from_usage("<UserPassword> 'PassWord used by User'"),
                          ]
                        )
            )
            .subcommand(
                        App::new("viewUserTweet")
                        .about("View one specific tweet for specified user")
                        .args(
                          &[Arg::from_usage("<UserLogin> 'Login used by User'"),
                          Arg::from_usage("<UserPassword> 'PassWord used by User'"),                      
                          Arg::from_usage("<TweetID> 'User's Tweet Unique ID'"),
                          ]
                        )
            )
            .subcommand(
                        App::new("likeTweet")
                        .about("Like or Unlike one specific tweet")
                        .args(
                          &[Arg::from_usage("-u 'Unlike tweet'"),
                          Arg::from_usage("<UserLogin> 'Login used by User'"),
                          Arg::from_usage("<UserPassword> 'PassWord used by User'"),                      
                          Arg::from_usage("<TweetID> 'User's Tweet Unique ID'"),
                          ]
                        )
            )
            .subcommand(
              App::new("followUser")
              .about("Follow or Unfollow one specific user")
              .args(
                &[Arg::from_usage("-u 'Unfollow user'"),
                Arg::from_usage("<UserLogin> 'Login used by User'"),
                Arg::from_usage("<UserPassword> 'PassWord used by User'"),                      
                Arg::from_usage("<UserId> 'Unique ID user to follow'"),
                ]
            )
          )
}

fn create_user(cmd : &clap::ArgMatches) -> CodeManagerUser
{
    return manager_users::create_user(&cmd);
}

fn create_tweet_user(cmd : &clap::ArgMatches) -> CodeManagerTweet
{
    return manager_tweets::create_tweet_user(&cmd);
}

fn show_all_tweets_by_user(cmd : &clap::ArgMatches)
{
    manager_tweets::show_all_tweets_by_user(&cmd);
}

fn view_user_tweet(cmd : &clap::ArgMatches)
{
    manager_tweets::view_user_tweet(&cmd);
}

fn like_user_tweet(cmd : &clap::ArgMatches)
{
    manager_tweets::like_user_tweet(&cmd);
}

fn follow_user(cmd : &clap::ArgMatches)
{
    manager_users::follow_user(&cmd);
}


fn main()
{
    enable_app_msg(true);

    let matches = create_default_line_options().get_matches();

    database::create_database(TypeDataBase::DataBaseProduction);

    if let Some(matches) = matches.subcommand_matches("createUser")
    {
      create_user(&matches);
	  }
    else if let Some(matches) = matches.subcommand_matches("createUserTweet")
    {
      create_tweet_user(&matches);
    }
    else if let Some(matches) = matches.subcommand_matches("showAllUserTweet")
    {
      show_all_tweets_by_user(&matches);
    }
    else if let Some(matches) = matches.subcommand_matches("viewUserTweet")
    {
      view_user_tweet(&matches);
    }
    else if let Some(matches) = matches.subcommand_matches("likeTweet")
    {
      like_user_tweet(&matches);
    }
    else if let Some(matches) = matches.subcommand_matches("followUser")
    {
      follow_user(&matches);
    }
    else
    { 
      print_app_msg!("\nPlease the use option help for more information!\n"); 
    }
}

#[cfg(test)]
mod tests {

  use super::*;

  fn setup() {
    enable_app_msg(false);
    database::clean_database(TypeDataBase::DataBaseTest);
    database::create_database(TypeDataBase::DataBaseTest);
  }

  #[test]
  fn create_user() {

    setup();

    let arg_vec = vec!["TweetApp", "createUser", "userLoginTest", "userNameTest", "passwordTest"];

    let cmd = create_default_line_options();
    
    let args_prog : clap::ArgMatches = cmd.get_matches_from(arg_vec);


    if let Some(cmd) = args_prog.subcommand_matches("createUser")
    {
      assert_eq!(CodeManagerUser::CodeOk, super::create_user(&cmd));
    }
  }

  #[test]
  fn dont_insert_same_user() {

    setup();

    let arg_vec = vec!["TweetApp", "createUser", "userLoginTest", "userNameTest", "passwordTest"];

    let cmd = create_default_line_options();
    
    let args_prog : clap::ArgMatches = cmd.get_matches_from(arg_vec);

    if let Some(cmd) = args_prog.subcommand_matches("createUser")
    {
      assert_eq!(CodeManagerUser::CodeOk, super::create_user(&cmd));
      assert_eq!(CodeManagerUser::DoNotIncludeNewUser, super::create_user(&cmd));
    }
  }

  #[test]
  fn dont_insert_same_user_login() 
  {

    setup();

    let arg_vec_orign = vec!["TweetApp", "createUser", "userName 1", "userLoginTest", "passwordTest"];
    let arg_vec_dup = vec!["TweetApp", "createUser", "userName 2", "userLoginTest", "passwordTest"];

    let cmd_orign = create_default_line_options();
    let cmd_dup = create_default_line_options();
    
    let args_prog_orign : clap::ArgMatches = cmd_orign.get_matches_from(arg_vec_orign);
    let args_prog_dup : clap::ArgMatches = cmd_dup.get_matches_from(arg_vec_dup);

    let cmd_orign = match args_prog_orign.subcommand_matches("createUser")
    {
      Some(a) => a,
      _ => panic!("Error: Do not process line for create 'userName 1'!!!")
    };

    let cmd_dup = match args_prog_dup.subcommand_matches("createUser")
    {
      Some(a) => a,
      _ => panic!("Error: Do not process line for create 'userName 2'!!!")
    };

    assert_eq!(CodeManagerUser::CodeOk, super::create_user(&cmd_orign));
    assert_eq!(CodeManagerUser::DoNotIncludeNewUser, super::create_user(&cmd_dup));
  }

  #[test]
  fn create_tweet_user()
  {
    setup();

    let arg_create_user = vec!["TweetApp", "createUser", "userName 1", "userLoginTest", "passwordTest"];
    let arg_create_tweet = vec!["TweetApp", "createUserTweet", "userLoginTest", "passwordTest", "Tweet de Teste"];

    let cmd_create_user = create_default_line_options();
    let cmd_create_tweet = create_default_line_options();
    
    let args_create_user  : clap::ArgMatches = cmd_create_user.get_matches_from(arg_create_user);
    let args_create_tweet : clap::ArgMatches = cmd_create_tweet.get_matches_from(arg_create_tweet);

    let match_cmd_create_user = match args_create_user.subcommand_matches("createUser")
    {
      Some(a) => a,
      _ => panic!("Error: Do not process line arguments for create 'userName 1'!!!")
    };

    let match_cmd_create_tweet = match args_create_tweet.subcommand_matches("createUserTweet")
    {
      Some(a) => a,
      _ => panic!("Error: Do not process line for create 'userName 2'!!!")
    };

    assert_eq!(CodeManagerUser::CodeOk, super::create_user(&match_cmd_create_user));
    assert_eq!(CodeManagerTweet::CodeOk, super::create_tweet_user(&match_cmd_create_tweet));
  }  
}
