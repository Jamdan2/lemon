pub struct Config<'a> {
    pub path: &'a str,
}

impl Config {
    pub fn from(args: Vec<String>) -> Config {
        let path = match args.first() {
            Some(T) => T,
            None => panic!("Path must be specified."),
        };

        Config { path }
    }
}
