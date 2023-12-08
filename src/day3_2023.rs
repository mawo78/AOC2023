use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
pub fn day_3_part1() {
    let file = File::open("input/input3_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut dots:Vec<char> = Vec::new();
    for _ in 0..lines[0].len() + 2 {
        dots.push(' ');
    }

    let mut tab:Vec<Vec<char>> = Vec::new();
    tab.push(dots.clone());
    lines.iter().for_each(|ln|{
        //let mut chr_ln:Vec<char> = Vec::new();
        let mut ln1:String = String::new();
        ln1.push_str(" ");
        ln1.push_str(ln);
        ln1.push(' ');
        let chr_ln = ln1.chars().map(|f| if '.' == f {' '} else {f}) .collect();
        tab.push(chr_ln);
    });
    tab.push(dots);

    let mut ok_nums:Vec<String> = Vec::new();

    for y in 0..tab.len() {
        let mut nmb = String::new();
        let mut ok = false;
        for x in 0..tab[0].len(){
            //print!("{}", tab[y][x]);
            if tab[y][x].is_ascii_digit(){
                nmb.push(tab[y][x]);
                ok |= check_adj(&tab, y, x);
            } else if !nmb.is_empty() {
                if ok {
                    println!("{}", nmb);
                    ok_nums.push(nmb.clone());
                }
                ok = false;
                nmb.clear();
            }
        }
        println!();
    }

    let res:i32 = ok_nums.iter().map(|n| i32::from_str_radix(n, 10).unwrap()).sum();
    println!("res: {}", res);

}

fn check_adj(tab: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if tab[y+1][x+1].is_ascii_punctuation() { return true;}
    if tab[y+1][x].is_ascii_punctuation() { return true;}
    if tab[y+1][x-1].is_ascii_punctuation() { return true;}
    if tab[y][x+1].is_ascii_punctuation() { return true;}
    if tab[y][x-1].is_ascii_punctuation() { return true;}
    if tab[y-1][x+1].is_ascii_punctuation() { return true;}
    if tab[y-1][x].is_ascii_punctuation() { return true;}
    if tab[y-1][x-1].is_ascii_punctuation() { return true;}

    return false;
}

pub fn day_3_part2() {
    let file = File::open("input/input3_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut dots:Vec<char> = Vec::new();
    for _ in 0..lines[0].len() + 2 {
        dots.push(' ');
    }

    let mut tab:Vec<Vec<char>> = Vec::new();
    tab.push(dots.clone());
    lines.iter().for_each(|ln|{
        //let mut chr_ln:Vec<char> = Vec::new();
        let mut ln1:String = String::new();
        ln1.push_str(" ");
        ln1.push_str(ln);
        ln1.push(' ');
        let chr_ln = ln1.chars().map(|f| if '.' == f {' '} else {f}) .collect();
        tab.push(chr_ln);
    });
    tab.push(dots);

    let mut gear_id = 0;
    let mut gear2pos:HashMap<(usize,usize), i32> = HashMap::new();
    for y in 0..tab.len() {
        for x in 0..tab[0].len(){
            if tab[y][x] == '*' {
                for i in 0..=2 {
                    for j in 0..=2 {
                        gear2pos.insert((y+i-1, x+j-1), gear_id);
                    }    
                }
                gear_id += 1;
            }
        }
    }


    let mut ok_nums:HashMap<i32, i32> = HashMap::new();

    let mut final_num:Vec<i32> = Vec::new(); 

    for y in 0..tab.len() {
        let mut nmb = String::new();
        let mut gear= -1;
        for x in 0..tab[0].len(){
            //print!("{}", tab[y][x]);
            if tab[y][x].is_ascii_digit(){
                nmb.push(tab[y][x]);
                if gear2pos.contains_key(&(y,x)){
                    gear = *gear2pos.get(&(y,x)).unwrap();
                }
            } else if !nmb.is_empty() {
                if gear != -1 {
                    println!("{}", nmb);
                    if ok_nums.contains_key(&gear){
                        let pom = ok_nums.get(&gear).unwrap();
                        //ok_nums.insert(gear, pom * i32::from_str_radix(&nmb, 10).unwrap());
                        final_num.push(pom * i32::from_str_radix(&nmb, 10).unwrap());
                    } else {
                        ok_nums.insert(gear, i32::from_str_radix(&nmb, 10).unwrap());
                    }
                }
                gear = -1;
                nmb.clear();
            }
        }
        println!();
    }

    let res:i32 = final_num.iter().sum();
    println!("res: {}", res);

}
