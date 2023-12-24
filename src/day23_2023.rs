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

    let mut graf:HashMap<(i32,i32), HashMap<(i32,i32), i32>> = HashMap::new(); 
    display_map(&map, &graf);

    let costs = req(&start, &map, &stop, &mut graf);
    display_map(&map, &graf);

    println!("{:?}", graf);

    println!("Part1: {}", costs);

    let mut vis :HashSet<(i32, i32)> = HashSet::new();
    vis.insert(start.clone());
    let res2 = req2(&start, &map, &stop, &graf, &mut vis);
    println!("Part2: {}", res2);

}

// part 2:
// 6480 too high
// 6477 too high

fn req2(start: &(i32,i32), map: &Vec<Vec<char>>, stop: &(i32,i32), 
graf:&HashMap<(i32,i32), HashMap<(i32,i32), i32>>, vis:&mut HashSet<(i32,i32)>) -> i32 {
    if start == stop {
        return 0;
    }
    let dests = graf.get(&start).unwrap();
    let mut maxw = -100000;
    for dest in dests{
        let pkt = dest.0;
        let val = dest.1;
        if !vis.contains(pkt){
            vis.insert(pkt.clone());
            maxw = maxw.max(val + req2(&pkt, map, stop, graf, vis));
            vis.remove(&pkt);
        }
    }
    maxw
}

fn req(start: &(i32,i32), map: &Vec<Vec<char>>, stop: &(i32,i32), 
    graf:&mut HashMap<(i32,i32), HashMap<(i32,i32), i32>>) -> i32 {

    let mut costs:HashMap<(i32,i32), i32> = HashMap::new();
    costs.insert(start.clone(), 0);
    let mut qq:VecDeque<(i32, i32)> = VecDeque::new();
    qq.push_back(start.clone());
    let mut curcost = 0;
    let mut vis:HashSet<(i32,i32)> = HashSet::new();

    while !qq.is_empty(){
        let mut cur = qq.pop_front().unwrap();
        vis.insert(cur.clone());
        // if &cur == stop {
        //     //println!("{}", costs.get(&stop).unwrap());
        //     return curcost;
        //     //break;
        // }

        let rght = map[cur.0 as usize][(cur.1 + 1) as usize] == '>';
        let dwn = map[(cur.0 + 1) as usize][cur.1 as usize] == 'v';
        if rght || dwn || &cur == stop{
            let mut rcost = 0;
            if rght {
                rcost = rcost.max(req(&(cur.0, cur.1 + 1), &map, stop, graf));
            }
            if  dwn {
                rcost = rcost.max(req(&(cur.0 + 1, cur.1), &map, stop, graf));
            }
            let mut res = curcost + rcost;
            if rght || dwn {res += 1;}

            println!("{:?} -> {:?}", start, &cur);
            let start_n = move_slp(&start, &map, &mut curcost);
            let empt: HashMap<(i32,i32),i32> = HashMap::new();
            if !graf.contains_key(&start_n) {
                graf.insert(start_n.clone(), empt);
            }
            graf.get_mut(&start_n).unwrap().insert(cur.clone(), curcost);

            let empt: HashMap<(i32,i32),i32> = HashMap::new();
            if !graf.contains_key(&cur) {
                graf.insert(cur.clone(), empt);
            }
            graf.get_mut(&cur).unwrap().insert(start_n.clone(), curcost);

            return res;
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

fn move_slp(p: &(i32, i32), map: &Vec<Vec<char>>, curcost: &mut i32) -> (i32, i32) {
    if map[p.0 as usize][p.1 as usize] == '>' {
        *curcost += 1;
        return (p.0, p.1 - 1);
    }
    if map[p.0 as usize][p.1 as usize] == 'v' {
        *curcost += 1;
        return (p.0 - 1, p.1);
    }

    // if map[(p.0 + 1) as usize][p.1 as usize] == 'v' && map[(p.0 + 3) as usize][p.1 as usize] == 'v'{
    //     return (p.0+2, p.1);
    // }
    // if map[p.0 as usize][(p.1 + 1) as usize] == '>' && map[p.0 as usize][(p.1 + 3) as usize] == '>'{
    //     return (p.0, p.1+2);
    // }
    p.clone()
}

fn is_inside(map: &[Vec<char>], pos: (i32, i32)) -> bool {
    map[pos.0 as usize][pos.1 as usize] != '#'
}

fn display_map(map: &Vec<Vec<char>>, graf:&HashMap<(i32,i32), HashMap<(i32,i32), i32>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if graf.contains_key(&(y as i32,x as i32)){
                print!("O");
            } else {
                print!("{}", &map[y][x]);
            }
        }
        println!();    
    }
    //println!();    
}
