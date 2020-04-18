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

fn create_tweet_user(cmd : &clap::ArgMatches)
{
    manager_tweets::create_tweet_user(&cmd);
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
    let matches = App::new("TweetApp")
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
        ).get_matches();

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
        println!("\nPlease the use option help for more information!\n"); 
    }
}

#[cfg(test)]
mod tests {

  use super::*;

  fn setup() {
    database::clean_database(TypeDataBase::DataBaseTest);
    database::create_database(TypeDataBase::DataBaseTest);
  }

  #[test]
  fn create_user() {

    setup();

    let arg_vec = vec!["TweetApp", "createUser", "userLoginTest", "userNameTest", "passwordTest"];

    //let cmd : clap::App = App::new("TweetApp");

    let cmd = create_default_line_options();
    
    let args_prog : clap::ArgMatches = cmd.get_matches_from(arg_vec);
    //.get_matches_from(arg_vec);

    if let Some(cmd) = args_prog.subcommand_matches("createUser")
    {
      assert_eq!(CodeManagerUser::CodeOk, super::create_user(&cmd));
    }
    //assert_eq!(1, 1);
  }
}
