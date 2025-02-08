use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


struct Data {
    nums: Vec<Vec<u64>>, 
    state: State
}

#[derive(PartialEq)]
enum State {
    None,
    Increasing,
    Decreasing
}

impl Data {
    fn new() -> Self {
        Data { nums: Vec::new(), state: State::None }
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day2.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {
            self.nums.push(line.unwrap().split(' ').filter_map(|x| x.parse::<u64>().ok()).collect());
        }

        //println!("{}", self.nums.len());
    }

    fn part1(&mut self) -> usize {
        self.nums.iter().filter(|list| is_safe(list)).count()
    }

    fn part2(&mut self) -> u64 {
        let mut safe_reports = 0;

        for report in self.nums.iter(){
            if is_safe(&report) {
                safe_reports += 1;
            } else {
                for id in 0..report.len() {
                    let mut data = report.clone();
                    data.remove(id);

                    if is_safe(&data){
                        safe_reports += 1;
                        break;
                    }
                }
            }
        }

        safe_reports
    }

}

fn is_safe(nums: &[u64]) -> bool {
    if match nums[0].cmp(&nums[1]) {
        Ordering::Less => nums.windows(2).any(|pair| pair[0] > pair[1]),
        Ordering::Equal => false, 
        Ordering::Greater => nums.windows(2).any(|pair| pair[0] < pair[1]), 
    } { return false;}

    !nums.windows(2).any(|pair| !(1..=3).contains(&pair[0].abs_diff(pair[1])))
}

fn main(){
    let mut data = Data::new();
    data.parse();
    println!("{}", data.part1());
    println!("{}", data.part2());
}