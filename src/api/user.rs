use log::error;
use reqwest::Error;

use crate::utils::api::get_authenticated;

use super::datatypes::{UserResponse, User, UserBackgroundResponse, UserBackground};

pub async fn get_user_by_id(user_id: u64) -> Result<User, Error> {
    let response: Result<UserResponse, Error> = get_authenticated(format!("/user/{}", user_id)).await.json().await;

    match response {
        Ok(user) => Ok(user.content),
        Err(why) => {
            error!("Could not get user: {:?}", why);
            Err(why)
        }
    }
}

pub async fn get_user_background_by_id(user_id: u64) -> Result<UserBackground, Error> {
    let response: Result<UserBackgroundResponse, Error> = get_authenticated(format!("/user/{}/background", user_id)).await.json().await;

    match response {
        Ok(user) => Ok(user.content),
        Err(why) => {
            error!("Could not get user background: {:?}", why);
            Err(why)
        }
    }
}