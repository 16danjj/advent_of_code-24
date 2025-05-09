use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::u64;

struct Data {
    list: Vec<(i64, i64)>,
}

const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Data {
    fn new() -> Self {
        Data { list: Vec::new() }
    }

    fn search(&self, limit: usize) -> Option<(Vec<(i64, i64)>, i64)>{
        let memory: HashSet<&(i64, i64)> = self.list.iter().take(limit).collect();
        astar(
            &(0i64, 0i64),
            |state| {
                DIRS.iter()
                    .map(|dir| ((state.0 + dir.0, state.1 + dir.1), 1))
                    .filter(|(pos, _)| !memory.contains(&pos))
                    .filter(|(pos, _)| pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 70 && pos.1 <= 70)
                    .collect::<Vec<_>>()
            },
            |state| (70 - state.0) + (70 - state.1),
            |state| *state == (70, 70),
        ) 
    }   

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day18.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.list.push(
                line.unwrap()
                    .split_once(',')
                    .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                    .unwrap(),
            );
        }

        //println!("{:?}", &self.list[0..20]);
    }
    fn part1(&mut self) -> i64 {

        let x = self.search(1024);

        x.unwrap().1
    }

    fn part2(&mut self) -> (i64,i64) {

        let mut coordinates: (i64, i64) = (0,0);
        
        for limit in 1024..self.list.len() {
            if self.search(limit).is_none() {
                 coordinates = self.list[limit - 1];
                 break;
                
            }
        }

        coordinates
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}

fn main() {
    let mut data = Data::new();
    data.parse();
    println!("{}", data.part1());
    println!("{:?}", data.part2());
}
