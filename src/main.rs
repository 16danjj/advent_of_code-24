
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::u64;

struct Data {
    results : Vec<u64>,
    configurations : Vec<Vec<u64>>
}


impl Data {
    fn new() -> Self {
        Data { results : Vec::new(), configurations : Vec::new() }
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day7.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            let (left, right) = line.as_ref().unwrap().split_once(": ").unwrap();
            self.results.push(left.parse::<u64>().unwrap());
            self.configurations.push(right.split(' ').filter_map(|x| x.parse::<u64>().ok()).collect());
        }

    }

    fn part1(&mut self) -> usize {
        
        let mut total = 0;

        for i in 0..self.results.len() {

            let mut op:  HashMap<&str, Vec<u64>> = HashMap::new();

            let add = op.entry("+").or_default();
            add.push(self.configurations[i][0] + self.configurations[i][1]);
            let mult = op.entry("*").or_default();
            mult.push(self.configurations[i][0] * self.configurations[i][1]);
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