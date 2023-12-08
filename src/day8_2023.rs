use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn day_8_part1() {
    let file = File::open("input/input8_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let way = &lines[0];
    let mut mapa:HashMap<String, (&str, &str)> = HashMap::new();
    lines.iter().filter(|ln| ln.contains(" = ")).for_each(|ln| {
        //BPQ = (VXR, TLN)
        let prs1 = ln.split(" = ").collect::<Vec<&str>>();
        let prs2 = prs1[1].split(&['(', ')', ',', ' ']).collect::<Vec<&str>>();
        //println!("{:?}", prs2);
        mapa.insert(prs1[0].to_string(), (prs2[1], prs2[3]));
    });

    println!("part 1 steps: {}", find_way_from("AAA", way, &mapa));

    let mut minways:Vec<i32> = mapa.keys().filter(|k| k.chars().nth(2).unwrap() == 'A')
        .map(|start| find_way_from(start, way, &mapa)).collect();
    println!("{:?}", minways);

    let mut res:i64 = minways.pop().unwrap() as i64;
    minways.iter().for_each(|x|{
        res = (res * (*x as i64)) / gcd(res, *x as i64);
    });

    println!("part2 steps: {}", res)
}

fn gcd(l:i64, r:i64) ->i64{
    if r == 0 {
        l
    } else {
        gcd(r, l % r)
    }
}
//13133452426987

fn find_way_from(start:&str, way:&str, mapa:&HashMap<String, (&str, &str)>) -> i32 {
    let mut curr = start;
    let mut itr = way.chars().cycle();
    let mut step = 0;
    while curr.chars().nth(2).unwrap() != 'Z' {
        let x = itr.next().unwrap();
        step += 1;
        curr = match x {
            'L' => mapa.get(curr).unwrap().0,
            'R' => mapa.get(curr).unwrap().1,
            _ => unreachable!(),
        }         
    }
    step
}