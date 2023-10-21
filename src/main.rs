use std::{env, fs};
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use soundex;

#[derive(Debug)]
struct PersonName {
    provided_name: String,
    name_parts: Vec<String>,
    initials: Vec<char>,
    name_sounds: Vec<String>
}

impl PersonName {
    pub fn new(personName: String) -> PersonName {
        let deconstructed_name = PersonName::standard_deconstruct(&personName);
        PersonName{
            initials : PersonName::generate_initials(&deconstructed_name),
            name_sounds: PersonName::generate_soundex(&deconstructed_name),
            provided_name: personName,
            name_parts : deconstructed_name,
        }
    }

    pub fn generate_initials(names: &Vec<String>) -> Vec<char>{
        names.iter().map(|x| x.chars().next().unwrap()).collect()
    }

    pub fn generate_soundex(name_parts: &Vec<String>) -> Vec<String> {
        name_parts.into_iter().map(|x| soundex::american_soundex(x)).collect()
    }

    pub fn standard_deconstruct(s: &String) -> Vec<String>{
        PersonName::deconstruct_name(s, r"['.]", r"[,\- ]")
    }

    pub fn deconstruct_name(s: &String, remove: &str, split_on: &str) -> Vec<String>{
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
            let mut new_person = PersonName::new(arg);
            deconstructed_names.push(new_person);
        }
    }

    println!("{:#?}", deconstructed_names);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn Deconstruct() {
        assert_eq!(PersonName::standard_deconstruct(&"Benjamin David Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
        assert_eq!(PersonName::standard_deconstruct(&"Benjamin D'avid Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
        assert_eq!(PersonName::standard_deconstruct(&"Benjamin, D'avid, Pinter".to_string()), vec!["Benjamin", "David", "Pinter"]);
    }

    #[test]
    pub fn Initialism() {
        let new_person = PersonName::new("Benjamin David Pinter".to_string());
        assert_eq!(PersonName::generate_initials(&new_person.name_parts), vec!['B', 'D', 'P']);

        let new_person2 = PersonName::new("Mary Meg Patrick".to_string());
        assert_eq!(PersonName::generate_initials(&new_person2.name_parts), vec!['M', 'M', 'P']);
    }
}