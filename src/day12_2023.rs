use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, time::Instant};
use lazy_static::lazy_static;

use regex::Regex;

pub fn day_12() {
    let file = File::open("input/input12_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let res:i32 = lines.iter().map(|ln| find_ways(ln)).sum();
    println!("part1: {}", res);
    let mut i = 1;
    let res:i32 = lines.iter().map(|ln| {print!("{}", i); i+=1; find_ways2(ln)}).sum();
    println!("part2: {}", res);
}

fn parse_line(ln:&String) -> (String, Vec<usize>){
    let psd0 = ln.split(' ').collect::<Vec<&str>>();
    let patr = psd0[0].to_string();
    let grps:Vec<usize> = psd0[1].split(",").map(|s| usize::from_str_radix(s, 10).unwrap()).collect();
    (patr, grps)
}

fn find_ways(ln: &String) -> i32 {
    let mut res = 0;
    let psd0 = parse_line(ln);
    let grps = psd0.1;
    let patr = psd0.0;

    let qmarks = patr.chars().filter(|c| *c == '?').count();
    //println!("{}", patr);
    let re = Regex::new(r"#+").unwrap();
    for x in 0..1<<(qmarks){
        let mut x1 = x;
        let pom = String::from_iter(            
        patr.chars().map(|c|{
            let z = match c {
                '#' => '#',
                '.' => '.',
                '?' => {let z = if x1 % 2 == 1 {'#'} else {'.'}; x1 /= 2; z},
                 _  => unreachable!()
            };
            z
        }));

        let v:Vec<usize> = re.find_iter(&pom).filter_map(|gr| Some(gr.as_str().len())).collect();
        //println!("{} {:?}", pom, v);

        if v == grps {
            res += 1;
        }
    }
    res
}

fn find_ways2(ln:&String) -> i32 {
    let start = Instant::now();
    let mut res = 0;
    let psd0 = parse_line(ln);
    let grps = psd0.1.repeat(5);
    let mut patr = psd0.0.clone();
    for _ in 0..4 {
        patr.push('?');
        patr.push_str(&psd0.0);    
    }

    let mut cache:HashMap<String, i32> = HashMap::new();

    res = req(&patr, &grps, &mut cache);
    let duration = start.elapsed();
    println!("Time elapsed in calculation is: {:?}", duration);

    res
}

fn req2(grps: &Vec<usize>, patr: &String, cache: &mut HashMap<String, i32>) -> i32 {
    if cache.contains_key(patr) {
        print!("*");
        return *cache.get(patr).unwrap();
    }

    let mut res:i32 = 0;
    if patr.contains("?"){
        let p1 = patr.replacen("?", "#", 1);
        res += req2(grps, &p1, cache);
        let p1 = patr.replacen("?", ".", 1);
        res += req2(grps, &p1, cache);
    } else {
        lazy_static! {
            static ref RE:Regex = Regex::new(r"#+").unwrap();
        }
        let v:Vec<usize> = RE.find_iter(&patr).filter_map(|gr| Some(gr.as_str().len())).collect();
        //println!("{} {:?}", pom, v);

        if v == *grps {
            res += 1;
        }
    }
    cache.insert(patr.clone(), res);
    res
}

fn req(patr: &String, grps: &Vec<usize>, cache:&HashMap<String, i32>) -> i32 {
    //println!("[{}]   {:?}", patr, grps);
    let mut res = 0;

    if grps.is_empty() {
        if check_postfix(patr) {
            return 1
        } else {
            return 0
        }
    } else if patr.is_empty() {
        return 0;
    }

    let pom:Vec<char> = patr.chars().collect();

    let mut p0 = 0;
    while p0 < patr.len() && prefix_valid(&pom, p0){
        if p0 + grps[0] <= patr.len() && valid_block(&pom, p0, grps[0]) {
            let mut grps_cl = grps.clone();
            grps_cl.remove(0);
            let patr_cl = if p0 + grps[0] + 1 < patr.len() {
                patr.split_at(p0 + grps[0] + 1).1.to_string()
            } else {
                "".to_string()
            };
            res += req(&patr_cl, &grps_cl, cache);
        }
        p0 += 1;
    }

    res
}

fn check_postfix(patr: &str) -> bool {
    return patr.is_empty() || !patr.contains("#");
}

fn valid_block(pom: &[char], p0: usize, grps: usize) -> bool {
    for i in p0..p0 + grps {
        if pom[i] == '.' {
            return false;
        }
    }
    return p0 + grps == pom.len() || pom[p0 + grps] != '#';
}

#[test]
fn test_valid_block() {
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 0, 3), false);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 1, 2), true);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 2, 1), true);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 3, 3), false);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 4, 3), true);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 5, 2), true);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 8, 3), true);
    assert_eq!(valid_block(&".??.#??.????###".chars().collect::<Vec<char>>(), 11, 4), true);
}

// all before p0 : 0..p0
fn prefix_valid(pom:&Vec<char>, p0:usize) -> bool {
    if 0 == p0 {
        true
    } else {
        //for x in 0..p0 {
            if pom[p0 - 1] == '#' {
                return false
            }
        //}
        true
    }
}

#[test]
fn test_prefix_valid() {
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 0), true);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 1), true);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 2), true);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 3), true);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 4), true);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 5), false);
    assert_eq!(prefix_valid(&".??.#??.????###".chars().collect::<Vec<char>>(), 7), false);
}

#[test]
fn test1(){
    let n = find_ways(&".??..??...?##. 1,1,3".to_string());
    println!("{}", n);
    let n = find_ways(&"?###???????? 3,2,1".to_string());
    println!("{}", n);
    
    let n = find_ways(&".??..??...?##..??..??...?##. 1,1,3,1,1,3".to_string());
    println!("{}", n);
    let n = find_ways(&"?###?????????###???????? 3,2,1,3,2,1".to_string());
    println!("{}", n);
    let n = find_ways(&"??#???????????#?? 1,1,2,4,4".to_string());
    println!("{}", n);    
}

#[test]
fn test2(){
    let mut cache:HashMap<String, i32> = HashMap::new();
    let ln = "?###???????? 3,2,1".to_string();
    //let ln ="??#???????????#?? 1,1,2,4,4".to_string();
    let psd0 = parse_line(&ln);
    let grps = psd0.1.repeat(1);
    let patr = psd0.0.repeat(1);
   
    let res = req(&patr, &grps, &cache);
    println!("{}", res);

    let ln ="??#???????????#?? 1,1,2,4,4".to_string();
    let psd0 = parse_line(&ln);
    let grps = psd0.1.repeat(1);
    let patr = psd0.0.repeat(1);
   
    let res = req(&patr, &grps, &cache);
    println!("{}", res);
}