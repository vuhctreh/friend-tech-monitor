use std::{env, thread};
use std::env::VarError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct SmsInitRequest {
    #[serde(rename="phoneNumber")]
    phone_number: String,
}

impl SmsInitRequest {
    pub fn new() -> SmsInitRequest {
        let number: Result<String, VarError> = env::var("PHONE_NUMBER");

        match number {
            Ok(x) => {
                SmsInitRequest {
                    phone_number: x,
                }
            }
            Err(e) => {
                log::error!("Could not read phone number in env: {}", e);
                thread::sleep(std::time::Duration::from_secs(4));
                panic!("{}", e)
            }
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SmsInitResponse {
    pub(crate) status: String,
}

#[derive(Serialize, Clone)]
pub struct SmsAuthRequest {
    #[serde(rename="phoneNumber")]
    phone_number: String,
    code: String,
}

impl SmsAuthRequest {
    pub fn new(code: String) -> SmsAuthRequest {
        let number: Result<String, VarError> = env::var("PHONE_NUMBER");

        match number {
            Ok(x) => {
                SmsAuthRequest {
                    phone_number: x,
                    code
                }
            }
            Err(e) => {
                log::error!("Could not read phone number in env: {}", e);
                thread::sleep(std::time::Duration::from_secs(4));
                panic!("{}", e)
            }
        }
    }
}

