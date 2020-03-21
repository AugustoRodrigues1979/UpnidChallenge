//main of the app
extern crate clap;

mod routines;
mod manager_users;
mod database;

use clap::{Arg, App};
use crate::database::create_database;


fn main()
{
    println!("Begin.....\n");
    //create_database();

    main1();
}


fn main1() {
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
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("createUser")
    {
        manager_users::create_user(&matches);
	}
    else if let Some(_matches) = matches.subcommand_matches("createUserTweet")
    {
        println!("SubCommand createUserTweet not implemented!");     
	}
    else if let Some(_matches) = matches.subcommand_matches("showAllUserTweet")
    {
        println!("SubCommand showAllUserTweet not implemented!");
	}
    else if let Some(_matches) = matches.subcommand_matches("viewUserTweet")
    {
        println!("SubCommand viewUserTweet not implemented!");
    }
    else { println!("Invalid Arguments!"); }
}
