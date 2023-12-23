use std::io;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::any::type_name;



// TODO
//
// dmhkvgbc6four6eightwofkk
//
// CORRECT:
// dmhkvgbc64682kk = 62
// WRONG (Mine):
// dmhkvgbc6468wofkk = 68
// Can't replace the spelled letter immediately because it overwrites the addition of another
// spelled number

fn main() -> io::Result<()> {
    let testing = String::from("a b c d e f g h i j k l m n o p q r z t u v w x y z");

    let split_string = testing.split(' ').collect::<Vec<&str>>();
    for word in split_string {
        let gh = type_name;
        println!("{}", gh);
    }


    let spelled_numbers: Vec<&str> = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut number_map = HashMap::new();
    number_map.insert("one", "1");
    number_map.insert("two", "2");
    number_map.insert("three", "3");
    number_map.insert("four", "4");
    number_map.insert("five", "5");
    number_map.insert("six", "6");
    number_map.insert("seven", "7");
    number_map.insert("eight", "8");
    number_map.insert("nine", "9");




    let mut answer: u32 = 0;

    let input_file = File::open("src/input.txt")?;
    
    let reader = BufReader::new(input_file);

    let mut digit_1: Option<u32> = None;
    let mut digit_2: Option<u32> = None;


    for line in reader.lines() {
        // println!("{}", line.unwrap());    
        let line_string = &line.unwrap(); 
        let mut temp_line = String::from("");
        let mut final_string = String::from("");
        // build the line one char at a time
        for c in line_string.chars() {
            temp_line.push(c); 
            // check if any words are formed
            for number in &spelled_numbers {
                if temp_line.contains(number) {
                    temp_line = temp_line.replace(number, number_map[number]);
                    final_string.push_str(number_map[number]);
                }
            }
        }
        println!("{}", final_string);
        // do the calculation
        for c in final_string.chars() { 
            let current_char = c.to_digit(10); 
            match current_char {
                Some(i) => {

                    if digit_1.is_none() {
                        // println!("Made it to digit 1 is none");
                        digit_1 = Some(i);
                    } else {
                       digit_2 = Some(i);
                    }
                },
                None => continue,
            }
        }
        // went through the entire line
        // if only one number was read
        if digit_2.is_none() {
            // println!("Digit 1 is {}", digit_1.unwrap());
            digit_2 = digit_1.clone();
        }
        let secret_code: u32 = 10 * digit_1.unwrap() + digit_2.unwrap();
        answer += secret_code;
        digit_1 = None;
        digit_2 = None;
        // println!("{}", secret_code);
        // println!("Sum: {}", answer);
        continue;

    }
    println!("The answer is {answer}");
    //end
    





    Ok(())
}

