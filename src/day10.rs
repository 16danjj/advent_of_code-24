
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    grid : Vec<Vec<u8>>,
    rows : usize,
    cols : usize
}


impl Data {
    fn new() -> Self {
        Data { grid : Vec::new(), rows : 0, cols : 0}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day10.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.grid.push(line.unwrap().chars().map(|ch| ch as u8 - b'0').collect())
        }

        self.rows = self.grid.len();
        self.cols = self.grid[0].len();

        //println!("{:?}", self.grid);

    }

    fn find_score(&mut self, row: usize, col: usize, part2: bool) -> usize {
        
        let mut found = 0;
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        stack.push((row,col,0));

        while let Some((r,c, level)) = stack.pop() {
            if part2 || visited.insert((r,c)) {
                if level == 9 {
                    
                    found += 1;
                    continue
                }

                for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let row = r as i64 + dir.0;
                    let col = c as i64 + dir.1;

                    if row < 0 || col < 0 || row >= self.rows as i64 || col >= self.cols as i64 {
                        continue;
                    }

                    if self.grid[row as usize][col as usize] == level + 1 {
                        stack.push((row as usize, col as usize, level+1));
                    }
                    
                }
            }

        }
        
        found
    }

    fn part1(&mut self) -> usize {
        
        let mut total = 0;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == 0 {
                    total += self.find_score(row, col, false);
                }
            }
        }

        total
    }

    fn part2(&mut self) -> usize {

        let mut total = 0;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == 0 {
                    total += self.find_score(row, col, true);
                }
            }
        }

    total }
    
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