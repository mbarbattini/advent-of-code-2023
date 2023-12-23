use std::fs;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use queues::*;


#[derive(Eq, PartialEq, Hash, Clone)]
struct Cell {
    x: u64,
    y: u64,
    g_score: i64,
    f_score: i64
}

impl Cell {
    pub fn new(x: u64, y: u64) -> Self {
        Self {
            x, 
            y, 
            g_score: 0, 
            f_score: 0,
        } 
    }
}


fn main() {
    let filename: &str = "src/input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file");
    
    let mut grid: Vec<Vec<u64>> = vec![vec![0; 141]; 141];

    
    let lines = file.lines();
    
    let mut j: usize = 0;
    let mut i: usize;

    for l in lines {
        i = 0;
        for c in l.chars() {
            grid[i][j] = c.to_digit(10).expect("Could not convert to int.") as u64;
            i += 1;
        }
        j += 1;
    }

    // for row in grid.iter() {
    //    for entry in row.iter() {
    //        print!("{}", entry);
    //    }
    //    print!("\n");
    // }


    let mut start: Cell = Cell::new(0, 0);
    
    let mut open_set: Queue<Cell> = Queue::new();    

    open_set.add(start).expect("Could not add to the queue.");

    open_set.peek();


}




