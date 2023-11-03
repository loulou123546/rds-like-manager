mod mysql;
mod postgresql;
use std::env;
use dotenv::dotenv;
use passwords::PasswordGenerator;

fn main() {
    match dotenv() {
        Ok(_) => println!("Loaded connections variables from .env : OK"),
        Err(_) => eprintln!("Warning: Cannot find .env")
    };
    let args: Vec<String> = env::args().collect();
    let mut mariadb = false;
    let mut postgres = false;
    let mut appname = String::from("");
    
    if args.len() < 2 {
        eprintln!("Usage: rds-manager [--mariadb --postgres] application_name");
    }
    for arg in args {
        if arg == "--mariadb" {
            mariadb = true;
        }
        else if arg == "--postgres" {
            postgres = true;
        }
        else {
            appname = arg;
        }
    }
    let user = format!("app_{}", &appname);
    let pg = PasswordGenerator {
        length: 32,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let pass = pg.generate_one().expect("Failed to generate a new password");

    if mariadb {
        println!("Creating user '{}' with password '{}' on mariadb ...", &user, &pass);

        let pool = mysql::init().expect("Cannot start connexion with mariadb server");
        if !mysql::create_user(&pool, &user, &pass).expect("Cannot create user") {
            return;
        }
        println!("User created, now setting up database ...");
        if !mysql::create_database_and_grant(&pool, &user, &appname).expect("Cannot create database or grant options") {
            return;
        }
        println!("mariadb Database created and permissions granted to your user. Ready to go!");
    }
    if postgres {
        let user = appname.clone(); // In postgres, user have same name as database they own
        println!("Creating user '{}' with password '{}' on postgres ...", &user, &pass);

        let mut client = postgresql::init().expect("Cannot start connexion with postgresql server");
        if !postgresql::create_user(&mut client, &user, &pass).expect("Cannot create user") {
            return;
        }
        println!("User created, now setting up database ...");
        if !postgresql::create_database_and_grant(&mut client, &user, &appname).expect("Cannot create database or grant options") {
            return;
        }
        println!("postgres Database created and permissions granted to your user. Ready to go!");
    }
}
