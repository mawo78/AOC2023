use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

pub fn day_20() {
    let file = File::open("input/input20_2016.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut prz:Vec<(u32,u32)> = lines.iter().map(|ln|{
        let psd:Vec<&str> = ln.split("-").collect();
        (u32::from_str_radix(psd[0], 10).unwrap(), u32::from_str_radix(psd[1], 10).unwrap())
    }).collect();
    prz.sort_by(|l,r|{ l.0.cmp(&r.0)});

    part_2(&prz);
}

fn part_1(prz: &Vec<(u32, u32)>) {
    let mut min_el = prz[0].clone();
    for i in 1..prz.len(){
        let prz_jn = (min_el.0.max(prz[i].0), min_el.1.min(prz[i].1));

        if prz_jn.0 > prz_jn.1 {
            if prz_jn.0 > prz_jn.1 + 1 {
                println!("{}", prz_jn.1 + 1);
                break;
            }
        }
        min_el = (min_el.0.min(prz[i].0), min_el.1.max(prz[i].1));
    }
}

fn part_2(prz: &Vec<(u32, u32)>) {
    let mut min_el = prz[0].clone();
    let mut res_v:Vec<(u32, u32)> = Vec::new();

    for i in 1..prz.len(){
        let prz_jn = (min_el.0.max(prz[i].0), min_el.1.min(prz[i].1));

        if prz_jn.0 > prz_jn.1 {
            if prz_jn.0 > prz_jn.1 + 1 {
                //println!("{}", prz_jn.1 + 1);
                res_v.push(min_el);
                min_el = prz[i].clone();
                //break;
            }
        }
        min_el = (min_el.0.min(prz[i].0), min_el.1.max(prz[i].1));
    }
    res_v.push(min_el);
    println!("{:?}", res_v);
    let res:u32 = res_v.iter().map(|x| x.1 - x.0 + 1).sum();
    println!("{}", u32::MAX - res);
    println!("{}, {}", u32::MAX, res_v.len());

    let mut res = 0;
    for i in 1..res_v.len(){
        res += res_v[i].0 - res_v[i-1].1 - 1;
    }
    println!("{}", res);
}

//part 2
// 4294967179 too high
// 116 too low