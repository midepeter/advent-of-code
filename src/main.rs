use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input04.txt").unwrap();
    let buf = BufReader::new(file);
    let mut response: i32 = 0;

    for line in buf.lines()  {
        match line {
            Ok(word) => {
                let mut result = Vec::new();
                let mut ans: i32 = 0;
                let num: Vec<&str> = word.split("|").collect();
                let winning_num: Vec<&str> = num[0].split(":").collect();

                for i in clean_up(num[1].split(" ").collect()) {
                   let num: Vec<&str> = clean_up(winning_num[1].split(" ").collect());
                   if num.contains(&i) {
                        result.push(i)
                   }
                }
                
                for i in 0..result.len() {
                    if i == 0 {
                        ans = 1;
                        continue;
                    }
                  //  let val = 2 * i32::try_from(i).unwrap();
                    ans = ans*2;
                }
                response += ans;
            }
            Err(_) => {}
        }
    }
    print!("{}\n", response)
}  

fn clean_up(nums: Vec<&str>) -> Vec<&str> {
    let mut ans: Vec<&str> = Vec::new();

    for i in 0..nums.len() {
        if nums[i] != "" {
            ans.push(nums[i])
        }
    }

    return ans
}