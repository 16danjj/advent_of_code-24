
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::u64;

struct Data {
    list: Vec<(i64,i64)>
}



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

        //println!("{:?}", self.list);
 
    }
    fn part1(&mut self) -> usize {
        0
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