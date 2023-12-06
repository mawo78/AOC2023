use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

pub fn day_5_part1() {
    let file = File::open("input5_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut res = i64::MAX;
    let mut seeds:Vec<i64> = Vec::new();
    let mut maps:Vec<Vec<(i64,i64,i64)>> = Vec::new();
    lines.iter().for_each(|ln|{
        if ln.starts_with("seeds:"){
            // seeds: 79 14 55 13
            seeds = ln.split(" ").map(|p| { 
                match i64::from_str_radix(p, 10) {
                    Ok(x) => x,
                    Err(_) => -1,
                }
            }).collect();
            seeds.remove(0);
            println!("{:?}", seeds);

        } else if ln.contains(":"){
            if !maps.is_empty() {                
                println!("{:?}", maps.last().unwrap());
            }
            maps.push(Vec::new());
        } else if !ln.is_empty() {
            //0 15 37
            let psd:Vec<&str> = ln.split(" ").collect();
            let a = i64::from_str_radix(psd[0], 10).unwrap();
            let b = i64::from_str_radix(psd[1], 10).unwrap();
            let c = i64::from_str_radix(psd[2], 10).unwrap();
            let m = (a, b, c);
            maps.last_mut().unwrap().push(m);
        }
    });

    seeds.iter().for_each(|s0| {
        let mut s = *s0;
        maps.iter().for_each(|mp|{
            let mut mapped = false;
            mp.iter().for_each(|m| {
                //50 98 2
                let r = *m;
                //println!("{:?}", r);
                if s >= r.1 && s < r.1 + r.2 && !mapped{
                    s = r.0 + s - r.1;
                    mapped = true; 
                }
            });
            //print!(" [{} -> {}]", s0, s);
        });
        //println!();
        res = res.min(s);
    });

    println!("{}", res)
}

pub fn day_5_part2() {
    let file = File::open("input5_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut res = i64::MAX;
    let mut seeds:Vec<i64> = Vec::new();
    let mut seed_rng:Vec<(i64,i64)> = Vec::new();
    let mut maps:Vec<Vec<(i64,i64,i64)>> = Vec::new();
    lines.iter().for_each(|ln|{
        if ln.starts_with("seeds:"){
            // seeds: 79 14 55 13
            seeds = ln.split(" ").map(|p| { 
                match i64::from_str_radix(p, 10) {
                    Ok(x) => x,
                    Err(_) => -1,
                }
            }).collect();
            seeds.remove(0);
            println!("{:?}", seeds);

            let mut i = 0;
            let mut pr = (0,0);
            seeds.iter().for_each(|s| {
                if i % 2 == 0 {
                    pr.0 = *s;
                } else {
                    pr.1 = pr.0 + *s - 1;
                    seed_rng.push(pr);
                }
                i += 1;
            });

            println!("{:?}", seed_rng);

        } else if ln.contains(":"){
            if !maps.is_empty() {                
                println!("{:?}", maps.last().unwrap());
            }
            maps.push(Vec::new());
        } else if !ln.is_empty() {
            //0 15 37
            let psd:Vec<&str> = ln.split(" ").collect();
            let a = i64::from_str_radix(psd[0], 10).unwrap();
            let b = i64::from_str_radix(psd[1], 10).unwrap();
            let c = i64::from_str_radix(psd[2], 10).unwrap();
            let m = (a, b, c);
            maps.last_mut().unwrap().push(m);
        }
    });

    let mut stack = seed_rng.clone();
    let mut mapped:Vec<(i64,i64)> = Vec::new();

    maps.iter().for_each(|mp|{
        while !stack.is_empty() {
            let mut rng = stack.pop().unwrap();
            mp.iter().for_each(|m| {

                let low = m.1.max(rng.0);
                let high = rng.1.min(m.1 + m.2 - 1);
                if low <= high {
                    mapped.push((low + m.0 - m.1, high + m.0 - m.1));
                    if rng.0 <= low - 1 {
                        stack.push((rng.0, low - 1));
                    }

                    if high+1 <= rng.1 {
                        stack.push((high+1, rng.1));
                    }

                    rng = (-1, -1);
                }
            });
            if rng.0 != -1 && rng.1 != -1 {
                mapped.push(rng);
            }
        }
        stack = mapped.clone();
        mapped.clear();
    });

    stack.sort();

    println!("{}", stack[0].0);
}
