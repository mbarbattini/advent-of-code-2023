use std::fs;
use std::collections::HashMap;


struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    sum_numbers: i32,
}

impl Part {
    fn new(x: i32, m: i32, a: i32, s:i32) -> Self {
        Self {x, m, a, s, sum_numbers: 0}
    }
}


fn solve_part_2(input: String) -> Result<(), String> {

    let (workflows, _) = input.split_once("\r\n\r\n").unwrap();

    let mut instructions_list: Vec<String> = vec![];

    // create a vector of workflows
    for (index, l) in workflows.lines().enumerate() {

        let Some((name, instructions)) = l.split_once("{") else {
            return Err(String::from("Could not split line."));  
        };

        let instructions_stripped = instructions.replace("}", "");
        
        instructions_list.push(instructions_stripped);
    }


    // initalize to max range
    let mut max_allowed_x: i64 = 10000;
    let mut min_allowed_x: i64 = -1;
    let mut max_allowed_m: i64 = 10000;
    let mut min_allowed_m: i64 = -1;
    let mut max_allowed_a: i64 = 10000;
    let mut min_allowed_a: i64 = -1;
    let mut max_allowed_s: i64 = 10000;
    let mut min_allowed_s: i64 = -1;

    for work in instructions_list {
        let steps: Vec<&str> = work.split(',').collect();
        //println!("{}", work);
        for s in steps {
            // check if the step is a comparison. Non-comparisons will not have a ':'
            if s.contains(':') {
                let letter = &s[0..1];
                let symbol = &s[1..2];
                let number = &s[2..].split(":").collect::<Vec<_>>()[0].parse::<i64>().unwrap();


                match (letter, symbol) {
                    ("x", ">")  => { if number > &min_allowed_x { min_allowed_x = number + 1; } println!("{}", min_allowed_x);},
                    ("x", "<")  => { if number < &max_allowed_x { max_allowed_x = number - 1; }/* println!("{}", max_allowed_x);*/},
                    ("m", ">")  => { if number > &min_allowed_m { min_allowed_m = number + 1; }},
                    ("m", "<")  => { if number < &max_allowed_m { max_allowed_m = number - 1; }},
                    ("a", ">")  => { if number > &min_allowed_a { min_allowed_a = number + 1; }},
                    ("a", "<")  => { if number < &max_allowed_a { max_allowed_a = number - 1; }},
                    ("s", ">")  => { if number > &min_allowed_s { min_allowed_s = number + 1; }},
                    ("s", "<")  => { if number < &max_allowed_s { max_allowed_s = number - 1; }},
                    _ => continue, // move on to the next step in this instruction
                }
            }
        }
 
    }
    let x_range: i64 = min_allowed_x - max_allowed_x;
    let m_range: i64 = min_allowed_m - max_allowed_m;
    let a_range: i64 = min_allowed_a - max_allowed_a;
    let s_range: i64 = min_allowed_s - max_allowed_s;

    println!("x: {}", x_range);
    println!("m: {}", m_range);
    println!("a: {}", a_range);
    println!("s: {}", s_range);

    let combinations: i64 = x_range * m_range * a_range * s_range;
    
    println!("The answer is {}", combinations / 4);

    Ok(())
}



fn solve_part_1(input: String) -> Result<(), String>{

    let (workflows, parts) = input.split_once("\r\n\r\n").unwrap();

    let mut names_to_index: HashMap<&str, u32> = HashMap::new();
    
    let mut instructions_list: Vec<String> = vec![];

    let mut parts_list: Vec<Part> = vec![];

    // create a vector of workflows
    for (index, l) in workflows.lines().enumerate() {

        let Some((name, instructions)) = l.split_once("{") else {
            return Err(String::from("Could not split line."));  
        };

        let instructions_stripped = instructions.replace("}", "");
        
        instructions_list.push(instructions_stripped);


        // insert map of workflow name to index 
        names_to_index.insert(name, index as u32);
    }


    // create a vector of parts
    for (index, l) in parts.lines().enumerate() {
        let part_stripped = l.replace("{", "").replace("}", "");

        let mut part: Part = Part::new(0, 0, 0, 0);


        let ratings: Vec<i32> = part_stripped
            .split(',')
            .map(|s| s[2..].parse().unwrap())
            .collect();

        part.x = ratings[0];
        part.m = ratings[1];
        part.a = ratings[2];
        part.s = ratings[3];
        part.sum_numbers = ratings[0] + ratings[1] + ratings[2] + ratings[3];
        parts_list.push(part);
    }
    
    let mut total: i32 = 0;
    // start algorithm. loop through parts
    let current_instruction_index = *names_to_index.get("in").expect("Could not find 'in' instruction") as usize;
    for p in parts_list {
        // find the 'in' instruction
        print!("\nin ");
        total += process_instructions(&p, &current_instruction_index, &names_to_index, &instructions_list);
    }

    println!("\nThe answer is {}", total);
    
    Ok(())
}


