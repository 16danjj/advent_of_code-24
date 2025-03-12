
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    grid : Vec<Vec<char>>, 
    map : HashMap<char, Vec<(i64,i64)>>,
    rows : usize, 
    cols : usize,
    result : HashSet<(i64, i64)>
}


impl Data {
    fn new() -> Self {
        Data { grid : Vec::new(), map : HashMap::new(), rows : 0, cols : 0, result : HashSet::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day8.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.grid.push(line.unwrap().chars().collect());
        }

        self.rows = self.grid.len();
        self.cols = self.grid[0].len();

        for (row, line) in self.grid.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                if *ch != '.' {
                    let entry = self.map.entry(*ch).or_default();
                    entry.push((row as i64, col as i64));
                }

            }
        }
    }

    fn part1(&mut self) -> usize {
        
        let mut total = HashSet::new();

        for list in self.map.values() {

            for i in 0..list.len() - 1 {
                for j in i+1..list.len() {
                    let rise = list[j].0 - list[i].0;
                    let run = list[j].1 - list[i].1;

                    let p1 = (list[i].0 - rise, list[i].1 - run);
                    let p2 = (list[j].0 + rise, list[j].1 + run);

                    if self.in_bound(p1) {
                        total.insert(p1);
                    }

                    if self.in_bound(p2) {
                        total.insert(p2);
                    }


                }
            }
        }

    
        total.len()

        
    }

    fn part2(&mut self) -> usize {

        //let mut total = Vec::new();

        for list in self.map.values() {
            for i in 0..list.len() - 1 {
                for j in i+1..list.len() {
                    let rise = list[j].0 - list[i].0;
                    let run = list[j].1 - list[i].1;
                    
                    let mut p1 = (list[i].0, list[i].1);
                    
                    while self.in_bound(p1) {
                        self.result.insert(p1);
            
                        p1.0 = p1.0 - rise;
                        p1.1 = p1.1 - run;
                    }

                    let mut p2 = (list[j].0, list[j].1);

                    while self.in_bound(p2) {
                        self.result.insert(p2);
            
                        p2.0 = p2.0 + rise;
                        p2.1 = p2.1 + run;
                    }
                }
            }
        }

        self.result.len()

    }

    fn in_bound(&self, pos: (i64,i64)) -> bool {

        if pos.0 >= 0 && pos.0 < self.rows as i64 && pos.1 >= 0 && pos.1 < self.cols as i64 {
            return true
        }

        false
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