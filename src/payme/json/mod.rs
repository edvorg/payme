extern crate iron;
extern crate bodyparser;
extern crate persistent;

use iron::prelude::*;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InvoiceInfo {
    pub task: String,
    pub hours: String,
    pub rate: String,
    pub email: String,
    pub company: String,
    pub company_address: String,
    pub client_email: String,
    pub client_company: String,
    pub client_company_address: String,
    pub terms: String,
}

pub fn parse_invoice_info(request: &mut Request) -> Result<Option<InvoiceInfo>, bodyparser::BodyError> {
    request.get::<bodyparser::Struct<InvoiceInfo>>()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RecaptchaResponse {
    pub success: bool,
    pub challenge_ts: String,
    pub hostname: String,
}

pub fn parse_recaptcha_response(request: &mut Request) -> Result<Option<RecaptchaResponse>, bodyparser::BodyError> {
    request.get::<bodyparser::Struct<RecaptchaResponse>>()
}

pub fn middleware() -> persistent::Read<bodyparser::MaxBodyLength> {
    persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH)
}
