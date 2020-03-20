extern crate clap;

use std::env;
use clap::{Arg, App, SubCommand};
use std::fmt::Write;

fn main() {
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

    if let Some(matches) = matches.subcommand_matches("createUser")
    {
        create_user(matches);
	}
    else if let Some(matches) = matches.subcommand_matches("createUserTweet")
    {
        println!("SubCommand createUserTweet not implemented!");     
	}
    else if let Some(matches) = matches.subcommand_matches("showAllUserTweet")
    {
        println!("SubCommand showAllUserTweet not implemented!");
	}
    else if let Some(matches) = matches.subcommand_matches("viewUserTweet")
    {
        println!("SubCommand viewUserTweet not implemented!");
    }
    else { println!("Invalid Arguments!"); }
}

fn len_vec_str(input_vec : Vec<&str>) -> usize
{
  let mut out = String::new();
    
  for n in input_vec {
    let _ = write!(&mut out, "{}", n);
  }

  return out.chars().count();
}

fn create_user(cmd : &clap::ArgMatches)
{
    let name: Vec<&str> = cmd.values_of("UserName").unwrap().collect();

    let login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();
    
    let mut checkUserInfo : bool = false;
  
    if len_vec_str(name) == 0
    {
        println!("\nInvalid UserName\n");
    }
    else if len_vec_str(login) == 0 
    {
        println!("\nInvalid UserLogin\n");
    }
    else if len_vec_str(password) == 0  
    {
        println!("\nInvalid UserPassword\n");
    }
    else
    {
        checkUserInfo = true;
    }
  
    if checkUserInfo 
    {
        //Code for include user data in file here..
        println!("\nUser created with success!\n");
    }
    else 
    {
        println!("\nUser don't created with success!\n");
    }
}