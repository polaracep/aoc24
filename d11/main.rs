use curl::easy::Easy;
use curl::easy::List;
use rustc_hash::FxHashMap;
use std::fs;
use std::ops::Add;
use std::process;
use std::time::Instant;

fn get_input(day: i32) -> String {
    let input: String;
    let mut curl = Easy::new();

    let file_name = format!("./input{}.txt", day);
    let token_name = format!("../key");
    let token: String;

    token = match fs::read_to_string(&token_name) {
        Ok(s) => s.trim().to_string(),
        Err(_) => {
            println!("No token!");
            process::exit(1);
        }
    };

    input = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(_) => {
            let mut request_str = String::new();
            {
                println!("Downloading input...");

                let mut headers = List::new();

                headers
                    .append(format!("Cookie: session={}", token).as_str())
                    .unwrap();
                curl.http_headers(headers).unwrap();

                curl.url(format!("https://adventofcode.com/2024/day/{}/input", day).as_str())
                    .unwrap();
                let mut transfer = curl.transfer();

                transfer
                    .write_function(|data| {
                        request_str.push_str(std::str::from_utf8(data).unwrap());
                        Ok(data.len())
                    })
                    .unwrap();
                transfer.perform().unwrap();
            }

            let request_str = &request_str;

            fs::write(file_name, request_str).unwrap();

            request_str.to_string()
        }
    };
    return input;
}

trait AppendExt {
    fn insert_or_add(&mut self, key: u64, cnt: u64) -> Option<u64>;
    fn len_sum(&self) -> u64;
}

impl AppendExt for FxHashMap<u64, u64> {
    fn insert_or_add(&mut self, key: u64, cnt: u64) -> Option<u64> {
        match self.insert(key, cnt) {
            None => None,
            Some(i) => {
                *self.get_mut(&key).unwrap() += i;
                Some(i.add(1))
            }
        }
    }

    fn len_sum(&self) -> u64 {
        let mut sum = 0;
        for e in self.values() {
            sum += e;
        }
        sum
    }
}

fn main() {
    let input = get_input(11);

    println!("{input}");
    let mut stones: Vec<u64> = input
        .trim()
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut unique_stones: FxHashMap<u64, u64> =
        FxHashMap::from_iter(stones.clone().into_iter().map(|x| (x, 1)));

    // println!("{unique_stones:?}");

    for i in 0..25 {
        print!("IT: {} | ", i + 1);
        let now = Instant::now();
        stones = process_stones(stones);
        println!(
            "t: {}, l: {}, s: {:?}",
            now.elapsed().as_millis(),
            stones.len(),
            stones.len()
        );
    }
    println!("=====================");

    for _i in 0..75 {
        // print!("IT: {} | ", i + 1);
        let _now = Instant::now();
        unique_stones = anezka_count(unique_stones);
        // println!( "t: {}, l: {}, s: {:?}", now.elapsed().as_nanos(), unique_stones.len_sum(), unique_stones.len());
    }

    process::exit(0);
}

// 30 188 549 103

fn anezka_count(last: FxHashMap<u64, u64>) -> FxHashMap<u64, u64> {
    let mut next: FxHashMap<u64, u64> = Default::default();
    for num_key in last {
        let stone_str = num_key.0.to_string();
        if num_key.0 == 0 {
            next.insert_or_add(1, num_key.1);
        } else if stone_str.len() % 2 == 0 {
            let split = stone_str.split_at(stone_str.len() / 2);
            next.insert_or_add(split.0.parse().unwrap(), num_key.1);
            next.insert_or_add(split.1.parse().unwrap(), num_key.1);
        } else {
            next.insert_or_add(num_key.0 * 2024, num_key.1);
        }
    }
    next
}

fn get_next(num: u64, i: i32) {
    if i == 75 {
        unsafe {
            ARGH += 1;
        }
        return;
    }
    let num_str = num.to_string();

    if num == 0 {
        get_next(1, i + 1);
    } else if num_str.len() % 2 == 0 {
        let split = num_str.split_at(num_str.len() / 2);
        get_next(split.0.parse().unwrap(), i + 1);
        get_next(split.1.parse().unwrap(), i + 1);
    } else {
        get_next(num * 2024, i + 1);
    }
}

fn transform_stones(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let num_digits = (stone as f64).log(10.0).ceil() as usize;

            if num_digits % 2 == 0 {
                let divisor = 10u64.pow((num_digits / 2) as u32);
                let left_half = stone / divisor;
                let right_half = stone % divisor;
                new_stones.push(left_half);
                new_stones.push(right_half);
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }

    new_stones
}

fn process_stones(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::with_capacity(stones.len() * 2);
    for s in stones.into_iter() {
        let stone_str = s.to_string();
        if s == 0 {
            new_stones.push(1);
        } else if stone_str.len() % 2 == 0 {
            let split = stone_str.split_at(stone_str.len() / 2);
            new_stones.push(split.0.parse().unwrap());
            new_stones.push(split.1.parse().unwrap());
        } else {
            // rule 3
            new_stones.push(s * 2024);
        }
    }
    return new_stones;
}
