#[derive(Debug, PartialEq)]
pub struct Cli {
    args: Vec<String>,
    flags: Vec<String>,
}

impl Cli {
    pub fn new(cla: impl Iterator<Item = String>) -> Self {
        let cla = cla.skip(1);
        let (flags, args): (Vec<String>, Vec<String>) = cla.partition(|s| s.starts_with('-'));
        Cli { args: args, flags: flags }
    }

    pub fn next_arg(&mut self) -> Option<String> {
        if self.args.is_empty() {
            None
        } else {
            Some(self.args.remove(0))
        }
    }

    pub fn check_flag(&self, flag: &str) -> bool {
        self.flags.iter().find(|&f| f == flag).is_some()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_no_args() {
        let cla = vec!["kvstore"].into_iter().map(|s| s.to_string());
        let cli = Cli::new(cla);
        assert_eq!(cli, Cli {
            args: vec![],
            flags: vec![],
        })
    }

    #[test]
    fn new_args() {
        let cla = vec!["kvstore", "hello", "world"].into_iter().map(|s| s.to_string());
        let cli = Cli::new(cla);
        assert_eq!(cli, Cli {
            args: vec![String::from("hello"), String::from("world")],
            flags: vec![],
        })
    }

    #[test]
    fn new_args_and_flags() {
        let cla = vec!["kvstore", "hello", "world", "--init"].into_iter().map(|s| s.to_string());
        let cli = Cli::new(cla);
        assert_eq!(cli, Cli {
            args: vec![String::from("hello"), String::from("world")],
            flags: vec![String::from("--init")],
        })
    }

    #[test]
    fn next_arg() {
        let mut cli = Cli {
            args: vec![String::from("hello"), String::from("world")],
            flags: vec![String::from("--init")],
        };
        assert_eq!(cli.next_arg(), Some("hello".to_string()));
        assert_eq!(cli.next_arg(), Some("world".to_string()));
        assert_eq!(cli.next_arg(), None);
    }

    #[test]
    fn check_flag() {
        let mut cli = Cli {
            args: vec![String::from("hello"), String::from("world")],
            flags: vec![String::from("--init")],
        };
        assert_eq!(cli.check_flag("--init"), true);
        assert_eq!(cli.check_flag("--help"), false);
    }
}