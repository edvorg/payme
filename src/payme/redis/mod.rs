extern crate redis;
extern crate serde_json;
extern crate uuid;

use self::redis::{Commands, PipelineCommands};
use self::redis::RedisError;
use payme::json;
use self::uuid::Uuid;

static INVOICE_ID_KEY: &'static str = "invoice_id";
static INVOICE_EXPIRATION_SECONDS: usize = 1 * 60 * 60 * 24 * 20;
static UNSUBSCRIBE_EXPIRATION_SECONDS: usize = 1 * 60 * 60 * 24 * 30;

fn get_confirmed_key(id: isize) -> String {
    format!("invoice:confirmed:{}", id)
}

fn get_unsubscribed_key(email: String) -> String {
    format!("unsubscribed:{}", email)
}

fn get_info_key(id: isize) -> String {
    format!("invoice:info:{}", id)
}

fn redis_con() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_connection().unwrap()
}

pub fn get_new_invoice_id() -> isize {
    let con = redis_con();
    let (new_val,) : (isize,) = redis::transaction(&con, &[INVOICE_ID_KEY], |pipe| {
        let old_val : isize = con.get(INVOICE_ID_KEY).unwrap_or(0);
        pipe.set(INVOICE_ID_KEY, old_val + 1)
            .ignore()
            .get(INVOICE_ID_KEY)
            .query(&con)
    }).unwrap();
    new_val
}

#[test]
fn get_new_invoice_id_test() {
    let a = get_new_invoice_id();
    let b = get_new_invoice_id();
    assert!(0 < a && a < b);
}

pub fn set_confirmed(id: isize) -> bool {
    let con = redis_con();
    let _ : () = con.set(get_confirmed_key(id), true).unwrap();
    let _ : () = con.expire(get_confirmed_key(id), INVOICE_EXPIRATION_SECONDS).unwrap();
    true
}

#[test]
fn set_confirmed_test() {
    let id = get_new_invoice_id();
    assert!(set_confirmed(id));
}

pub fn is_confirmed(id: isize) -> bool {
    let con = redis_con();
    con.get(get_confirmed_key(id))
        .map(|s: String| {
            s == "true"
        }).unwrap_or(false)
}

#[test]
fn is_confirmed_test() {
    let id = get_new_invoice_id();
    assert!(!is_confirmed(id));
    set_confirmed(id);
    assert!(is_confirmed(id));
}

pub fn set_info(id: isize, invoice: json::InvoiceInfo) -> json::InvoiceInfo {
    let con = redis_con();
    let _ : () = con.set(get_info_key(id), serde_json::to_string(&invoice).unwrap()).unwrap();
    let _ : () = con.expire(get_info_key(id), INVOICE_EXPIRATION_SECONDS).unwrap();
    invoice
}

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

#[test]
fn set_info_test() {
    let id = get_new_invoice_id();
    let invoice = make_test_info();
    assert_eq!(invoice, set_info(id, invoice.clone()));
}

pub fn get_info(id: isize) -> Option<json::InvoiceInfo> {
    let con = redis_con();
    let s: Result<String, RedisError> = con.get(get_info_key(id));
    match s {
        Ok(s) => match serde_json::from_str(&s) {
            Ok(i) => Some(i),
            Err(_) => None,
        },
        _ => None,
    }
}

#[test]
fn get_info_test() {
    let id = get_new_invoice_id();
    let invoice = make_test_info();
    set_info(id, invoice.clone());
    assert_eq!(invoice, get_info(id).unwrap());
}

pub fn del_info(id: isize) {
    let con = redis_con();
    let _ : () = con.del(get_info_key(id)).unwrap();
}

#[test]
fn del_info_test() {
    let id = get_new_invoice_id();
    let invoice = make_test_info();
    set_info(id, invoice);
    del_info(id);
    assert_eq!(None, get_info(id));
}


pub fn set_unsubscribed(email: String) -> bool {
    let con = redis_con();
    let _ : () = con.set(get_unsubscribed_key(email.clone()), true).unwrap();
    let _ : () = con.expire(get_unsubscribed_key(email.clone()), UNSUBSCRIBE_EXPIRATION_SECONDS).unwrap();
    true
}

pub fn is_unsubscribed(email: String) -> bool {
    let con = redis_con();
    con.get(get_unsubscribed_key(email))
        .map(|s: String| {
            s == "true"
        }).unwrap_or(false)
}

#[test]
fn set_unsubscribed_test() {
    let my_uuid = format!("{}@testing.testing", Uuid::new_v4());
    assert!(set_unsubscribed(my_uuid.clone()));
}

#[test]
fn is_unsubscribed_test() {
    let my_uuid = format!("{}@testing.testing", Uuid::new_v4());
    assert!(!is_unsubscribed(my_uuid.clone()));
    set_unsubscribed(my_uuid.clone());
    assert!(is_unsubscribed(my_uuid.clone()));
}
