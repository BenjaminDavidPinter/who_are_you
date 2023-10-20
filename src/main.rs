use std::{env};

struct HumanName {
    FirstNames: Vec<String>,
    MiddleNames: Vec<String>,
    LastNames: Vec<String>
}

fn main() {
    let mut deconstructedNames = vec!();
    let programArgs = env::args();
    for arg in programArgs{
        if !arg.contains("who_are_you"){
            deconstructedNames.push(deconstruct_name(arg));
        }
    }
    
    println!("{:?}", deconstructedNames);
}

fn deconstruct_name(s: String) -> Vec<String>{
    let mut deconstructed_name: Vec<String> = vec![];
    let split_input = s.split_whitespace();
    for namePart in split_input {
        let mut commaSplitName = namePart.split(',');
        if let Some(smallerPart) = commaSplitName.next() {
            deconstructed_name.push(String::from(smallerPart));
            for smallerPart in commaSplitName {
                if !str::is_empty(smallerPart) {
                    deconstructed_name.push(String::from(smallerPart));    
                }
            }
        }
        else {
            deconstructed_name.push(String::from(namePart));
        }
        
    }
    
    deconstructed_name
}
