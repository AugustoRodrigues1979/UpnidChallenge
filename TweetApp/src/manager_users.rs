//module responsible for manager all data the tweet user
extern crate clap;

use crate::routines::len_vec_str;
use crate::routines::vec_str_to_string;
use crate::database::add_user;

use crate::database::UserData;

pub enum CodeManagerUser {
    CodeOk,
    InvalidUserName,
    InvalidLogin,
    InvalidPassword
}

pub fn create_user(cmd : &clap::ArgMatches) -> CodeManagerUser
{    
    let mut result : self::CodeManagerUser = CodeManagerUser::CodeOk;
    let ref name: Vec<&str> = cmd.values_of("UserName").unwrap().collect();

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    if len_vec_str(name.to_vec()) == 0
    {
        println!("\nInvalid UserName\n");
        result = CodeManagerUser::InvalidUserName;
    }
    else if len_vec_str(login.to_vec()) == 0 
    {
        println!("\nInvalid UserLogin\n");
        result = CodeManagerUser::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        println!("\nInvalid UserPassword\n");
        result = CodeManagerUser::InvalidPassword;
    }

    if let CodeManagerUser::CodeOk = result {
        //Code for include user data in file here..
        let mut user_info = UserData { name :vec_str_to_string(name.to_vec()), 
                                      login:vec_str_to_string(login.to_vec()), 
                                      password:vec_str_to_string(password.to_vec()) };

        let (status, msg_err) = add_user(&mut user_info);
        if status { println!("\nUser created with success!\n"); }
        else      { println!("\n{}\n", msg_err); }
    }
    else 
    {
        println!("\nUser don't created with success!\n");
    }

    return result;
}
