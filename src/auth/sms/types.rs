//! This module contains the Request and Response structs
//! for calling privy.io auth endpoints and Kosetto
//! token signature.

use std::{env, thread};
use std::env::VarError;
use serde::{Deserialize, Serialize};
use eyre::{eyre, Result};

#[derive(Serialize, Clone)]
pub struct SmsInitRequest {
    #[serde(rename="phoneNumber")]
    phone_number: String,
}

impl SmsInitRequest {
    pub fn new() -> Result<SmsInitRequest> {
        let number: String = env::var("PHONE_NUMBER")?;

        if number.is_empty() {
            return Err(eyre!("PHONE_NUMBER env var is not set."));
        }

        Ok(SmsInitRequest {
            phone_number: number,
        })
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

#[derive(Deserialize, Clone)]
pub struct SmsAuthResponse {
    pub user: PrivyUser,
    is_new_user: bool,
    pub token: String,
    refresh_token: String,
}

#[derive(Deserialize, Clone)]
pub struct PrivyUser {
    id: String,
    created_at: u64,
    pub linked_accounts: Vec<LinkedAccounts>,
}

#[derive(Deserialize, Clone)]
pub struct LinkedAccounts {
    #[serde(rename="type")]
    account_type: String,
    verified_at: u64,
    phone_number: Option<String>,
    pub address: Option<String>,
    chain_id: Option<String>,
    chain_type: Option<String>,
    wallet_client: Option<String>,
    wallet_client_type: Option<String>,
    connector_type: Option<String>,
    recovery_method: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct SignTokenRequest<'a> {
    address: &'a str
}

impl SignTokenRequest<'_> {
    pub fn new(address: &str) -> SignTokenRequest {
        SignTokenRequest {
            address
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct SignTokenResponse {
    message: String,
    pub token: String,
}
