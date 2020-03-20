//module responsible for manager all tweets contents
extern crate clap;

mod routines;

use clap::{Arg, App, SubCommand};

mod manager_users {
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
}

