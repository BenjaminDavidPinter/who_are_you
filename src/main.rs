use std::{env, fs};
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

#[derive(Debug)]
struct PersonName {
    name_parts: Vec<String>,
    initials: Vec<char>
}

impl PersonName {
    pub fn new(deconstructed_name: Vec<String>) -> PersonName {
        PersonName{
            name_parts : deconstructed_name,
            initials : vec![]
        }
    }

    pub fn generate_initials(name: &PersonName) -> Vec<char>{
        name.name_parts.iter().map(|x| x.chars().next().unwrap()).collect()
    }

    pub fn standard_deconstruct(s: String) -> Vec<String>{
        PersonName::deconstruct_name(s, r"['.]", r"[,\- ]")
    }

    pub fn deconstruct_name(s: String, remove: &str, split_on: &str) -> Vec<String>{
        PersonName::regex_split(PersonName::regex_replace(&s,remove), split_on)
    }

    pub fn regex_split(s: String, pattern: &str) -> Vec<String> {
        let re = Regex::new(pattern).unwrap();
        re.split(&s).into_iter().map(|x| String::from(x)).filter(|x| !String::is_empty(x)).collect()
    }

    pub fn regex_replace(s: &str, pattern: &str) -> String {
        let re = Regex::new(pattern).unwrap();
        re.replace(s, "").to_string()
    }
}

fn main() {
    let mut deconstructed_names = vec!();
    for arg in env::args().skip(1).take(2){
        if !arg.contains("who_are_you"){
            let mut new_person = PersonName::new(PersonName::standard_deconstruct(arg));
            new_person.initials = PersonName::generate_initials(&new_person);
            deconstructed_names.push(new_person);
        }
    }

    println!("{:?}", deconstructed_names);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn Deconstruct() {
        assert_eq!(PersonName::standard_deconstruct("Benjamin David Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
        assert_eq!(PersonName::standard_deconstruct("Benjamin D'avid Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
        assert_eq!(PersonName::standard_deconstruct("Benjamin, D'avid, Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
    }

    #[test]
    pub fn Initialism() {
        let new_person = PersonName::new(PersonName::standard_deconstruct("Benjamin David Pinter".to_string()));
        assert_eq!(PersonName::generate_initials(&new_person), vec!['B', 'D', 'P']);

        let new_person = PersonName::new(PersonName::standard_deconstruct("Rocco Sucks BigTime".to_string()));
        assert_eq!(PersonName::generate_initials(&new_person), vec!['R', 'S', 'B']);
    }
}