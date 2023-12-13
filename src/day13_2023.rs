use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, time::Instant};

pub fn day_13() {
    let start = Instant::now();
    let file = File::open("input/input13_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut mp:Vec<&String> = Vec::new();
    let mut res = 0;
    lines.iter().for_each(|ln| {
        if ln.is_empty() {
            res += 100 * find_mrrs(&mp);
            let flpd =  flip_mat(&mp);
            //println!("{:?}", flpd);
            let flp_ref:Vec<&String> = flpd.iter().collect();
            res += find_mrrs(&flp_ref);
            mp.clear();
        } else {
            mp.push(ln);
        }
    });
    res += 100 * find_mrrs(&mp);
    let flpd =  flip_mat(&mp);
    //println!("{:?}", flpd);
    let flp_ref:Vec<&String> = flpd.iter().collect();
    res += find_mrrs(&flp_ref);
    println!("part 1: {}", res);
}

fn flip_mat(mat:&Vec<&String>) -> Vec<String> {
    let chrmat:Vec<Vec<char>> = mat.iter().map(|ln| ln.chars().collect::<Vec<char>>()).collect();
    let mut res:Vec<String> = Vec::new();
    for x in 0..mat[0].len(){
        let mut pom = String::new();
        for y in 0..mat.len(){
            pom.push(chrmat[y][x]);
        }
        res.push(pom);
    }

    res
}

fn find_mrrs(mp: &[&String]) -> i32 {
    let mut res:i32 = 0;
    let nums:Vec<i64> = mp.iter().map(|ln| {
        let mut w = 1<<(ln.len() - 1);
        ln.chars().map(|c| {
            let x = match c {
            '#' => w,
            '.' => 0,
            _ => !unreachable!()
            };
            w >>= 1;
            x
        }).sum()
    }).collect();
    println!("{:?}", nums);

    for i in 0..nums.len() {
        let mut x = 0;
        if i < nums.len() / 2 {
            for j in 0..i+1 {
                if nums[i - j] == nums[i + 1 + j] { x += 1} else { break; };
            }
            if i + 1 == x { res = (i + 1) as i32; break;}
        } else {
            for j in 0..(nums.len() - i) {
                if nums[i + j] == nums[i - 1 - j] { x += 1} else { break; };
            }
            if nums.len() - i == x { res = (i ) as i32; break;}
        }
    }
    println!("{}", res);
    res
}

// 11012 too low
// 18259 too low
// 29213 ok