fn main() -> Result<(), String> {

    let input = fs::read_to_string(r"C:\Users\H457071\Desktop\AdventOfCode2023\day_19\src\input.txt").expect("Could not open the file.");
    //let input = fs::read_to_string(r"C:\Users\H457071\Desktop\AdventOfCode2023\day_19\src\test_input.txt").expect("Could not open the file.");

    //solve_part_1(input);
    solve_part_2(input);

    Ok(())
}



fn process_instructions(part: &Part, instruction_index: &usize, names_to_index: &HashMap<&str, u32>, instructions_list: &Vec<String>) -> i32 {
    let current_instruction = &instructions_list[*instruction_index];
    let steps: Vec<&str> = current_instruction.split(',').collect();
    for s in steps {
        // check if the step is a comparison. Non-comparisons will not have a ':'
        if s.contains(':') {
            let letter = &s[0..1];
            let symbol = &s[1..2];
            let number = &s[2..].split(":").collect::<Vec<_>>()[0];

            // check
            let mut move_next: bool = false;
            match (letter, symbol) {
                ("x", ">")  => move_next = part.x > number.parse::<i32>().unwrap(),
                ("x", "<")  => move_next = part.x < number.parse::<i32>().unwrap(),
                ("m", ">")  => move_next = part.m > number.parse::<i32>().unwrap(),
                ("m", "<")  => move_next = part.m < number.parse::<i32>().unwrap(),
                ("a", ">")  => move_next = part.a > number.parse::<i32>().unwrap(),
                ("a", "<")  => move_next = part.a < number.parse::<i32>().unwrap(),
                ("s", ">")  => move_next = part.s > number.parse::<i32>().unwrap(),
                ("s", "<")  => move_next = part.s < number.parse::<i32>().unwrap(),
                _ => continue, // move on to the next step in this instruction
            }
            // condition was true, so either A R or move on 
            if move_next {
                let next_move_string = s.split(":").collect::<Vec<_>>()[1];
                match next_move_string {
                    // rejected in one of the comparison steps, can immediately move on to next
                    // part
                    "R" => {print!("-> Rejected"); break},
                    "A" => {
                        // Accepted
                        print!(": Accepted with score of {}", part.sum_numbers);
                        return part.sum_numbers
                    },
                    // move on to another workflow. The workflow is "next_move_string"
                    _ => {
                        print!("-> {} ", next_move_string);
                        let next_instruction_index = *names_to_index.get(next_move_string).unwrap() as usize;
                        return process_instructions(part, &next_instruction_index, names_to_index, instructions_list)
                    }
                }
            }
        } else {
            // we are at the last step in the instruction. Has to be these choices
            match s {
                // can do nothing because we are at the end
                "R" => print!("-> Rejected"),
                "A" => {
                    // Accepted
                    print!(": Accepted with score of {}", part.sum_numbers);
                    return part.sum_numbers
                },
                // move on to another workflow
                _ => {
                    print!("-> {} ", s);
                    let next_instruction_index = *names_to_index.get(s).expect("Could not find the next instruction") as usize;
                    return process_instructions(part, &next_instruction_index, names_to_index, instructions_list)
                }
            }
        }
        // move on to the next instruction
    }
    // somehow got to the end with no steps parsed, Rust needs a return in this case, which
    // shouldn't happen
    return 0
}
