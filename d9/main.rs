use curl::easy::Easy;
use curl::easy::List;
use std::fs;
use std::process;

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

fn main() {
    let input: String = get_input(9).trim().to_string();

    // id, len
    let mut fs: Vec<(String, usize)> = vec![];

    let mut choose = true;

    let mut i = 0;
    for c in input.chars().into_iter() {
        let inserted;
        match choose {
            false => inserted = ".".to_string(),
            true => inserted = i.to_string(),
        }
        if choose {
            fs.push((inserted, c.to_digit(10).unwrap() as usize));
            i += 1;
        } else {
            fs.push((".".to_string(), c.to_digit(10).unwrap() as usize));
        }
        choose = !choose;
    }

    print_vec_to_string(&fs);

    // odzadu
    for i in 0..fs.len() - 1 {
        if (fs.len() as i32 - i as i32 - 1) < 0 {
            break;
        }
        let mut bindex = fs.len() - i - 1;

        // jestli narazime na tecky tak pokracujem dal
        if fs[bindex].0 == "." {
            continue;
        }
        // println!("s: {}", fs[bindex].0);

        // nasledne odpredu
        for j in 0..fs.len() - 1 {
            if fs[j].0 == fs[bindex].0 {
                break;
            }
            if fs[j].0 == "." {
                // neni to presne
                if fs[j].1 > fs[bindex].1 {
                    fs[j].1 -= fs[bindex].1;
                    let c = fs[bindex].clone();
                    fs[bindex].0 = ".".to_string();
                    fs.insert(j, c);
                    fs[bindex].0 = ".".to_string();
                    bindex += 1;
                    println!("bindex: {}", bindex);
                    // print_vec_to_string(&fs);
                } else if fs[j].1 == fs[bindex].1 {
                    fs.swap(j, bindex);
                    println!("bindex: {}", bindex);
                    // print_vec_to_string(&fs);
                } else {
                    continue;
                }
                println!(
                    "-1: {}, 0: {}, +1: {}",
                    fs.get(bindex - 1).unwrap_or(&("x".to_string(), 0)).0,
                    fs[bindex].0,
                    fs.get(bindex + 1).unwrap_or(&("x".to_string(), 0)).0
                );
                if fs.get(bindex - 1).unwrap_or(&("x".to_string(), 0)).0 == fs[bindex].0 {
                    if fs[bindex].0 != ".".to_string() {
                        process::exit(1);
                    }
                    fs[bindex - 1].1 += fs[bindex].1;
                    fs.remove(bindex);
                    bindex -= 1;
                }
                if fs.get(bindex + 1).unwrap_or(&("x".to_string(), 0)).0 == fs[bindex].0 {
                    fs[bindex + 1].1 += fs[bindex].1;
                    fs.remove(bindex);
                }
                break;
            }
            continue;
        }
    }

    print_vec_to_string(&fs);
    // println!("{fs:?}");

    fs::write("output.txt", vec_to_string(&fs)).unwrap();

    let mut sum = 0;

    for (i, c) in fs.iter().enumerate() {
        if c.0 == "." {
            continue;
        }
        sum += i as u64 * c.0.parse::<u64>().unwrap() as u64
    }

    println!("Sum: {sum}");

    process::exit(0);
    // println!("P1: {}", part1(&input));
}

fn part1(input: &String) -> u64 {
    let mut fs: Vec<String> = vec![];

    let mut choose = true;

    let mut i = 0;
    for c in input.chars().into_iter() {
        // println!("d: {}", c.to_digit(10).unwrap());
        let mut inserted: String = i.to_string();
        match choose {
            false => {
                inserted = ".".to_string();
            }
            _ => i += 1,
        }
        for _ in 0..(c.to_digit(10).unwrap()) {
            fs.push(String::from(inserted.clone()));
            //println!("{}", String::from(i.to_string()));
        }
        choose = !choose;
    }

    _print_vec_to_string(&fs);

    let mut i = 0;
    while i < fs.len() {
        while fs[i] == "." {
            fs[i] = "x".to_string();
            fs.swap_remove(i);
        }
        i += 1;
    }

    let mut sum: u64 = 0;

    fs.iter()
        .enumerate()
        .for_each(|(i, c)| sum += i as u64 * c.parse::<i32>().unwrap() as u64);
    // print_vec_to_string(&fs);
    sum
}

fn print_vec_to_string(v: &Vec<(String, usize)>) {
    println!("{}", vec_to_string(v));
}

fn _print_vec_to_string(v: &Vec<String>) {
    let s = v
        .iter()
        .map(|c| c.to_string().repeat(c.len()))
        .collect::<String>();
    println!("{}", s);
}

fn vec_to_string(v: &Vec<(String, usize)>) -> String {
    let s = v
        .iter()
        .map(|c| c.0.to_string().repeat(c.1))
        .collect::<String>();
    s
}

/*
fn part2(input: &String) -> i32 {
    let mut sum = 0;

    sum
}
*/
