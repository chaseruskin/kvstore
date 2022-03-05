pub mod database;
pub mod cli;
pub mod kvstore;

pub const VERSION: &str = "kvstore 0.1.0";

pub const USAGE: &str = "\
kvstore is a key-value keeper.

Usage:
    kvstore [<key>] [<value>] [flags]

Args:
    <key>       label to identify data
    <value>     data to store behind a label

Flags:
    --init      list pairs in exporting environment format to stdout 
    --append    add additional <value> to existing value behind given <key>
    --version   print the current version
    --help      print help information

More:
    Enter only a <key> to view its value. To view all values, pass '.'.

    kvstore's database is a 'kv.db' file located where the program is ran
    unless the environment variable KVSTORE_HOME is set to an existing 
    directory.";