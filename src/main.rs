
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Data {
    nums: Vec<String>, 

}

impl Data {
    fn new() -> Self {
        Data { nums: Vec::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day3.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.nums.push(line.unwrap());
        }

        //println!("{}", self.nums.len());
    }

    fn part1(&mut self) -> usize {
        0
    }

    fn part2(&mut self) -> usize {
        0

}



fn main(){
    let mut data = Data::new();
    data.parse();
    println!("{}", data.part1());
    println!("{}", data.part2());
}