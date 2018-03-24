extern crate hmac;
extern crate sha2;
extern crate hex_slice;

use self::sha2::Sha256;
use self::hmac::{Hmac, Mac};
use payme::json;
use payme::config;
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

fn make_hmac(op: String, id: isize, invoice: json::InvoiceInfo) -> Hmac<Sha256> {
    let mut hmac = Hmac::<Sha256>::new(config::get_crypto_secret().as_bytes()).unwrap();
    hmac.input(format!("{}{}{:?}", op, id, invoice).as_bytes());
    hmac
}

fn gen_token(op: String, id: isize, invoice: json::InvoiceInfo) -> String {
    let hmac = make_hmac(op, id, invoice);
    let result = hmac.result();
    let code_bytes = result.code();
    let mut s = "".to_string();
    for x in code_bytes {
        s = format!("{}{}", s, hex(x))
    }
    s
}

#[test]
fn gen_token_test() {
    assert_eq!("d7b9eb850129f83f14abaf617da614c4bd167dada9b9b45467bda38af079fa00", gen_token("".to_string(),
                                                                                             0,
                                                                                             make_test_info()));
}

fn is_token_valid(op: String, id: isize, invoice: json::InvoiceInfo, token: String) -> bool {
    let hmac = make_hmac(op, id, invoice);
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

#[test]
fn is_token_valid_test() {
    assert!(is_token_valid("".to_string(),
                           0,
                           make_test_info(),
                           "d7b9eb850129f83f14abaf617da614c4bd167dada9b9b45467bda38af079fa00".to_string()));
    assert!(!is_token_valid("".to_string(),
                            0,
                            make_test_info(),
                            "1f4e576dc41d78e8d58236daf288c7322117791815fbedc9617a877e2e226025".to_string()));
}

pub fn gen_recipt_token(id: isize, invoice: json::InvoiceInfo) -> String {
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

pub fn gen_unsubscribe_token(id: isize, invoice: json::InvoiceInfo) -> String {
    gen_token("unsubscribe".to_string(), id, invoice)
}

pub fn is_unsubscribe_token_valid(id: isize, invoice: json::InvoiceInfo, token: String) -> bool {
    is_token_valid("unsubscribe".to_string(), id, invoice, token)
}
