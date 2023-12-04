use std::io::{BufReader, BufRead};
use std::fs::File;
use std::io;
use std::task::Wake;




fn load_file_lines(filepath :&str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(filepath).unwrap();

    let reader = BufReader::new(file);

    reader.lines()
}



fn main() {
    
    const MAX_RED_CUBES: i32 = 12;
    const MAX_GREEN_CUBES: i32 = 13;
    const MAX_BLUE_CUBES: i32 = 14;

    let colors = vec!([
        "red",
        "blue",
        "green",
    ]);
    let mut game_index = 1;
    let mut answer = 0;
    let lines = load_file_lines("src/input.txt");
    let mut occurance = None;
    let mut index = 0;
    let mut color_index = 0;


    let mut power = 1;

    for l in lines {
        let mut minimum_red = 0;
        let mut minimum_green = 0;
        let mut minimum_blue = 0;
        let mut valid_game = true;
        let mut line =  l.expect("There is no line.");  
        while line.find("red").is_some() {

            occurance = line.find("red");
            match occurance {
                Some(e) => index = occurance.unwrap(),
                None => continue
            }
            let num_str = &line[(index-3)..index];
            let num_str = &num_str.replace(" ", "");
            // println!("{num_str}");
            let number_real = num_str.parse::<i32>().expect("Cannot convert to number");

            // part 2
            if number_real > minimum_red {
                minimum_red = number_real;
            }


            if number_real > MAX_RED_CUBES {
                valid_game = false;
            }
            // remove the string from the line
            line = line.replacen("red", "", 1);
            // println!("{line}");
        }
        
        while line.find("green").is_some() {

            occurance = line.find("green");
            match occurance {
                Some(e) => index = occurance.unwrap(),
                None => continue
            }
            let num_str = &line[(index-3)..index];
            let num_str = &num_str.replace(" ", "");
            // println!("{num_str}");
            let number_real = num_str.parse::<i32>().expect("Cannot convert to number");


            if number_real > minimum_green {
                minimum_green = number_real;
            }

            if number_real > MAX_GREEN_CUBES {
                valid_game = false;
            }
            // remove the string from the line
            line = line.replacen("green", "", 1);
            // println!("{line}");
        }

        while line.find("blue").is_some() {

            occurance = line.find("blue");
            match occurance {
                Some(e) => index = occurance.unwrap(),
                None => continue
            }
            let num_str = &line[(index-3)..index];
            let num_str = &num_str.replace(" ", "");
            // println!("{num_str}");
            let number_real = num_str.parse::<i32>().expect("Cannot convert to number");


            if number_real > minimum_blue {
                minimum_blue = number_real;
            }

            if number_real > MAX_BLUE_CUBES {
                valid_game = false;
            }
            // remove the string from the line
            line = line.replacen("blue", "", 1);
            // println!("{line}");
        }


        power = minimum_red * minimum_green * minimum_blue;
        println!("{power}");
        answer += power;
        // if valid_game {
            // answer += game_index;
           // println!("{game_index}")
       // }         
        game_index += 1;


    }

    println!("The answer is {answer}");


}



// for red, blue, green
// get index of color in the string
// for each index, go two chars to the left
// read the number going left until there is a space
// check if number is greater than max number
