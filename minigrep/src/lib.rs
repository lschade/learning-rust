use std::{collections::HashMap, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results;
    if config.case_sensitive {
        results = search(&config.query, &contents)
    } else {
        results = search_case_insensitive(&config.query, &contents)
    }

    results.iter().for_each(|r| println!("{}", r));
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = match args.next() {
            Some(v) => v,
            None => return Err("query missing")
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("Filename missing")
        };

        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})        
    }
}


struct Cacher<T>
where T: Fn(u32) -> u32 {
    function: T,
    values: HashMap<u32, u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(function: T) -> Cacher<T> {
        Cacher {
            function,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            None => {
                let val = (self.function)(arg);
                self.values.insert(arg, val);
                val
            },
            Some(v) => *v
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::Cacher;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], crate::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            crate::search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_cache() {
        let function = |arg| arg;

        let mut cache = Cacher::new(function);

        let val1 = cache.value(1);
        let val2 = cache.value(2);

        assert_eq!(val1, 1);
        assert_eq!(val2, 2);
    }
}

