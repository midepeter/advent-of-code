//run through file 
//break the file into sections using the and empty space and a demacator
//try and build a table that you will be able lookup
//struct seed --> soil, fertilizer, water, light, temperature, humidity and location
//have a map to seed -> properties
//build the map based on the data in the file
//do you lookup

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct Properties {
    fertilizer: u32,
    water: u32,
    light: u32,
    temperature: u32,
    humidity: u32,
    location: u32,
}


pub fn run_day_five()  {
    let file = File::open("inputs/input05.txt").unwrap();
    let mut reader = BufReader::new(file);

   // let map_data = HashMap::new();
    let mut seed = String::new();
    let _ = reader.read_line(&mut seed);

    let seed = String::from(seed.trim());
    let seeds: Vec<&str> = seed.split(":").collect();

    let seeds = clean_up(seeds[1].split(" ").collect());

    let mut prop =  Properties::new();

    //link seed -> properties 
    let seed_map: HashMap<String, Properties> = HashMap::new();

    for section in reader.lines() {
        match section {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                if line == "seed-to-soil map:" {
                    let( src, dest, range) = get_src_value("".to_string());
                    let _ = prop.seed_to_soil(src, dest, range);
                }
            }
            Err(_) => {}
        }
    }
}

impl Properties {
    fn new() -> Properties {
        Properties {
            fertilizer: 0,
            light: 0,
            humidity: 0,
            water: 0,
            temperature: 0,
            location: 0,
        }
    }

    fn seed_to_soil(&mut self,src: i32, dest: i32, range: i32) -> String {
        for i in src..src+range {
            for j in dest..dest+range {
            }
        }
        "".to_string()
    }
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

fn get_src_value(line: String) -> (i32, i32, i32) {
    let value: Vec<&str> = line.split(" ").collect();
    (value[1].parse().expect("not a valid number"),
    value[0].parse().expect("not a valid number"),
    value[2].parse().expect("not a valid number"))
}