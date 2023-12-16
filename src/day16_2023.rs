use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

pub fn day_16() {
    let file = File::open("input/input16_2023tst.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines.insert(0, "#".repeat(lines[0].len()));
    lines.push("#".repeat(lines[0].len()));

    let map:Vec<Vec<char>> = lines.iter().map(|ln| { 
        let mut pom = "#".to_string();
        pom.push_str(ln);
        pom.push('#');
        pom.chars().collect()
    }).collect();

    //println!("{:?}", map);

    let enrgzd = find_way(&map, (1, 1), (0, 1));
    println!("Part 1: {}", enrgzd);

    let mut max_ener = 0;
    for i in 1..map[0].len()-1{
        max_ener = max_ener.max(find_way(&map, (1, i as i32), (1, 0)));
    }
    for i in 1..map.len()-1{
        max_ener = max_ener.max(find_way(&map, (i as i32, 1), (0, 1)));
    }

    for i in 1..map[0].len()-1{
        max_ener = max_ener.max(find_way(&map, (map.len() as i32 -1, i as i32), (-1, 0)));
    }
    for i in 1..map.len()-1{
        max_ener = max_ener.max(find_way(&map, (i as i32, map[0].len() as i32 -1), (0, -1)));
    }

    println!("Part 2: {}", max_ener);
}

fn find_way(map: &Vec<Vec<char>>, pos0:(i32,i32), dir0:(i32,i32)) -> usize {
    let mut stk: Vec<((i32,i32), (i32,i32))> = Vec::new();
    let mut visd:HashSet<((i32,i32), (i32,i32))> = HashSet::new();
    let mut enrgzd:HashSet<(i32,i32)> = HashSet::new();
    stk.push((pos0,dir0));
    while !stk.is_empty() {
        let stt = stk.pop().unwrap();
        if visd.contains(&stt) {
            continue;
        }
        visd.insert(stt.clone());

        //println!("{:?}", &stt);
        let mut pos = stt.0.clone();
        let dir = stt.1;

        match map[pos.0 as usize][pos.1 as usize] {
            '.' => {
                enrgzd.insert(pos); 
                pos.0 += dir.0; pos.1 += dir.1; stk.push((pos, dir))
            },
            '#' => { },
            '|' => {
                enrgzd.insert(pos); 
                if dir == (0, 1) || dir == (0, -1) {
                    stk.push(((pos.0 + 1,pos.1), (1, 0)));
                    stk.push(((pos.0 - 1,pos.1), (-1, 0)));
                } else { pos.0 += dir.0; pos.1 += dir.1; stk.push((pos, dir))}
            },
            '-' => { 
                enrgzd.insert(pos); 
                if dir == (1, 0) || dir == (-1, 0) {
                    stk.push(((pos.0,pos.1+1), (0, 1)));
                    stk.push(((pos.0,pos.1-1), (0, -1)));
                } else { pos.0 += dir.0; pos.1 += dir.1; stk.push((pos, dir))}
            },
            '/' => { 
                enrgzd.insert(pos); 
                if dir == (1, 0) {
                    stk.push(((pos.0,pos.1-1), (0, -1)));
                } else if dir == (-1, 0) {
                    stk.push(((pos.0,pos.1+1), (0, 1)));
                } else if dir == (0, 1) {
                    stk.push(((pos.0-1,pos.1), (-1, 0)));
                } else {
                    stk.push(((pos.0+1,pos.1), (1, 0)));
                }
            },
            '\\' => { 
                enrgzd.insert(pos); 
                if dir == (1, 0) {
                    stk.push(((pos.0,pos.1+1), (0, 1)));
                } else if dir == (-1, 0) {
                    stk.push(((pos.0,pos.1-1), (0, -1)));
                } else if dir == (0, 1) {
                    stk.push(((pos.0+1,pos.1), (1, 0)));
                } else {
                    stk.push(((pos.0-1,pos.1), (-1, 0)));
                }
            },
            _ => unreachable!()
        }
    }
    enrgzd.len()
}