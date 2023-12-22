use std::fs;
use std::i8;
use std::collections::HashMap;



fn main() {
    let good: HashMap<u32, &str> = HashMap::new();
    let file = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines: Vec<&str> = file.split("\n").collect();
    
    let mut tilted_board: Vec<Vec<char>> = vec![vec!['.'; 100]; 100];

    
    // convert lines (&str) into Vec<String>
    let rows: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    
    let mut cols: Vec<String> = Vec::new();
    cols.resize_with(rows[0].len(), String::new);


    // take the iterator of lines and transform it into an iterator of the chars in the line
    for row in rows.iter().map(|row| row.chars()) {
        // loop through each char in the line
        for i in 0..rows[0].len() {
            cols[i].push(
                row.clone()
                .nth(i)
                .expect("Could not unpack character from row!"),
                );
        }
    }

    // for e in &rows {
    //   println!("{}", e);
    //}

    println!("Columns: {}", cols.len());
    println!("Rows: {}", rows.len());

    

    let mut i: usize = 0;
    for column in &cols {
        let mut boulder_indexes: Vec<usize> = vec![];
        let mut barrier_indexes: Vec<usize> = vec![];
        for (ci, c) in column.chars().enumerate() {
            match c {
                'O' => boulder_indexes.push(ci),
                '#' => barrier_indexes.push(ci),
                _ => continue,
            }
        }
        // create the shifted column
        let mut j: usize = 0;
        for barrier_i in &barrier_indexes {
            for boulder_i in &boulder_indexes {
                if boulder_i < barrier_i {
                    // move the boulder to the top
                    tilted_board[i][j] = 'O';
                    // remove this boulder from the index list
                    j += 1;
                } else {
                    // all boulders inside the barrier have moved
                    // update placement offset for next boulder
                    j = barrier_i + 1;
                    // place the current barrier in the tilted board
                    tilted_board[i][*barrier_i] = '#';
                    break;
                }
            }
        }
        for colddd in tilted_board[i].iter() {
            println!("{}", colddd);
        }
    }



}
