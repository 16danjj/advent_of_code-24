
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    disk : Vec<char>,
    detailed_disk : Vec<i64>
}


impl Data {
    fn new() -> Self {
        Data { disk : Vec::new(), detailed_disk : Vec::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day9.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.disk.extend(line.unwrap().chars());
        }

        //println!("{:?}", self.disk);

    }

    fn part1(&mut self) -> i64 {
        
        0
    }

    fn part2(&mut self) -> i64 {

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