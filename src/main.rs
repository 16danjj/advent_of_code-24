
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Data {
    grid: Vec<Vec<char>>, 

}

const DIR: [(i64, i64); 8] = [(0,-1),(0, 1),(1, 0),(-1, 0),(1, 1),(-1, 1),(-1,-1),(1,-1)];
const LETTERS: [char;4] = ['X', 'M', 'A', 'S'];

const DIR_2: [(i64, i64); 4] = [(1, 1),(-1, 1),(-1,-1),(1,-1)];

impl Data {
    fn new() -> Self {
        Data { grid: Vec::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day4.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.grid.push(line.unwrap().chars().collect());
        }

        //println!("{}", self.nums.len());
    }

    fn part1(&mut self) -> usize {
        
        let mut total = 0;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                for dir in DIR {
                    total += self.find_target(row as i64, col as i64, dir) as usize;
                }
            }
        }

        total
    }

    fn part2(&mut self) -> usize {

        let mut total = 0; 
        //let letter_dict = Hash
        0
    }

    fn find_target(&mut self, mut row: i64, mut col: i64, dir: (i64, i64)) -> bool {
        
        for letter in LETTERS {

            if row < 0 || col < 0 || row >= self.grid.len() as i64|| col >= self.grid[row as usize].len() as i64 {
                return false
            }

            if letter != self.grid[row as usize][col as usize] {
                return false
            }

            row += dir.0;
            col += dir.1;

        }

        true
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