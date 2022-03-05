pub mod database;
pub mod cli;

use std::path::PathBuf;

struct KvStore {
    key: Option<String>,
    value: Option<String>,
    init: bool,
    home: Option<PathBuf>,
}

fn run(kv: KvStore) {
    
}