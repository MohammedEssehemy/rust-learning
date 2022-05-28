use std::{
    env, 
    error::Error,
    fs,
};
use regex::Regex;


#[derive(Debug)]
struct Config {
    folder: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2  {
            return Err("arg folder is required");
        }
        let folder = args[1].clone();
        Ok(Config { folder })
    }
}

fn main() -> Result<(),  Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args)?;
    println!("Hello, world!, {:?}", config);
    run(config)?;
    Ok(())
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let numbers_regex = Regex::new(r"(?P<before>.+)- (?P<num>\d+) -(?P<after>.+)").unwrap();
    fs::read_dir(&config.folder)?
    .for_each(|entry_result| {
        match entry_result {
            Err(_) => (),
            Ok(entry) => {
                let original_name = entry.file_name();
                let new_name = numbers_regex
                .replace(original_name.to_str().unwrap(), "$num - $before $after");
                println!("{}", new_name);
                fs::rename(entry.path(), format!("{}/{}", config.folder, new_name)).unwrap();
            }
        }
    });
    Ok(())
}