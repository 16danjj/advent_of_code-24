use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Data {
    nums: Vec<[u64; 2]>
}

impl Data {
    fn new() -> Self {
        Data { nums: Vec::new() }
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day1.txt").unwrap();
        let reader = BufReader::new(&f);
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        for line in reader.lines() {
            let (left,right) = line.as_ref().unwrap().split_once("   ").unwrap();
            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();
            left_list.push(left);
            right_list.push(right);
        }

        left_list.sort();
        right_list.sort();

        self.nums.extend(left_list.iter().zip(right_list.iter()).map(|(a,b)| [*a, *b]));

    }

    fn part1(&mut self) -> u64 {
        self.nums.iter().map(|pair| pair[0].abs_diff(pair[1])).sum()
    }
}


fn main(){
    let mut data = Data::new();
    data.parse();
    print!("{}", data.part1());
}