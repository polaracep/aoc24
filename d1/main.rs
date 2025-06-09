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
    let input = get_input(1);

    let mut list1 = Vec::<String>::new();
    let mut list2 = Vec::<String>::new();

    let mut list_helper = Vec::<String>::new();

    input.split(&['\n', ' ']).into_iter().for_each(|x| {
        if !x.is_empty() {
            list_helper.push(x.to_string());
        }
    });

    for i in 0..(list_helper.len() / 2) {
        list1.push(list_helper[i * 2].clone());
    }

    for i in 0..(list_helper.len() / 2) {
        list2.push(list_helper[1 + (i * 2)].clone());
    }

    let mut sum: i32 = 0;
    // 1. cast
    {
        // list1.sort_unstable();
        // list2.sort_unstable();

        // // list1[0].parse().unwrap();
        // for i in 0..list1.len() {
        //     sum += i32::abs(list1[i].parse::<i32>().unwrap() - list2[i].parse::<i32>().unwrap());
        // }
    }

    // 2. cast
    {
        for i in 0..list1.len() {
            let number = list1[i].clone();
            let mut count = 0;
            for j in 0..list2.len() {
                if number == list2[j] {
                    count += 1;
                }
            }
            sum += count * number.parse::<i32>().unwrap();
        }
    }

    println!("Sum: {}", sum);
    process::exit(0);
}
