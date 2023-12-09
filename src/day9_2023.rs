use std::{fs::File, io::{BufReader, BufRead}};

pub fn day_9() {
    let file = File::open("input/input9_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    // part 1
    let res = lines.iter().map(|ln| extrapolate(ln)).sum::<i64>();
    println!("part 1: {}", res);
    // part 2
    //vals.reverse();
    let res = lines.iter().map(|ln| extrapolate2(ln)).sum::<i64>();
    println!("part 2: {}", res);
}
fn extrapolate2(ln: &str) -> i64 {
    let mut vals = ln.split(" ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect::<Vec<i64>>();
    vals.reverse();
    extrapolate_main(&vals)
}
fn extrapolate(ln: &str) -> i64 {
    let vals = ln.split(" ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect::<Vec<i64>>();
    extrapolate_main(&vals)
}
fn extrapolate_main(vals: &Vec<i64>) -> i64 {
    let mut delta_col:Vec<i64> = Vec::new();
    let mut cur_row = vals.clone();
    for _i in 0..vals.len() - 1 {
        let mut delta_row:Vec<i64> = Vec::new();
        for j in 0..cur_row.len() - 1{
            delta_row.push(cur_row[j + 1] - cur_row[j]);
        }
        delta_col.push(delta_row.last().unwrap().clone());
        cur_row = delta_row;
    }
    delta_col.reverse();
    //println!("{:?}", delta_col);

    let mut res_row:Vec<i64> = Vec::new();
    res_row.push(0);
    for i in 1..delta_col.len() {
        res_row.push(res_row[i-1] + delta_col[i]);
    }
    //println!("{:?}", res_row);
    let res = res_row.last().unwrap() + vals.last().unwrap();
    //println!("{:?}", res);
    res
}