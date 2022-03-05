//! Crate    : kvstore
//! Author   : Chase Ruskin
//! File     : main.rs
//! Abstract : 
//!     Entry-point to `kvstore` command-line tool. The main process follows
//!         1. reads env and accept arguments, 
//!         2. loads database from a file, 
//!         3. interacts with database
//!         4. Saves any necessary changes to database.

use kvstore::cli::Cli;
use kvstore::kvstore::KvStore;

fn main() {
    let cli = Cli::new(std::env::args());
    match KvStore::new(cli) {
        Ok(mut kv) => match kv.run() {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("kv-error: {}", e),
        },
        Err(e) => eprintln!("kv-error: {}", e)
    };
}