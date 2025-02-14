
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Data {
    grid: Vec<Vec<char>>, 

}

const DIR: [(i64, i64); 8] = [(0,-1),(0, 1),(1, 0),(-1, 0),(1, 1),(-1, 1),(-1,-1),(1,-1)];
const LETTERS: [char;4] = ['X', 'M', 'A', 'S'];

const DIR_LEFT_DIAGONAL: [(i64, i64); 2] = [(1, -1),(-1, 1)];
const DIR_RIGHT_DIAGONAL: [(i64, i64); 2] = [(1, 1),(-1,-1)];

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
        let mut letter_dict = std::collections::HashMap::new();
        letter_dict.insert('M', true);
        letter_dict.insert('S', true);

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == 'A' {
                    total += self.find_target_part2(row as i64, col as i64, letter_dict.clone()) as usize;
                }
            }
        }

        total
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

    fn find_target_part2(&mut self, mut row: i64, mut col: i64, target: std::collections::HashMap<char, bool>) -> bool {

        let mut left_check = target.clone();
        let mut right_check = target.clone();

        for dir in DIR_LEFT_DIAGONAL {

            let mut row_new = row;
            let mut col_new = col;

            row_new += dir.0;
            col_new += dir.1;

            if row_new < 0 || col_new < 0 || row_new >= self.grid.len() as i64 || col_new >= self.grid[row_new as usize].len() as i64 {
                return false
            }

            let letter_check = self.grid[row_new as usize][col_new as usize];

            if !left_check.contains_key(&letter_check){
                return false
            }

            left_check.remove(&letter_check);
        }

        for dir in DIR_RIGHT_DIAGONAL {

            let mut row_new = row;
            let mut col_new = col;

            row_new += dir.0;
            col_new += dir.1;

            if row_new < 0 || col_new < 0 || row_new >= self.grid.len() as i64 || col_new >= self.grid[row_new as usize].len() as i64 {
                return false
            }

            let letter_check = self.grid[row_new as usize][col_new as usize];

            if !right_check.contains_key(&letter_check){
                return false
            }

            right_check.remove(&letter_check);
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