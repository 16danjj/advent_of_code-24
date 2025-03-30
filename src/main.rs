
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    grid : Vec<Vec<char>>,
    rows : usize,
    cols : usize,
    visited : HashSet<(usize,usize)>
}


impl Data {
    fn new() -> Self {
        Data { grid : Vec::new(), rows : 0, cols : 0, visited: HashSet::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day12.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.grid.push(line.unwrap().chars().collect())
        }

        self.rows = self.grid.len();
        self.cols = self.grid[0].len();

        //println!("{:?}", );


    }
    fn part1(&mut self) -> usize {
        
       let mut cost = 0;

       for row in 0..self.grid.len() {
        for col in 0.. self.grid[row].len() {

            if self.visited.contains(&(row, col)) {
                continue;
            }

            cost += self.find_cost(row, col);
        }
       }

       cost
    }

    fn part2(&mut self) -> usize {

        0
    }

    fn find_cost(&mut self, row: usize, col: usize) -> usize {

        let mut area = 0;
        let mut perimeter = 0;
        let mut stack = Vec::new();

        self.visited.insert((row,col));
        stack.push((row,col));

        while let Some((row, col)) = stack.pop() {
            let plot = self.grid[row][col];
            area += 1;

            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {

                let new_row = row as i64 + dir.0;
                let new_col = col as i64 + dir.1;

                if new_row < 0 || new_row >= self.rows as i64 || new_col < 0 || new_col >= self.cols as i64{
                    perimeter += 1; 
                    continue;
                }

                if self.grid[new_row as usize][new_col as usize] != plot {
                    perimeter += 1;
                    continue;
                }

                
                if self.visited.insert((new_row as usize, new_col as usize)){
                    stack.push((new_row as usize, new_col as usize));
                }

            }
        }

        area * perimeter
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