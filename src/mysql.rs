use std::{env, error::Error};

use mysql::*;
use mysql::prelude::*;


fn check_valid_chars (car: char) -> bool {
    char::is_ascii_lowercase(&car) || char::is_ascii_digit(&car) || car == '_'
}
fn check_valid_password (car: char) -> bool {
    char::is_ascii_alphanumeric(&car) || car == '_'
}


pub fn init () -> Option<Pool> {
    let url = env::var("MARIADB_ACCESS").ok()?;
    let pool = Pool::new(Opts::from_url(&url).ok()?).ok();
    return pool;
}

pub fn create_user (pool: &Pool, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    if !username.chars().all(check_valid_chars) {
        eprintln!("Username contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }
    if !password.chars().all(check_valid_password) {
        eprintln!("Password contain invalid characters");
        return Ok(false);
    }
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        format!("CREATE USER IF NOT EXISTS '{}'@'%' IDENTIFIED BY '{}';", username, password)
    )?;
    Ok(true)
}

pub fn create_database_and_grant (pool: &Pool, username: &str, application: &str) -> Result<bool, Box<dyn Error>> {
    if !username.chars().all(check_valid_chars) {
        eprintln!("Username contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }
    if !application.chars().all(check_valid_chars) {
        eprintln!("Application name contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", application)
    )?;
    let grants = "SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, INDEX, ALTER, EVENT, TRIGGER, CREATE VIEW, SHOW VIEW, LOCK TABLES, CREATE TEMPORARY TABLES";
    conn.query_drop(
        format!("GRANT {} ON `{}`.* TO '{}'@'%';", grants, application, username)
    )?;
    conn.query_drop(
        "FLUSH PRIVILEGES;"
    )?;
    Ok(true)
}
