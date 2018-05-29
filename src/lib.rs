use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::io;
use std::path::Path;
use std::collections::HashMap;


pub struct Config {
    extensions: HashMap<String, u32>,
    pattern: String,
    path: String,
}

impl Config {
    pub fn new (mut args: std::env::Args) -> Result<Config, & 'static str> {
        let mut extensions = HashMap::new();
        args.next();
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err ("You didn't specify a path in which a file should be found")

        };
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err ("You didn't specify a pattern to be found."),
        };
        match args.next() {
            Some(arg) => extensions.insert(arg, 1),
            None => return Err ("You didn't specify any extension in which a pattern is searched."),
        };
        loop  {
            match args.next() {
                Some(arg) => {extensions.insert(arg, 1);}
                None => {break;}
            };
        }
        Ok(Config {extensions, pattern, path})
    }
}

pub fn run (config: Config) -> Result <(), io::Error> {
    let path = path::Path::new(&config.path);
    if let Err(e) = fs::read_dir(path){  
        return Err(io::Error::new(io::ErrorKind::Other, "You specified wrong directory name"));
    }
    process_directory (&config, fs::read_dir(path)?)?;
    Ok(())
}

fn process_directory (config: &Config, paths: fs::ReadDir) -> Result<(), io::Error> {
    for path in paths {
        let currPath = path.unwrap().path();
        if currPath.is_dir() {
            process_directory (config, fs::read_dir(currPath.as_path())?);
        } else if is_correct_extension (config, &currPath.as_path()) {
            if let Ok(true) = find_pattern(&config.pattern, currPath.as_path()){
                println!("{:?}", currPath);
            }
        }

    }
    Ok(())

}

fn find_pattern(pattern: &String, path: &Path) -> Result<bool, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    for line in content.lines() {
        if line.contains(pattern) {
            return Ok(true);
        }
    }

    Ok(false)

}

fn is_correct_extension (config: &Config, path: &Path) -> bool
{
    if let Some(arg) = path.extension() {
        return config.extensions.contains_key(arg.to_str().unwrap())
    }
    false
}