use std::{fs::File, io::{BufReader, BufRead}};
pub fn day_2() {
    let file = File::open("input/input2_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut id = 1;
    let mut gmssum = 0;
    lines.iter().for_each(|ln| {
        let mut prsd:Vec<&str> = ln.split([':', ';']).collect();
        println!("{:?}", prsd);
        prsd.remove(0);

        let mut res = false;
        for gm in prsd {
            let gmpr:Vec<&str> = gm.split(",").collect();
            println!("{:?}", gmpr);

            for dc in gmpr {
                let a:Vec<&str> = dc.split(" ").collect();
                println!("{:?}", a);

                res |= match a[2] {
                    "red" => i32::from_str_radix(a[1], 10).unwrap() > 12,
                    "green" => i32::from_str_radix(a[1], 10).unwrap() > 13,
                    "blue" => i32::from_str_radix(a[1], 10).unwrap() > 14,
                    _ => false
                }
            }
            println!("{}", res);
        }
        if !res {
            gmssum += id;
        }
        id += 1;
    });
    println!("{}", gmssum);
}

// 2408 too low

pub fn day_2_part2() {
    let file = File::open("input/input2_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut res = 0;
    lines.iter().for_each(|ln| {
        let mut prsd:Vec<&str> = ln.split([':', ';']).collect();
        println!("{:?}", prsd);
        prsd.remove(0);

        let mut minr = 0;
        let mut ming = 0;
        let mut minb = 0;
        for gm in prsd {
            let gmpr:Vec<&str> = gm.split(",").collect();
            println!("{:?}", gmpr);

            for dc in gmpr {
                let a:Vec<&str> = dc.split(" ").collect();
                println!("{:?}", a);

                match a[2] {
                    "red" => minr = minr.max(i32::from_str_radix(a[1], 10).unwrap()),
                    "green" => ming = ming.max(i32::from_str_radix(a[1], 10).unwrap()),
                    "blue" => minb = minb.max(i32::from_str_radix(a[1], 10).unwrap()),
                    _ => {}
                }
            }
        }
        res += minr *ming *minb;
        println!("{}", res);
    });
    println!("{}", res);
}
