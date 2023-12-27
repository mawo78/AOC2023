use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

pub fn day_17() {
    let file = File::open("input/input17_2023.txt").unwrap();
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

    let endypos = map.len() as i32 - 2;
    let endxpos = map[0].len() as i32 - 2;

    println!("Part 1: {}", find_way(&map, (1,1), (endypos,endxpos), 1, 3));
    println!("Part 2: {}", find_way(&map, (1,1), (endypos,endxpos), 4, 10));
}

type NodeType = (i32,i32,(i32, i32),char);

fn find_way(map:&Vec<Vec<char>>, pos: (i32, i32), dest: (i32, i32), minstep:i32, maxstep:i32) -> i32 {
    let mut res = 0;
    let mut todo:Vec<NodeType> = Vec::new();
    todo.push((0, 0, pos, 'R'));
    todo.push((0, 0, pos, 'D'));
    let mut vis:HashSet<((i32,i32), char)> = HashSet::new();
    let mut x = 0;

    while !todo.is_empty() {
        let mut min_el_i = 0;
        for i in 1..todo.len() {
            if f_w(&todo[i]) < f_w(&todo[min_el_i]) {
                min_el_i = i;
            }
        }
    
        let min_el = todo[min_el_i].clone();
    
        if dest == min_el.2 {
            res = min_el.0;
            break;
        }
        todo.remove(min_el_i);    
        if vis.contains(&(min_el.2, min_el.3)){
            continue;
        }
        vis.insert((min_el.2, min_el.3).clone());
        //println!("{:?}", min_el);
    
        let sides:Vec<((i32, i32),char)> = match min_el.3 {
            'L' => vec![ ((-1, 0), 'U'), ((1, 0), 'D')],
            'R' => vec![ ((-1, 0), 'U'), ((1, 0), 'D')],
            'U' => vec![ ((0, -1), 'L'), ((0, 1), 'R')],
            'D' => vec![ ((0, -1), 'L'), ((0, 1), 'R')],
            _ => vec![],
        };

        for d in sides{
            let mut pos = (min_el.2, d.1).clone();
            for i in 1..=maxstep {
                pos.0.0 += d.0.0;
                pos.0.1 += d.0.1;
                if is_inside(&map, pos.0){
                    if i >= minstep {
                        let h = heat_range(&map,&min_el.2, i, &d.0);
                        todo.push(( min_el.0 + h, x, pos.0, pos.1 ));
                        x += 1;    
                    }
                }
            }
        }
    }
    res
}

fn heat_range(map:&Vec<Vec<char>>, pos0: &(i32, i32), n: i32, d: &(i32, i32))->i32{
    let mut res = 0;
    let mut pos = pos0.clone();
    for _ in 0.. n {
        pos.0 += d.0;
        pos.1 += d.1;
        res += heat(&map, pos);
    }
    res
}

fn f_w(n:&NodeType)->i32{
    n.0 * 100000 + n.1
}

fn is_inside(map: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }
    if pos.0 > map.len() as i32 - 1 || pos.1 > map[0].len() as i32 - 1 {
        return false;
    }
    map[pos.0 as usize][pos.1 as usize] != '#'
}

fn heat(map: &[Vec<char>], pos: (i32, i32)) -> i32 {
    map[pos.0 as usize][pos.1 as usize] as i32 - '0' as i32
}