
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
struct Data {
    update: Vec<Vec<u64>>, 
    rules : HashMap<u64, HashSet<u64>>
}

impl Data {
    fn new() -> Self {
        Data { update: Vec::new(), rules: HashMap::new()}
    }

    fn parse(&mut self) {
        let f = File::open("inputs\\input_day5.txt").unwrap();
        let reader = BufReader::new(&f);

        for line in reader.lines() {

            if let Some((left, right)) = line.as_ref().unwrap().split_once("|") {
                let left = left.parse::<u64>().unwrap();
                let right = right.parse::<u64>().unwrap();

                let e = self.rules.entry(right).or_default();
                e.insert(left);
                continue
                
            }

            if line.as_ref().unwrap().len() == 0 {
                continue
            }

            self.update.push(line.unwrap().split(",").map(|e| e.parse::<u64>().unwrap()).collect());
        }

        //println!("{:?}", self.rules);
    }

    fn part1(&mut self) -> u64 {
        
        let mut total = 0;

        'next: for update in &self.update {

            for i in 0..update.len() - 1 {
                for j in i + 1..update.len() {
                    if let Some(hs) = self.rules.get(&update[i]) {
                        if hs.contains(&update[j]) {
                            continue 'next
                        }
                    }
                }
            }

            let middle = update.len()/2;
            total += update[middle];
        }

        total
    }

    fn part2(&mut self) -> u64 {

        let mut total = 0;
        let mut unordered = false;

        for update in &mut self.update {
            unordered = false;

            for i in 0..update.len() - 1 {
                for j in i + 1..update.len() {
                    if let Some(hs) = self.rules.get(&update[i]) {
                        if hs.contains(&update[j]) {
                            unordered = true;
                            
                            let temp = update[j];
                            update[j] = update[i];
                            update[i] = temp;
                        }
                    }
                }
            }

            if (unordered) {
                let middle = update.len()/2;
                total += update[middle];
            }

        }

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