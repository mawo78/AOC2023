use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn day_15() {
    let file = File::open("input/input15_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut res:i32 = lines[0].split(",").map(|s|{
        hash(s)
    }).sum();

    println!("part1: {}", res);

    res = 0;
    let mut boxs:HashMap<i32, Vec<(String, i32)>> = HashMap::new();
    lines[0].split(",").for_each(|s|{
        if s.ends_with("-") {
            let bxname = s.split("-").next().unwrap();
            let bx = hash(bxname);
            if boxs.contains_key(&bx) {
                let v = boxs.get_mut(&bx).unwrap();
                let pos = v.iter().position(|x| x.0 == bxname);
                if pos.is_some() {
                    v.remove(pos.unwrap());
                }
            }
        } else {
            let psd:Vec<_> = s.split("=").collect();
            let bx = hash(psd[0]);
            if boxs.contains_key(&bx) {
                let v = boxs.get_mut(&bx).unwrap();
                let pos = v.iter().position(|x| x.0 == psd[0]);
                if pos.is_some() {
                    v[pos.unwrap()].1 = i32::from_str_radix(psd[1], 10).unwrap();
                } else {
                    v.push((psd[0].to_string(), i32::from_str_radix(psd[1], 10).unwrap()));
                }

            } else {
                boxs.insert(bx, vec![(psd[0].to_string(), i32::from_str_radix(psd[1], 10).unwrap())]);
            }
        }

    });

    res = boxs.iter().map(|it| { 
        let mut i = 0;
        let k = it.0;
        let v = it.1;
        (k+1) * v.iter().map(|x| { i+=1; i *x.1 }).sum::<i32>()
    }).sum();
    println!("part2: {}", res);

}

fn hash(s: &str) ->i32 {
    let bx:i32 = s.chars().fold(0, |acc, x| ((acc + x as i32) *17 % 256) );
    bx
}
