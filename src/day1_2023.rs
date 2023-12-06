use std::{fs::File, io::{BufReader, BufRead}};
pub fn day_1() {
    let file = File::open("input1_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let res:i32=lines.iter().map(|x| {
        let s:String = String::new();
        let mut d = x.chars().fold(s, |mut acc: String, x| {
            if x.is_ascii_digit(){
                acc.push(x);
            }
            acc
        });
        let lst = d.pop().unwrap();
        while d.len() > 1 {
            d.pop();
        }
        while d.len() < 2 {
            d.push(lst);
        }
        println!("{}", d);
        i32::from_str_radix(&d, 10).unwrap()
    }).sum();
    println!("{}", res)
}

// 15817782 too high
// 39362 too low

pub fn day1_part2(){
    let file = File::open("input1_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let res:i32=lines.iter().map(|x| {
        let mut d:String = String::new();
        let mut m:String = x.clone();
        while m.len() > 0 {
            if m.chars().nth(0).unwrap().is_ascii_digit() {
                d.push(m.chars().nth(0).unwrap());
                m.remove(0);
            } else {
                let pom = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
                let mut i = 1;
                for dstr in pom {
                    if m.starts_with(dstr) {
                        d.push(i.to_string().chars().nth(0).unwrap());
                        m.remove(0);
                        break;
                    }
                    i += 1;
                }

                if 10 == i {
                    m.remove(0);
                }
            }
        }

        let lst = d.pop().unwrap();
        while d.len() > 1 {
            d.pop();
        }
        while d.len() < 2 {
            d.push(lst);
        }
        println!("{} -> {}", x, d);
        i32::from_str_radix(&d, 10).unwrap()
    }).sum();
    println!("{}", res)
}

// 53683 too low
// 55445 too high