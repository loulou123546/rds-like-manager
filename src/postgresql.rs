use std::{env, error::Error};

use postgres::{Client, NoTls};


fn check_valid_chars (car: char) -> bool {
    char::is_ascii_lowercase(&car) || char::is_ascii_digit(&car) || car == '_'
}
fn check_valid_password (car: char) -> bool {
    char::is_ascii_alphanumeric(&car) || car == '_'
}


pub fn init () -> Result<Client, Box<dyn Error>> {
    let url = env::var("POSTGRES_ACCESS")?;
    Ok(Client::connect(&url, NoTls)?)
}


pub fn create_user (client: &mut Client, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    if !username.chars().all(check_valid_chars) {
        eprintln!("Username contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }
    if !password.chars().all(check_valid_password) {
        eprintln!("Password contain invalid characters");
        return Ok(false);
    }

    client.execute(
        &format!("CREATE ROLE {} LOGIN PASSWORD '{}';", username, password),
        &[]
    )?;
    Ok(true)
}

pub fn create_database_and_grant (client: &mut Client, username: &str, application: &str) -> Result<bool, Box<dyn Error>> {
    if !username.chars().all(check_valid_chars) {
        eprintln!("Username contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }
    if !application.chars().all(check_valid_chars) {
        eprintln!("Application name contain invalid characters. Only support a-z0-9 and _");
        return Ok(false);
    }

    client.execute(
        &format!("CREATE DATABASE {} WITH OWNER = '{}' ENCODING = 'UTF8';", application, username),
        &[]
    )?;
    client.execute(
        &format!("GRANT ALL ON DATABASE {} TO {}", application, username),
        &[]
    )?;
    Ok(true)
}

