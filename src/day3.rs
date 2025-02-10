
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
        let mut result = 0;
        for i in 0..self.nums.len() {
            result += process_unconditional(&self.nums[i]);
        }

        result
    }

    fn part2(&mut self) -> usize {
        let  mut result = 0;
        let mut total = 0;
        let s = self.nums.clone().into_iter().collect::<String>();
        let mut enabled = true;

        //println!("{}", s);

        for i in 0..self.nums.len() {
            (result, enabled) = process_conditional(&self.nums[i], enabled);
            total += result;
        }

        total
    }

}

fn process_unconditional(s: &str) -> usize {
    
    let mut total = 0;
    let mut pos = 0;
    while let Some(loc) = &s[pos..].find("mul(") {
        pos += loc + 4;
        if let Some(value) = multiply(&s[pos..]) {
            total += value;
        }
    }

    total
}

fn process_conditional(s: &str, enabled: bool) -> (usize, bool) {

    let mut total = 0;
    let mut pos = 0;
    let mut flag = enabled;

    while let Some(loc) = get_next(&s[pos..]) {
        pos += loc;
        match &s[pos..pos+3] {
            "mul" => {
                if flag {
                    if let Some(value) = multiply(&s[pos+4..]) {
                        total += value;
                    }
                }
            }, 
            "do(" => {flag = true;},
            "don" => {flag = false;},
            _ => panic!("bad pattern : {}", &s[pos..pos+4])
        }

        pos += 4;
    }
    
    (total, flag)
}

fn get_next(s:&str) -> Option<usize> {
    let mul_loc = s.find("mul(");
    let do_loc = s.find("do()");
    let dont_loc = s.find("don't()");

    [mul_loc, do_loc, dont_loc].iter().filter_map(|loc| *loc).min()
}


fn multiply(s: &str) -> Option<usize> {
    
    let (left, rest)= s.split_once(',')?;

    let left = left.parse::<usize>().ok()?;

    let (right, _) = rest.split_once(')')?;

    let right = right.parse::<usize>().ok()?;

    Some(left*right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(Some(25), multiply("5,5)and_the_rest"));
        assert_eq!(None, multiply("5;5)and_the_rest"));
    }

    #[test]
    fn test_line() {
        assert_eq!(161, process_unconditional("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
    }

    #[test]
    fn test_line_part2() {
        let mut flag = true;
        assert_eq!((48, true), process_conditional("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", flag));
    }
}

fn main(){
    let mut data = Data::new();
    data.parse();
    println!("{}", data.part1());
    println!("{}", data.part2());
}