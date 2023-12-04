use std::{io, fs};
use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use std::fs::File;



fn main() {
    
    let symbols = vec!['$', '*', '/', '%', '#', '@'];


    let input = fs::read_to_string("src/input.txt").expect("Could not read the file");
    let total_chars = input.len();
    for i in 0..total_chars {
        let c = input.chars().nth(i).expect("There was no char at this index"); 
        if symbols.contains(&c) {
            if symbols.contains(&input.chars().nth(i+1).expect("") {
                // build the number
                while !symbols.contains(input.chars().nth(j)) {

                }
            }
        }
    }




}



// for each char in string
// SYMBOL ON LEFT SIDE
// if c is a symbol
//    if the next c is a number
//       build the number until there is no more numbers
//       if c ends with a symbol, valid number, add to total
//          Then this number might be double counted: &3423@
//    else continue (symbol surrounded by periods)
// SYMBOL ON RIGHT SIDE
// if c is a number
//    build the number until there is no more numbers
//    if c ends with a symbol, valid number, add to total
