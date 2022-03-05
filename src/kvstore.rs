use crate::cli::Cli;
use std::env;
use crate::database::Database;

pub struct KvStore {
    db: Database,
    key: Option<String>,
    value: Option<String>,
    init: bool,
    help: bool,
    version: bool,
    append: bool,
}

impl KvStore {

    pub fn new(mut cli: Cli) -> Result<KvStore, Box<dyn Error>> {
        let root = env::var("KVSTORE_HOME")
            .unwrap_or(".".to_owned());
        let kv = KvStore {
            db: Database::new(&(root+"/kv.db"))?,
            key: cli.next_arg(),
            value: cli.next_arg(),
            init: cli.check_flag("--init"),
            help: cli.check_flag("--help"),
            version: cli.check_flag("--version"),
            append: cli.check_flag("--append"),
        };
        if let Some(a) = cli.next_arg() {
            Err(Box::new(KvError::UnknownArg(a)))
        } else {
            Ok(kv)
        }
    }

    fn set_key_to_var(&self, key: &str) -> String {
        // check if the key does not exist in env var
        let val = self.db.view(key).unwrap();
        match std::env::var(key) {
            // env var does not exist
            Err(_) => {
                format!("{}={} ", key, val)
            }
            // env var already set
            Ok(evar) => {
                // make special appending to PATH env var
                if key == "PATH" {
                    let sep = if cfg!(unix) {':'} 
                        else if cfg!(windows) {';'} 
                        else {','};
                    // remove duplicate paths found in key's value
                    let val = std::env::join_paths(
                        val.split(sep).filter(|&p| {
                            evar.split(sep).find(|&s| s == p).is_none()   
                        }))
                        .unwrap();
                    // no new paths to add
                    if val.is_empty() {
                        "".to_string()
                    // there are new paths to add
                    } else {
                        format!("{}={}{}{} ", key, evar, sep, val.to_str().unwrap())
                    }
                } else {
                    "".to_string()
                }
            }
        }
    }

    fn boot_env(&self) -> Result<String, Box<dyn Error>> {
        let mut iter = self.db.get_keys();
        let mut result = String::new();
        while let Some(key) = iter.next() {
            result += &self.set_key_to_var(key);
        }
        Ok(result)
    }

    pub fn run(&mut self) -> Result<String, Box<dyn Error>> {
        if self.version == true {
            return Ok(format!("{}", crate::VERSION));
        }
        
        if self.help == true || (self.key.is_none() && self.init == false) {
            return Ok(format!("{}", crate::USAGE));
        } else if self.init == true {
            return self.boot_env()
        }

        let key = self.key.as_ref().unwrap();
        let result = Ok(match &self.value {
            Some(value) => {
                self.db.edit(&key, &value, self.append);
                self.db.save()?;
                "kv-info: Save successful".to_string()
            }
            None => {
                // print value for corresponding key
                if let Some(v) = self.db.view(&key) {
                    format!("{}", v)
                // print all key-values
                } else if key == "."  {
                    let max = self.db.get_keys()
                        .fold(0, |max, k| {
                            if k.len() > max { k.len() } else { max }
                    });
                    let mut result = String::new();
                    self.db.get_keys().for_each(|k| {
                        let v = self.db.view(&k).unwrap();
                        if v.is_empty() == false {
                            result.push_str(&(k.to_owned() +
                            &format!("{:<1$}", "", max-k.len()+4) +
                            v + "\n"));
                        }
                    });
                    // remove the extra newline if key-values are listed
                    if result.is_empty() == false {
                        result.pop();
                    }
                    result
                }
                // key does not exist- print empty line
                else {
                    "".to_string()
                }
            }
        });

        // initialize environment variables
        if self.init == true {
            self.boot_env()
        } else {
            result
        }
    }
}

use std::error::Error;
use std::fmt::{Debug, Display};

pub enum KvError {
    UnknownArg(String),
}

impl Error for KvError {}

impl Debug for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownArg(arg0) => f.debug_tuple("UnknownArg").field(arg0).finish(),
        }
    }
}

impl Display for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownArg(arg0) => write!(f, "Unknown arg \"{}\"", arg0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_db(name: &str) -> Database {
        let file = env::temp_dir().join(name.to_owned()+".db");
        let mut db = Database::new(file.to_str().unwrap()).unwrap();
        // create new key-value pair
        db.edit("hello", "earth", false);
        db.edit("bonjour", "venus", false);
        db
    }

    #[test]
    fn view_arg() {
        let mut kv = KvStore {
            db: mock_db("view_arg"),
            key: Some("hello".to_owned()),
            value: None,
            init: false,
            help: false,
            version: false,
            append: false,
        };
        assert_eq!(kv.run().unwrap(), "earth".to_owned());

        let mut kv = KvStore {
            key: Some("key_not_found".to_owned()),
            ..kv
        };
        assert_eq!(kv.run().unwrap(), "".to_owned());
    }

    #[test]
    fn disp_help_version() {
        let mut kv = KvStore {
            db: mock_db("disp_help_version"),
            key: None,
            value: None,
            init: false,
            help: true,
            version: false,
            append: false,
        };
        assert_eq!(kv.run().unwrap(), crate::USAGE.to_owned());

        let mut kv = KvStore {
            key: Some("any_key".to_owned()),
            ..kv
        };
        assert_eq!(kv.run().unwrap(), crate::USAGE.to_owned());

        let mut kv = KvStore {
            version: true,
            help: false,
            ..kv
        };
        assert_eq!(kv.run().unwrap(), crate::VERSION.to_owned());
    }

    #[test]
    fn edit_arg() {
        let mut kv = KvStore {
            db: mock_db("edit_arg"),
            key: Some("hello".to_owned()),
            value: Some("world".to_owned()),
            init: false,
            help: false,
            version: false,
            append: false,
        };
        assert_eq!(kv.run().unwrap(), "kv-info: Save successful".to_owned());
    }

    #[test]
    fn edit_and_append_arg() {
        let mut kv = KvStore {
            db: mock_db("edit_and_append_arg"),
            key: Some("hello".to_owned()),
            value: Some(":world".to_owned()),
            init: false,
            help: false,
            version: false,
            append: true,
        };
        kv.run().expect("save failed");
        assert_eq!(kv.db.view("hello").unwrap(), &"earth:world".to_owned());
    }

}