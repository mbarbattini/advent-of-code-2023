use std::vec;
use std::fs;



#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}



impl Pattern {
    fn from(pattern: &str) -> Self {
        let rows: Vec<String> = pattern.lines().map(|line| line.to_string()).collect();

        let mut cols: Vec<String> = Vec::new();
        cols.resize_with(rows[0].len(), String::new);

        for row in rows.iter().map(|row| row.chars()) {
            for i in 0..rows[0].len() {
                cols[i].push(
                    row.clone()
                        .nth(i)
                        .expect("Could not unpack character from row!"),
                );
            }
        }

        Self { rows, cols }
    }
}


fn main() {
    let filename = r"src/input.txt";
    let file = fs::read_to_string(filename).expect("Failed to read the file"); 
        
    let blocks: Vec<Pattern> = file.split("\n\n").map(|pattern| Pattern::from(pattern)).collect();
    for b in blocks {
        println!("{}", b.rows[0]);
    }
        // .map(|pattern| Pattern::from(pattern))
        // .collect();


    // let blocks: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();


}
