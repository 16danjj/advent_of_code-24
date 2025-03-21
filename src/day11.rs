
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    stones : Vec<Stone>
}

impl Stone {
    fn split(&self) -> Vec<Self> {

        if self.val == 0 {
            return vec![Self {val : 1}]
        }

        let conv_string = format!("{}", self.val);

        if conv_string.len() % 2 == 0 {
            let mid = conv_string.len() / 2;
            let split_1 = &conv_string[0..mid];
            let split_2 = &conv_string[mid..];

            return vec![Self {val : split_1.parse().unwrap()}, Self {val : split_2.parse().unwrap()}]
        } else {
            return vec![Self { val: self.val * 2024}]
        }
    }
}

impl Data {
    fn new() -> Self {
        Data { stones : Vec::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day11.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.stones = line.unwrap().split(' ').map(|stone| stone.parse().unwrap()).collect();
        }

        println!("{:?}", self.stones);

    }
    fn part1(&mut self) -> usize {
        
        let mut current = self.stones.clone();

        for _ in 0..25 {

            let mut next = Vec::new();

            for stone in current {
                next.extend(stone.split());
            }

            current = next;
        }

        current.len()
    }

    fn part2(&mut self) -> usize {

        let mut current = self.stones.iter().map(|stone| (stone.clone(), 1)).collect::<HashMap<Stone, usize>>();
        
        for _ in 0..75 {
            let mut next = HashMap::new();

            for (stone, count) in current {
                for new_stone in stone.split() {
                    let entry = next.entry(new_stone).or_default();
                    *entry += count;
                }
            }
            current = next;
        }
        current.values().sum()
    }
    
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Stone {
    val : usize
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { val: s.parse().unwrap() })
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