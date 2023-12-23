use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap, VecDeque}};

pub fn day_23() {
    let file = File::open("input/input23_2023.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines.insert(0, "#".repeat(lines[0].len()));
    lines.push("#".repeat(lines[0].len()));

    let map:Vec<Vec<char>> = lines.iter().map(|ln| { 
        let mut pom = "#".to_string();
        pom.push_str(ln);
        pom.push('#');
        pom.chars().collect()
    }).collect();

    let startx = *map[1].iter().enumerate().filter(|en_x|{en_x.1 == &'.'}).map(|x| x.0).collect::<Vec<usize>>().first().unwrap(); 
    let stopx = *map[map.len() - 2].iter().enumerate().filter(|en_x|{en_x.1 == &'.'}).map(|x| x.0).collect::<Vec<usize>>().first().unwrap(); 

    let start = (1, startx as i32);
    let stop = ((map.len() - 2) as i32, stopx as i32);

    let costs = req(&start, &map, &stop);

    println!("Part1: {}", costs)
}

fn req(start: &(i32,i32), map: &Vec<Vec<char>>, stop: &(i32,i32)) -> i32 {

    let mut costs:HashMap<(i32,i32), i32> = HashMap::new();
    costs.insert(start.clone(), 0);
    let mut qq:VecDeque<(i32, i32)> = VecDeque::new();
    qq.push_back(start.clone());
    let mut curcost = 0;
    let mut vis:HashSet<(i32,i32)> = HashSet::new();

    while !qq.is_empty(){
        let cur = qq.pop_front().unwrap();
        vis.insert(cur.clone());
        //let curcost = *costs.get(&cur).unwrap_or(&0);
        if &cur == stop {
            println!("{}", costs.get(&stop).unwrap());
            return curcost;
            //break;
        }

        let rght = map[cur.0 as usize][(cur.1 + 1) as usize] == '>';
        let dwn = map[(cur.0 + 1) as usize][cur.1 as usize] == 'v';
        if rght || dwn {
            let mut rcost = 0;
            if rght {
                rcost = rcost.max(req(&(cur.0, cur.1 + 1), &map, stop));
            }
            if  dwn {
                rcost = rcost.max(req(&(cur.0 + 1, cur.1), &map, stop));
            }
            return curcost + 1 + rcost;
        }


        let mut dset:Vec<_> = vec![(1,0),(-1,0),(0,1),(0,-1)];
        if map[cur.0 as usize][cur.1 as usize] == '>' {
            dset = vec![(0,1)];
        } else if map[cur.0 as usize][cur.1 as usize] == 'v' {
            dset = vec![(1,0)];
        }
        for d in dset {
            let n = (cur.0 + d.0, cur.1 + d.1);
            let ncost = curcost + 1;
            if is_inside(&map, n) && !vis.contains(&n){
                //let prevcost = *costs.get(&n).unwrap_or(&0);
                //if ncost > prevcost || prevcost == 0 {
                    qq.push_back(n);
                    costs.insert(n, ncost);
               // }
            }
        }
        curcost += 1;
    }
    curcost
}

fn is_inside(map: &[Vec<char>], pos: (i32, i32)) -> bool {
    map[pos.0 as usize][pos.1 as usize] != '#'
}
