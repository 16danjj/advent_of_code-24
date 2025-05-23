use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::u64;

struct Data{
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Data{
    fn new() -> Self {
        Data { patterns: Vec::new(), designs : Vec::new()}
    }

    fn can_make_design(&self, design: &str, cache : &mut HashMap<String, usize>) -> usize {

        if let Some(value) = cache.get(design) {
            return *value
        }

        if design.is_empty() {
            return 1;
        }

        let count = self.patterns.iter().filter_map(|pattern| if design.starts_with(pattern) { Some (self.can_make_design(&design[pattern.len()..], cache))} else {None}).sum();

        cache.insert(design.to_string(), count);
        count
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day19.txt").unwrap();
        let reader = BufReader::new(&f);

        for (index, line) in reader.lines().enumerate(){
            if index == 0 {
                self.patterns= line.unwrap().split(", ").map(|p| p.to_string()).collect();
            }

            else if index > 1 {
                self.designs.push(line.unwrap());
            }
        }

        //println!("{:?}", self.patterns);
    }
    fn part1(&mut self) -> usize {
        let mut cache = HashMap::new();
        self.designs.iter().filter(|design| self.can_make_design(design, &mut cache) > 0).count()
    }

    fn part2(&mut self) -> usize {
        let mut cache = HashMap::new();
        self.designs.iter().map(|design| self.can_make_design(design, &mut cache)).sum()
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
    println!("{}", data.part2());
}
