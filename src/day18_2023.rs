use std::{fs::File, io::{BufReader, BufRead}};

pub fn day_18_part1() {
    let file = File::open("input/input18_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut pos = (1,1);
    let mut vtx:Vec<(i32,i32)> = Vec::new();
    vtx.push(pos.clone());
    let mut brdr = 0;

    let mut vtx2:Vec<(i32, i32)> = lines.iter().map(|ln| {
        // R 6 (#70c710)
        let psd:Vec<&str> = ln.split(" ").collect();
        let stps = i32::from_str_radix(psd[1], 10).unwrap();

        match psd[0].chars().next().unwrap() {
            'L' => {
                pos.1 -= stps;
            },
            'R' => {
                pos.1 += stps;
            },
            'U' => {
                pos.0 -= stps;
            },
            'D' => {
                pos.0 += stps;
            },
            _ => unreachable!()
        }
        brdr += stps;
        pos.clone()
    }).collect();
    vtx.append(&mut vtx2);

    let mut prev = vtx.last().unwrap().clone();
    let mut res:i32 = vtx.iter().peekable().map(|v| {
        let ps = (prev.1 + v.1) * (prev.0 - v.0);
        prev = v.clone();
        ps
    }).sum();

    res = res.abs() / 2;

    res = res + brdr/2 + 1;

    println!("Part 1: {}", res);
}

pub fn day_18_part2() {
    let file = File::open("input/input18_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut pos = (1, 1);
    let mut vtx:Vec<(i64,i64)> = Vec::new();
    vtx.push(pos.clone());
    let mut brdr:i64 = 0;

    let mut vtx2:Vec<(i64, i64)> = lines.iter().map(|ln| {
        // R 6 (#70c710)
        let psd:Vec<&str> = ln.split(" ").collect();
        let pom = psd[2].get(2..7).unwrap();
        let stps = i64::from_str_radix(pom, 16).unwrap();

        match psd[2].get(7..8).unwrap() {
            "2" => {
                pos.1 -= stps;
            },
            "0" => {
                pos.1 += stps;
            },
            "3" => {
                pos.0 -= stps;
            },
            "1" => {
                pos.0 += stps;
            },
            _ => unreachable!()
        }
        brdr += stps;
        pos.clone()
    }).collect();
    vtx.append(&mut vtx2);

    let mut prev = vtx.last().unwrap().clone();
    let mut res:i64 = vtx.iter().peekable().map(|v| {
        let ps:i64 = (prev.1 as i64 + v.1  as i64) * (prev.0  as i64 - v.0 as i64);
        prev = v.clone();
        ps
    }).sum();

    res = res.abs() / 2;

    res = res + brdr/2 + 1;

    println!("Part 2: {}", res);
}
