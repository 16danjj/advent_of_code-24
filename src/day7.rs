
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    equations : Vec<Equations>
}

#[derive(Debug)]
struct Equations {
    value : usize,
    list : Vec<usize>
}

impl Equations {
    fn solution(&self, part2: bool) -> Option<usize> {
        let mut ops = vec!['+'; self.list.len() - 1];
        loop {
            let total = ops.iter().zip(&self.list[1..]).fold(self.list[0], |val, (op, num)| match op {
                '+' => val + *num, 
                '*' => val * *num,
                '|' => {
                    if *num < 10 {
                        val * 10 + *num
                    } else if *num < 100 {
                        val * 100 + *num
                    } else if *num < 1000 {
                        val * 1000 + *num
                    } else {
                        panic!("Number greater than 1000")
                    }
                }
                _ => {panic!("invalid op {op}")}

            });

            if total == self.value {
                return Some(total)
            }

            let mut pos = ops.len() - 1;

            if (part2) {
                loop {
                    if ops[pos] == '+' {
                        ops[pos] = '*';
                        break;
                    } else if ops[pos] == '*' {
                        ops[pos] = '|';
                        break;
                    }
                    
                    else if pos == 0 {
                        return None;
                    }
    
                    ops[pos] = '+';
                    pos -= 1;
                }
            } else {
                loop {
                    if ops[pos] == '+' {
                        ops[pos] = '*';
                        break;
                    }
                    else if pos == 0 {
                        return None;
                    }
    
                    ops[pos] = '+';
                    pos -= 1;
                }
            }
        }

    }
}

impl FromStr for Equations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let value = left.parse().unwrap();
        let list = right.split(' ').map(|num| num.parse().unwrap()).collect();

        Ok(Self { value, list })
    }
}


impl Data {
    fn new() -> Self {
        Data { equations : Vec::new() }
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day7.txt").unwrap();
        let reader = BufReader::new(&f);

        self.equations = reader.lines().into_iter().map(|line| line.unwrap().parse().unwrap()).collect();

        //println!("{:?}", self.equations);
    }

    fn part1(&mut self) -> usize {
        
        //let mut total = 0;
        //total

        self.equations.iter().filter_map(|x| x.solution(false)).sum::<usize>()
    }

    fn part2(&mut self) -> usize {

        //let mut total = 0;

        //total

        self.equations.iter().filter_map(|x|   x.solution(true)).sum::<usize>()  }

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