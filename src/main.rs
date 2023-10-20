use std::{env};
use regex::Regex;

fn main() {
    let mut deconstructed_names = vec!();
    for arg in env::args().skip(1).take(2){
        if !arg.contains("who_are_you"){
            deconstructed_names.push(deconstruct_name(arg, r"[']", r"[,\- ]"));
        }
    }
    
    println!("{:?}", deconstructed_names);
}

fn deconstruct_name(s: String, remove: &str, split_on: &str) -> Vec<String>{
    regex_split(regex_replace(&s,remove), split_on)
}

fn regex_split(s: String, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.split(&s).into_iter()
    .map(|x| String::from(x))
    .filter(|x| !String::is_empty(x)).collect()
}

fn regex_replace(s: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    re.replace(s, "").to_string()
}