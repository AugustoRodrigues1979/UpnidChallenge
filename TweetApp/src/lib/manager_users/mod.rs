//module responsible for manager all data the tweet user


extern crate clap;

use crate::lib::routines::len_vec_str;
use crate::lib::routines::vec_str_to_string;
use crate::lib::routines::vec_str_to_i32;
use crate::lib::database::add_user;
use crate::lib::database::add_follower_user;
use crate::lib::database::FollowUserData;
use crate::lib::database::get_user_by_id;
use crate::lib::database::get_user_id;

use crate::lib::database::UserData;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeManagerUser {
    CodeOk,
    InvalidUserName,
    InvalidLogin,
    InvalidPassword,
    DoNotIncludeNewUser
}

pub fn follow_user(cmd : &clap::ArgMatches) -> CodeManagerUser
{
    let mut result : self::CodeManagerUser = CodeManagerUser::CodeOk;
    
    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    let ref user_id_followed: Vec<&str> = cmd.values_of("UserId").unwrap().collect();


    if len_vec_str(login.to_vec()) == 0 
    {
        print_app_msg!("\nInvalid UserLogin\n");
        result = CodeManagerUser::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        print_app_msg!("\nInvalid UserPassword\n");
        result = CodeManagerUser::InvalidPassword;
    }

    if CodeManagerUser::CodeOk != result {
        return result;
    }

    let (check_user, user_id_follower, user_name) = get_user_id(vec_str_to_string(login.to_vec()),
                                            vec_str_to_string(password.to_vec())
                                            );

    if check_user == false 
    {
        print_app_msg!("\nInvalid User Login or User Password!\n");
        return CodeManagerUser::InvalidLogin;
    }
    
    let (check_user, _, _) = get_user_by_id(vec_str_to_string(user_id_followed.to_vec()));
    let (_,id) = vec_str_to_i32(user_id_followed.to_vec());

    if (check_user == false) || ( id == user_id_follower) 
    {
        print_app_msg!("\nInvalid User ID to follow!\n");
        return CodeManagerUser::InvalidLogin;
    }

    let mut status_follow : String = String::new();
    
    if cmd.values_of("u").is_none() {
        status_follow.push_str("1");
    }
    else
    {
        status_follow.push_str("0");
    }

    let follow_info  =  FollowUserData {
        status_follow:status_follow.to_string(), 
        user_id_follower:user_id_follower.to_string(),
        user_id_followed:vec_str_to_string(user_id_followed.to_vec()) };

    if add_follower_user(&follow_info)
    {
        print_app_msg!("\nStatus of followed User updated with success\n");
    }
    else    
    {
        print_app_msg!("\nSorry but don't possible follow the specified user\n");
        result = CodeManagerUser::InvalidLogin;
    }

    return result;
}

pub fn create_user(cmd : &clap::ArgMatches) -> CodeManagerUser
{    
    let mut result : self::CodeManagerUser = CodeManagerUser::CodeOk;
    let ref name: Vec<&str> = cmd.values_of("UserName").unwrap().collect();

    let ref login: Vec<&str> = cmd.values_of("UserLogin").unwrap().collect();

    let ref password: Vec<&str> = cmd.values_of("UserPassword").unwrap().collect();

    if len_vec_str(name.to_vec()) == 0
    {
        print_app_msg!("\nInvalid UserName\n");
        result = CodeManagerUser::InvalidUserName;
    }
    else if len_vec_str(login.to_vec()) == 0 
    {
        print_app_msg!("\nInvalid UserLogin\n");
        result = CodeManagerUser::InvalidLogin;
    }
    else if len_vec_str(password.to_vec()) == 0  
    {
        print_app_msg!("\nInvalid UserPassword\n");
        result = CodeManagerUser::InvalidPassword;
    }

    if let CodeManagerUser::CodeOk = result {
        //Code for include user data in file here..
        let mut user_info = UserData { name :vec_str_to_string(name.to_vec()), 
                                      login:vec_str_to_string(login.to_vec()), 
                                      password:vec_str_to_string(password.to_vec()) };

        let (status, msg_err) = add_user(&mut user_info);
        if status 
        { 
            print_app_msg!("\nUser created with success!\n");
        }
        else
        { 
            print_app_msg!("\n{}\n", msg_err);
            //result = CodeManagerUser::DoNotIncludeNewUser;
        }
    }
    else 
    {
        print_app_msg!("\nUser don't created with success!\n");
    }

    return result;
}
