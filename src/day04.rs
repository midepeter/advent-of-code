use std::io::{BufReader, BufRead};
use std::fs::File;

//sample data 
/*
Card   1: 72 28 41 15 98 13 27 99 93 38 | 62  5 80 81 53 29 23 25 59 72 90 19 54 86 68 73 55 21 56 27 32 15 12 42 44
Card   2: 37 28 78 94 73 43 57 44 75 85 |  1 70 85 94 68 39 11 16 86 77 28 25 78 43 71 26 10 97 81 83 31 88 54 60 98
Card   3: 29 62 28 53 94 87 50 67  7 24 | 62 37 95 70 38  7 28 97 24 67 78 35 94 22 21 50 10 87 33 49 92 53 29 55  2
Card   4: 82 49 64 77 88 37 48 55 74 10 | 48  2 97 27 62 42 49 63 94 32 77 40  8 82 52 87 37 76 45 44 88 41 20 74  3
Card   5:  6 67 39 18 36 95 50 74 55 98 | 50 15 18 59 45 75 67 36  4 79 13 98 47 11 97 54 23 95 99 55 37 90  6 42 74
Card   6: 23 15 17 73 24 59 22 80 46 67 | 27 59 60 32 35 14 33 12 44 45 69 73  2 46  7 24 23 92 17 67 25 94 80 22 15
Card   7: 51 76  4 56 62 55 67 16 58 64 | 67 32 82 22 62 55 64 65 24  4 58 45 68 16 61 66 72 96 31 83 21 56 76 51 54
Card   8: 44 48 45 99 54 60 78 37 65  1 | 82 66 45 54 11 94 41  1 26 12 22 43 96 60  9 92 56 46 93 81 61 49 91  3 65
Card   9: 87 25 59 21 58 27 72  9 85 52 | 53  4 20 89 93 42 56 23 17  3 57 10 21  5 26 25 79 98 87 85 27  2 74 54 49
Card  10:  9 76 20 82 41 52 87 78 37 59 | 37 29 76 44 99 56 14 70  9 45 82 93  5 89 95 41 65 20 27 87 61 71 39 51 52

*/
fn run_day_four() {
    let file = File::open("inputs/input04.txt").unwrap();
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