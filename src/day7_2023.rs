use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use once_cell::unsync::OnceCell;

pub fn day_7() {
    let file = File::open("input/input7_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut gms:Vec<(&str, i32)> = lines.iter().map(|ls| {
        let prsd:Vec<&str> = ls.split(" ").collect();
        let gm = (prsd[0], i32::from_str_radix(prsd[1], 10).unwrap());
        return gm;
    }).collect();

    //println!("{:?}", gms);

    gms.sort_by(|l,r| {
        get_hand_val(&l.0).cmp(&get_hand_val(&r.0))
    });

    let mut i = 0;
    let res:i32 = gms.iter().map(|gm| {i += 1; gm.1 * i}).sum();
    println!("part 1: {}", res);

    gms.sort_by(|l,r| {
        unsafe{get_jokered_val(&l.0).cmp(&get_jokered_val(&r.0))}
    });

    //println!("{:?}", gms);

    let mut i = 0;
    let res:i32 = gms.iter().map(|gm| {i += 1; gm.1 * i}).sum();
    println!("part 2: {}", res);
}

fn get_card_val(c:&char) -> i32 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0
    }
}

fn get_jokered_card_val(c:&char) -> i32 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0
    }
}

fn get_hand_val(hnd0:&str) -> i32 {
    let mut p = 1;
    let mut res = hnd0.chars().rev().map(|c| {let r = get_card_val(&c)*p; p*=14; r}).sum();
    res += 1000000 * get_hand_type(hnd0);

    //println!("{} => {}", hnd0, res);
    res
}

fn get_hand_type(hnd:&str)->i32{
    let mut res = 0;
    let mut hsrt = hnd.chars().collect::<Vec<char>>();
    hsrt.sort();    
    if hsrt[0] == hsrt[1] && hsrt[1] == hsrt[2] && hsrt[2] == hsrt[3] && hsrt[3] == hsrt[4] {
        res = 10;
    } else if hsrt[0] == hsrt[1] && hsrt[1] == hsrt[2] && hsrt[2] == hsrt[3] ||
    hsrt[1] == hsrt[2] && hsrt[2] == hsrt[3] && hsrt[3] == hsrt[4] {
        res = 9;
    } else //full
    if hsrt[0] == hsrt[1] && hsrt[2] == hsrt[3] && hsrt[3] == hsrt[4] ||
    hsrt[0] == hsrt[1] && hsrt[1] == hsrt[2] && hsrt[3] == hsrt[4] {
        res = 8;
    } else //3 of a kind
    if hsrt[0] == hsrt[1] && hsrt[1] == hsrt[2] ||
    hsrt[1] == hsrt[2] && hsrt[2] == hsrt[3]|| 
    hsrt[2] == hsrt[3] && hsrt[3] == hsrt[4]{
        res = 7;
    } else //2 pairs
    if hsrt[0] == hsrt[1] && hsrt[2] == hsrt[3] ||
    hsrt[0] == hsrt[1] && hsrt[3] == hsrt[4]|| 
    hsrt[1] == hsrt[2] && hsrt[3] == hsrt[4]{
        res = 6;
    } else //1 pair
    if hsrt[0] == hsrt[1] || hsrt[1] == hsrt[2] ||
    hsrt[2] == hsrt[3] || hsrt[3] == hsrt[4]{
        res = 5;
    }  else //high card
    if hsrt[0] != hsrt[1] && hsrt[1] != hsrt[2] &&
    hsrt[2] != hsrt[3] && hsrt[3] != hsrt[4]{
        res = 4;
    } else {
    }
    res
}

static mut CACHE:OnceCell<HashMap<String,i32>> = OnceCell::new();

unsafe fn get_jokered_val(hnd:&str) -> i32{
    let cache = CACHE.get_or_init(|| HashMap::new());

    if cache.contains_key(hnd) {
        //print!("*");
        return *cache.get(hnd).unwrap();
    }

    let mut res = 0;
    if hnd.contains("J") {
        let crds = ["2", "3", "4", "5", "6", "7", "8", "9", "A", "K", "Q", "T"];
        crds.iter().for_each(|c| {
            let pom = hnd.replace("J", c);
            res = res.max(get_jokered_val(pom.as_str()) / 1000000);
        });
        res *= 1000000;
        let mut p = 1;
        res += hnd.chars().rev().map(|c| {let r = get_jokered_card_val(&c)*p; p*=14; r}).sum::<i32>();
    } else {
        let mut p = 1;
        res = hnd.chars().rev().map(|c| {let r = get_jokered_card_val(&c)*p; p*=14; r}).sum();
        res += get_hand_type(hnd) * 1000000;
    }

    CACHE.get_mut().unwrap().insert(hnd.to_string(), res);

    res
}

// part 2
//249847755 too high
//249776650
#[test]
fn test_hand_val(){
    let four1 = unsafe {get_jokered_val("JKKK2")};    
    let four2 = unsafe {get_jokered_val("QQQQ2")};    
    assert!(four1 < four2, "four: {} {}", four1, four2);
    let mut sorted = ["QQQQ2","JKKK2"];
    sorted.sort_by(|l,r| {
        unsafe{get_jokered_val(&l).cmp(&get_jokered_val(&r))}
    });
    println!("{:?}", sorted);
}