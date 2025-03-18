
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
        
        let mut disk:Vec<i64>= Vec::new();
        let mut file_no = 0;

        let mut iter = self.disk.iter();

        while let Some(len) = iter.next() {
            let len = *len as u8 - b'0';
            disk.extend(vec![file_no; len as usize]);
            file_no += 1;

            if let Some(gap) = iter.next() {
                let gap = *gap as u8 - b'0';
                disk.extend(vec![-1; gap as usize]);
            }
        }

        

        while let Some(free_index) = disk.iter().position(|val| *val == -1 ) {

            while let Some(val) = disk.pop() {
                if val == -1 {
                    continue
                }

                disk[free_index] = val;
                break;
            }
        }
        

        disk.iter().enumerate().map(|(idx, val)| idx as i64 * val).sum::<i64>()

    }

    fn part2(&mut self) -> i64 {

        let mut disk:Vec<i64>= Vec::new();
        let mut file_no = 0;

        let mut iter = self.disk.iter();

        

        while let Some(len) = iter.next() {
            let len = *len as u8 - b'0';
            disk.extend(vec![file_no; len as usize]);
            file_no += 1;

            if let Some(gap) = iter.next() {
                let gap = *gap as u8 - b'0';
                disk.extend(vec![-1; gap as usize]);
            }
        }


        let mut right = disk.len() - 1;
        self.detailed_disk = disk;
    
        while right != 0 {

            right = self.allocate_full_files(right);
        }

        self.detailed_disk.iter().enumerate().filter_map(|(idx, val)| if *val != -1 {Some(idx as i64 * val)} else {None} ).sum::<i64>()
        
    }

    fn allocate_full_files(&mut self, mut right: usize) -> usize {

        let mut file_length = 1;
        let mut right_returned = 0;

        loop {

            if right != 0 {

                if self.detailed_disk[right] == self.detailed_disk[right - 1] {
                    file_length += 1;
                    right -= 1;
                }

                else {
                    right_returned = right - 1;
                    break
                }

            } else {
                break
            } 

        }

        let mut free_space = 0;
        let mut left = 0;
        

        while left < right {
            if free_space == file_length {
                break;
            }

            if self.detailed_disk[left] == -1 {
                free_space += 1;
                left += 1;
                continue;
            } else {
                free_space = 0;
                left += 1;
            }
        }

        if free_space != file_length {
            return right_returned
        }


        let mut start_index = left - free_space;
        let mut temp_right = right;

        while file_length != 0 {

            self.detailed_disk[start_index] = self.detailed_disk[temp_right];
            self.detailed_disk[temp_right] = -1;

            temp_right += 1;
            start_index += 1;
            file_length -= 1;

        }

        right_returned
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