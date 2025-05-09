
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;
use pathfinding::prelude::astar;

struct Data {
    list: Vec<(i64,i64)>
}

const DIRS: [(i64,i64);4] = [(0,1), (0,-1), (1,0), (-1,0)];

impl Data {
    fn new() -> Self {
        Data { list : Vec::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day18.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.list.push(line.unwrap().split_once(',').map(|(x,y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())).unwrap());
        }

        //println!("{:?}", &self.list[0..20]);
 
    }
    fn part1(&mut self) -> i64 {
        let memory : HashSet<&(i64,i64)> = self.list.iter().take(1024).collect();
        let Some((_path, cost)) = astar(&(0i64,0i64), |state| {
            DIRS.iter().map(|dir| ((state.0 + dir.0, state.1 + dir.1), 1)).filter(|pos| !memory.contains(&pos.0))
            .filter(|pos| {pos.0.0 >= 0 && pos.0.1 >= 0 && pos.0.0 <= 70 && pos.0.1 <= 70} ).collect::<Vec<_>>()
        }, |state| (70 - state.0) + (70 - state.1), |state| *state == (70,70))
        else {
            panic!("No path found");
        };
        
        cost

    }

    fn part2(&mut self) -> usize {
        0
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