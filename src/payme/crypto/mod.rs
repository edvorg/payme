extern crate hex_slice;
extern crate hmac;
extern crate sha2;

use self::hmac::{Hmac, Mac};
use self::sha2::Sha256;
use payme::config;
use payme::json;
use std::u8;

#[allow(dead_code)]
fn make_test_info() -> json::InvoiceInfo {
    json::InvoiceInfo {
        task: "".to_string(),
        hours: 0,
        rate: 0,
        email: "payme@rust.cafe".to_string(),
        company: "Edward Knyshov".to_string(),
        company_address: "".to_string(),
        client_email: "test@rust.cafe".to_string(),
        client_company: "XLucidity".to_string(),
        client_company_address: "".to_string(),
        terms: "".to_string(),
        number: 0,
        date: None,
    }
}
#[warn(dead_code)]

fn hex(n: u8) -> String {
    let mut s: String = format!("{:x}", n);
    if s.len() == 1 {
        s = format!("0{}", s);
    }
    s
}

#[test]
fn hex_test_0() {
    assert_eq!("00", hex(0));
}

#[test]
fn hex_test_1() {
    assert_eq!("01", hex(1));
}

#[test]
fn hex_test_66() {
    assert_eq!("42", hex(66));
}

#[test]
fn hex_test_127() {
    assert_eq!("7f", hex(127));
}

#[test]
fn hex_test_128() {
    assert_eq!("80", hex(128));
}

#[test]
fn hex_test_255() {
    assert_eq!("ff", hex(255));
}

fn from_hex(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap_or(0)
}

#[test]
fn from_hex_test_0() {
    assert_eq!(0, from_hex("00"));
}

#[test]
fn from_hex_test_1() {
    assert_eq!(1, from_hex("01"));
}

#[test]
fn from_hex_test_66() {
    assert_eq!(66, from_hex("42"));
}

#[test]
fn from_hex_test_127() {
    assert_eq!(127, from_hex("7f"));
}

#[test]
fn from_hex_test_128() {
    assert_eq!(128, from_hex("80"));
}

#[test]
fn from_hex_test_255() {
    assert_eq!(255, from_hex("ff"));
}

fn make_hmac_generic(s: String) -> Hmac<Sha256> {
    let mut hmac = Hmac::<Sha256>::new(config::get_crypto_secret().as_bytes()).unwrap();
    hmac.input(s.as_bytes());
    hmac
}

fn concat_digest(op: String, id: isize, invoice: json::InvoiceInfo) -> String {
    format!("{}{}{:?}", op, id, invoice)
}

fn gen_token_generic(s: String) -> String {
    let hmac = make_hmac_generic(s);
    let result = hmac.result();
    let code_bytes = result.code();
    let mut s = "".to_string();
    for x in code_bytes {
        s = format!("{}{}", s, hex(x))
    }
    s
}

fn gen_token(op: String, id: isize, invoice: json::InvoiceInfo) -> String {
    gen_token_generic(concat_digest(op, id, invoice))
}

fn is_token_valid_generic(s: String, token: String) -> bool {
    let hmac = make_hmac_generic(s);
    let cs: Vec<char> = token.chars().collect();
    let mut v = Vec::new();
    for chunk in cs.chunks(2) {
        v.push(from_hex(&format!("{}{}", chunk[0], chunk[1])));
    }
    match hmac.verify(&v) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_token_valid(op: String, id: isize, invoice: json::InvoiceInfo, token: String) -> bool {
    is_token_valid_generic(concat_digest(op, id, invoice), token)
}

pub fn gen_invoice_token(id: isize, invoice: json::InvoiceInfo) -> String {
    gen_token("invoice".to_string(), id, invoice)
}

pub fn is_invoice_token_valid(id: isize, invoice: json::InvoiceInfo, token: String) -> bool {
    is_token_valid("invoice".to_string(), id, invoice, token)
}

pub fn gen_receipt_token(id: isize, invoice: json::InvoiceInfo) -> String {
    gen_token("receipt".to_string(), id, invoice)
}

pub fn is_receipt_token_valid(id: isize, invoice: json::InvoiceInfo, token: String) -> bool {
    is_token_valid("receipt".to_string(), id, invoice, token)
}

pub fn gen_unsubscribe_token(email: String) -> String {
    gen_token_generic(email)
}

pub fn is_unsubscribe_token_valid(email: String, token: String) -> bool {
    is_token_valid_generic(email, token)
}
