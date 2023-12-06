use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

pub fn day_4() {
    let file = File::open("input4_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut res = 0;
    //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    lines.iter().for_each(|ln|{
        let mut win:HashSet<i32> = HashSet::new();
        let spl1:Vec<&str> = ln.split("|").collect();
        let win1:Vec<&str> = spl1[0].split(":").collect();
        let win_nmb:Vec<&str> = win1[1].split(" ").collect();
        //println!("{:?}", winNmb);
        win_nmb.iter().for_each(|nb| {
            if nb.len() > 0 {
                win.insert(i32::from_str_radix(nb, 10).unwrap());
            }
        });

        let chk:Vec<&str> = spl1[1].split(" ").collect();
        //println!("{:?}", chk);
        let mut pres = 0;
        chk.iter().for_each(|nb| {
            if nb.len() > 0 {
                if win.contains(&i32::from_str_radix(nb, 10).unwrap()){
                    if pres == 0 {
                        pres = 1;
                    } else {
                        pres *= 2;
                    }
                }
            }
        });

        res += pres;
    });
    println!("{}", res);
}

pub fn day_4_part2() {
    let file = File::open("input4_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut staked:HashMap<i32, i32> = HashMap::new();
    for i in 1..=lines.len() {
        staked.insert(i as i32, 1);
    }
    let mut curr_card_id = 1;
    //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    lines.iter().for_each(|ln|{
        let mut win:HashSet<i32> = HashSet::new();
        let spl1:Vec<&str> = ln.split("|").collect();
        let win1:Vec<&str> = spl1[0].split(":").collect();
        let win_nmb:Vec<&str> = win1[1].split(" ").collect();
        //println!("{:?}", winNmb);
        win_nmb.iter().for_each(|nb| {
            if nb.len() > 0 {
                win.insert(i32::from_str_radix(nb, 10).unwrap());
            }
        });

        let chk:Vec<&str> = spl1[1].split(" ").collect();
        //println!("{:?}", chk);
        let mut pres = 0;
        let incval = *staked.get(&curr_card_id).unwrap();
        chk.iter().for_each(|nb| {
            if nb.len() > 0 {
                if win.contains(&i32::from_str_radix(nb, 10).unwrap()){
                    pres += 1;
                    let c = staked.get(&(pres + curr_card_id)).unwrap();
                    staked.insert(pres + curr_card_id, c + incval);
                }
            }
        });

        curr_card_id += 1;
    });

    let res:i32 = staked.values().sum();
    println!("{}", res);
}