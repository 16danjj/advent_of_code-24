
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Data {
    grid: Vec<Vec<char>>, 
    start : (usize, usize), 
    orientation : Direction,
    visited : HashMap<(usize, usize), bool>
}

enum Direction {
    NORTH,
    SOUTH,
    WEST, 
    EAST
}

impl Data {
    fn new() -> Self {
        Data { grid: Vec::new(), start : (0,0), orientation: Direction::NORTH, visited: HashMap::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day6.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.grid.push(line.unwrap().chars().collect());
        }

        let mut skip_flag = false;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == '^' {
                    self.start.0 = row;
                    self.start.1 = col;
                    skip_flag = true;
                    break;
                }
            }

            if skip_flag {
                break;
            }
        }

        println!("{:?}", self.start);
    }

    fn part1(&mut self) -> u64 {
        
        let mut total = 0;

        let mut row = self.start.0;
        let mut col = self.start.1;

        while row >= 0 && row < self.grid.len() && col >= 0 && col < self.grid[row].len() {

            match self.orientation {
                Direction::NORTH => {
                    if self.grid[row - 1][col] == '#' {
                        self.orientation = Direction::EAST;

                    } else {
                        row = row - 1;

                        self.visited.entry((row,col)).or_insert(true);
                    }
                },
                Direction::EAST => {},
                Direction::SOUTH => {},
                Direction::WEST => {},
            }
        }

        total
    }

    fn part2(&mut self) -> u64 {

        let mut total = 0;

        total
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

    }
}



fn main(){
    let mut data = Data::new();
    data.parse();
    println!("{}", data.part1());
    println!("{}", data.part2());
}