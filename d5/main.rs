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
    let input = get_input(5);

    println!("Part 1: {}", part1(&input));
    println!("Part 1 v2: {}", part1_v2(&input));

    println!("Part 2: {}", part2(&input));

    process::exit(0);
}

fn part1(input: &String) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let helper = parts[0].trim();

    let mut rules: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| rules.push(l.split("|").map(|n| n.parse::<i32>().unwrap()).collect()));

    let helper = parts[1].trim();

    let mut pages: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| pages.push(l.split(",").map(|n| n.parse::<i32>().unwrap()).collect()));
    let mut sum = 0;

    'pageloop: for page in pages {
        for (i, num) in page.clone().into_iter().enumerate() {
            for r in &rules {
                if num == r[0] {
                    if page.split_at(i).0.contains(&r[1]) {
                        // spatne, je to prohozeny blbe
                        continue 'pageloop;
                    }
                }
            }
        }
        // println!("{page:?}");
        sum += page[page.len() / 2];
    }
    sum
}

fn part1_v2(input: &String) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let helper = parts[0].trim();

    let mut rules: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| rules.push(l.split("|").map(|n| n.parse::<i32>().unwrap()).collect()));

    let helper = parts[1].trim();

    let mut pages: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| pages.push(l.split(",").map(|n| n.parse::<i32>().unwrap()).collect()));

    let mut sum = 0;

    for p in pages {
        // number, score

        let working: Vec<Vec<i32>> = rules
            .clone()
            .into_iter()
            .filter(|r| p.contains(&r[0]) && p.contains(&r[1]))
            .collect();

        let mut nums: Vec<i32> = vec![];

        // get working nums
        let _ = working.clone().into_iter().for_each(|x| {
            if !nums.contains(&x[0]) {
                nums.push(x[0]);
            }
            if !nums.contains(&x[1]) {
                nums.push(x[1]);
            }
        });

        let mut scores: Vec<(i32, i32)> = vec![];

        nums.clone().into_iter().for_each(|n| {
            let mut n_score = 0;
            working.clone().into_iter().for_each(|r| {
                if n == r[0] {
                    n_score -= 1;
                } else if n == r[1] {
                    n_score += 1;
                }
            });
            scores.push((n, n_score));
        });

        scores.sort_by(|a, b| a.1.cmp(&b.1));

        let scores: Vec<i32> = scores.clone().into_iter().map(|x| x.0).collect();

        if scores == p {
            sum += p[p.len() / 2];
        }
    }
    sum
}

fn part2(input: &String) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let helper = parts[0].trim();

    let mut rules: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| rules.push(l.split("|").map(|n| n.parse::<i32>().unwrap()).collect()));

    let helper = parts[1].trim();

    let mut pages: Vec<Vec<i32>> = vec![];
    helper
        .split("\n")
        .for_each(|l| pages.push(l.split(",").map(|n| n.parse::<i32>().unwrap()).collect()));

    let mut sum = 0;

    for p in pages {
        // number, score

        let working: Vec<Vec<i32>> = rules
            .clone()
            .into_iter()
            .filter(|r| p.contains(&r[0]) && p.contains(&r[1]))
            .collect();

        let mut nums: Vec<i32> = vec![];

        // get working nums
        let _ = working.clone().into_iter().for_each(|x| {
            if !nums.contains(&x[0]) {
                nums.push(x[0]);
            }
            if !nums.contains(&x[1]) {
                nums.push(x[1]);
            }
        });

        let mut scores: Vec<(i32, i32)> = vec![];

        nums.clone().into_iter().for_each(|n| {
            let mut n_score = 0;
            working.clone().into_iter().for_each(|r| {
                if n == r[0] {
                    n_score -= 1;
                } else if n == r[1] {
                    n_score += 1;
                }
            });
            scores.push((n, n_score));
        });

        scores.sort_by(|a, b| a.1.cmp(&b.1));

        let scores: Vec<i32> = scores.clone().into_iter().map(|x| x.0).collect();

        if scores != p {
            sum += scores[scores.len() / 2];
        }
    }
    sum
}
