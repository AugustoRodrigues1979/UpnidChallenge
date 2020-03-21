//module responsible for manager all data the tweet user
extern crate clap;

use crate::routines::len_vec_str;
use crate::database::*;

use std::string::ToString;

pub enum CodeManagerUser {
    CodeOk,
    InvalidUserName,
    InvalidLogin,
    InvalidPassword
}

pub fn create_user(cmd : &clap::ArgMatches) -> CodeManagerUser
{
    let name: Vec<&str> = cmd.values_of("UserName").unwrap().collect();

    let login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();
    
    let mut result : self::CodeManagerUser = CodeManagerUser::CodeOk;
  
    if len_vec_str(name) == 0
    {
        println!("\nInvalid UserName\n");
        result = CodeManagerUser::InvalidUserName;
    }
    else if len_vec_str(login) == 0 
    {
        println!("\nInvalid UserLogin\n");
        result = CodeManagerUser::InvalidLogin;
    }
    else if len_vec_str(password) == 0  
    {
        println!("\nInvalid UserPassword\n");
        result = CodeManagerUser::InvalidPassword;
    }

    if let CodeManagerUser::CodeOk = result {
        //Code for include user data in file here..
        let a : String = "vec_str_to_string(name)".to_owned();
        let b : String = "vec_str_to_string(name)".to_owned();
        let c : String = "vec_str_to_string(name)".to_owned();

        let mut userInfo = UserData { name:a, login:b, password:c };

        println!("\nUserName 1 :{}\n", userInfo.name);

        add_user(&mut userInfo);
        println!("\nUser created with success!\n");
    }
    else 
    {
        println!("\nUser don't created with success!\n");
    }

    return result;
}
