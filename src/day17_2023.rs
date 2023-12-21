use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

type NodeType = ((i32, i32), (char, char, char));

pub fn day_17() {
    let file = File::open("input/input17_2023tst.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines.insert(0, "#".repeat(lines[0].len()));
    lines.push("#".repeat(lines[0].len()));

    let map:Vec<Vec<char>> = lines.iter().map(|ln| { 
        let mut pom = "#".to_string();
        pom.push_str(ln);
        pom.push('#');
        pom.chars().collect()
    }).collect();

    println!("{:?}", map);

    let endypos = map.len() as i32 - 2;
    let endxpos = map[0].len() as i32 - 2;

    println!("Part 1: {}", a_star(&map, (1,1), (endypos,endxpos)));
}

fn a_star(map:&Vec<Vec<char>>, pos: (i32, i32), dest: (i32, i32)) -> i32 {
    let mut res = 0;
    let cur = pos.clone();
    let mut g:HashMap<(i32, i32), i32> = HashMap::new();
    g.insert(cur.clone(), 0);
    let mut open_list:Vec<NodeType> = Vec::new();
    open_list.push((cur, ('#','#','#')));
    let mut closed_list:Vec<NodeType> = Vec::new();
    while !open_list.is_empty() {
        let mut min_el_i = 0;
        for i in 1..open_list.len() {
            if f_v(&open_list[i].0, &g, dest) < f_v(&open_list[min_el_i].0, &g, dest) {
                min_el_i = i;
            }
        }

        let min_el = open_list[min_el_i];

        if dest == min_el.0 {
            break;
        }
        closed_list.push(min_el.clone());
        open_list.remove(min_el_i);

        let mut pleft = min_el.clone();
        pleft.0.1 -= 1;
        gen_next_step('L', pleft, map, &mut open_list, &closed_list, &mut g, min_el);

        let mut pright = min_el.clone();
        pright.0.1 += 1;
        gen_next_step('R', pright, map, &mut open_list, &closed_list, &mut g, min_el);

        let mut pup = min_el.clone();
        pup.0.0 -= 1;
        gen_next_step('U', pup, map, &mut open_list, &closed_list, &mut g, min_el);

        let mut pdown = min_el.clone();
        pdown.0.0 += 1;
        gen_next_step('D', pdown, map, &mut open_list, &closed_list, &mut g, min_el);

        //println!("o {:?}", open_list);
        //println!("c {:?}", closed_list);
    }

    //println!("{:?}", g);
    for i in 1..map[0].len()-1{
        for j in 1..map.len()-1{
            print!("{:3}", &g.get(&(i as i32, j as i32)).unwrap_or(&999));
        }
        println!();
    }
    res = *g.get(&dest).unwrap();
    res
}

fn gen_next_step(step:char, mut pleft: NodeType, map: &Vec<Vec<char>>, 
open_list: &mut Vec<NodeType>, 
closed_list:&Vec<NodeType>,
g: &mut HashMap<(i32, i32), i32>, 
min_el: NodeType) {
    if is_valid(&pleft, &map, step) {
        move_steps(&mut pleft, step);
        let pleft_closed_i = find_el(&pleft, &*closed_list);
        if pleft_closed_i != usize::MAX {
            return;
        }

        let pleft_i = find_el(&pleft, &*open_list);
        let cur_h = g.get(&min_el.0).unwrap() + heat(map, pleft.0);
        if pleft_i == usize::MAX {
            open_list.push(pleft);
            if g.get(&pleft.0).unwrap_or(&10000_i32) > &cur_h {
                g.insert(pleft.0, cur_h);
            }
        } else {
            if g.get(&pleft.0).unwrap() > &cur_h {
                g.insert(pleft.0, cur_h);
                open_list[pleft_i].1 = pleft.1;
            } 
        }
    }
}

fn find_el(pleft: &NodeType, open_list: &[NodeType]) -> usize {
    let mut res = usize::MAX;
    for i in 0..open_list.len() {
        if pleft.0 == open_list[i].0 {
            res = i;
            break;
        }
    }
    res
}

fn find_el_ex(pleft: &NodeType, open_list: &[NodeType]) -> usize {
    let mut res = usize::MAX;
    for i in 0..open_list.len() {
        if pleft == &open_list[i]{
            res = i;
            break;
        }
    }
    res
}

fn f_v(x: &(i32, i32) , g: &HashMap<(i32, i32), i32>, dest: (i32, i32)) -> i32 {
    let g_x = g.get(&x);
    let g_xv = if g_x.is_none() {10000} else {*g_x.unwrap()};
    g_xv + ((dest.0 - x.0) + (dest.1 - x.1))
}

fn move_steps(pleft: &mut NodeType, stp: char) {
    pleft.1.0 = pleft.1.1;
    pleft.1.1 = pleft.1.2;
    pleft.1.2 = stp;
}

fn is_valid(pleft: &NodeType, map:&Vec<Vec<char>>, stp: char) -> bool {
    if !is_inside(&map, pleft.0){
        return false;
    }
    if pleft.1.0 == pleft.1.1 && pleft.1.1 == pleft.1.2 && stp == pleft.1.2 {
        return false;
    }
    match stp {
        'L' => if pleft.1.2 == 'R' { return  false;}
        'R' => if pleft.1.2 == 'L' { return  false;}
        'U' => if pleft.1.2 == 'D' { return  false;}
        'D' => if pleft.1.2 == 'U' { return  false;}
        _ => unreachable!()
    }
    true
}

#[test]
fn test_is_valid(){
    let map = vec![vec!['#','#','#','#'],vec!['#','1','2','#'],vec!['#','5','9','#'],vec!['#','#','#','#'],];

    let pnt : NodeType = ((1,1), ('#', '#', '#'));
    assert_eq!(true, is_valid(&pnt, &map, 'R'));

    let pnt : NodeType = ((1,1), ('R', 'R', 'R'));
    assert_eq!(false, is_valid(&pnt, &map, 'R'));

    let pnt : NodeType = ((1,1), ('R', 'R', 'U'));
    assert_eq!(true, is_valid(&pnt, &map, 'R'));

    let pnt : NodeType = ((1,2), ('R', 'R', 'R'));
    assert_eq!(false, is_valid(&pnt, &map, 'L'));

    let pnt : NodeType = ((1,2), ('R', 'R', 'U'));
    assert_eq!(false, is_valid(&pnt, &map, 'D'));

    let pnt : NodeType = ((1,3), ('R', 'R', 'R'));
    assert_eq!(false, is_valid(&pnt, &map, 'D'));

    assert_eq!(1, heat(&map, (1,1)));
    assert_eq!(9, heat(&map, (2,2)));

    a_star(&map, (1,1), (2,2));
}

fn is_inside(map: &[Vec<char>], pos: (i32, i32)) -> bool {
    map[pos.0 as usize][pos.1 as usize] != '#'
}

fn heat(map: &[Vec<char>], pos: (i32, i32)) -> i32 {
    map[pos.0 as usize][pos.1 as usize] as i32 - '0' as i32
}