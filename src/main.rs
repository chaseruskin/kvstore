//! Crate    : kvstore
//! Author   : Chase Ruskin
//! File     : main.rs
//! Abstract : 
//!     Entry-point to `kvstore` command-line tool. The main process follows
//!         1. reads env and accept arguments, 
//!         2. loads database from a file, 
//!         3. interacts with database
//!         4. Saves any necessary changes to database.

use kvstore::database::*;
use std::env;
use kvstore::cli::*;

fn main() {
    let mut cli = Cli::new(env::args());
    let root = env::var("KVSTORE_HOME").unwrap_or(".".to_owned());
    let mut db = match Database::new(&(root+"/kv.db")) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("kv-error: {}", e);
            return;
        }
    };

    let key = if let Some(k) = cli.next_arg() {
        k
    } else {
        println!("{}", USAGE);
        return;
    };

    match cli.next_arg() {
        Some(value) => db.edit(&key, &value),
        None => {
            // print value for corresponding key
            if let Some(v) = db.view(&key) {
                println!("{}", v);
            // print all key-values
            } else if key == "."  {
                let max = db.get_keys()
                    .fold(0, |max, k| {
                        if k.len() > max { k.len() } else { max }
                });
                db.get_keys().for_each(|k| {
                    let v = db.view(&k).unwrap();
                    if v.is_empty() == false {
                        print!("{}", &k);
                        print!("{:<1$}", "", max-k.len()+4);
                        println!("{}", v);
                    }
                });
            }
            // key does not exist- print empty line
            else {
                println!("");
                return;
            };
            return;
        }
    };

    if let Err(e) = db.save() {
        eprintln!("kv-error: {}", e);
    } else {
        println!("kv-info: Save successful")
    }
}

const USAGE: &str = "\
kvstore is a key-value keeper.

Usage:
    kvstore [<key>] [<value>]

Args:
    <key>       label to identify data
    <value>     data to store behind a label

More:
    Enter only a <key> to view its value. To view all values, pass '.'.

    kvstore's database is a 'kv.db' file located where the program is ran
    unless the environment variable KVSTORE_HOME is set to an existing 
    directory.";