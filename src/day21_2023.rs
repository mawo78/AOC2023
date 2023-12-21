use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet}};

pub fn day_21() {
    let file = File::open("input/input21_2023.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    //lines.insert(0, "#".repeat(lines[0].len()));
    //lines.push("#".repeat(lines[0].len()));

    let mut map:Vec<Vec<char>> = lines.iter().map(|ln| { 
        //let mut pom = "#".to_string();
        //pom.push_str(ln);
        //pom.push('#');
        //pom.chars().collect()
        ln.chars().collect()
    }).collect();

    let mut strt = (1_i32,1_i32);
    for i in 1..map[0].len()-1{
        for j in 1..map.len()-1{
            if map[j][i] == 'S'{
                strt = (j as i32, i as i32);
                map[j][i] = '.';
                break;
            }
        }
    }

    let mut stps:HashSet<(i32, i32)> = HashSet::new();
    stps.insert(strt);
    
    let mut cache:Vec<i64> = Vec::new(); 
    let mut wsp:Vec<i64> = Vec::new(); 

    for i in 1..330 {
        let mut new_stps:HashSet<(i32, i32)> = HashSet::new();
        for stp in &stps {
            for delta in [(0,1), (0, -1), (1,0),(-1,0)]{
                let mut stpl = stp.clone();
                stpl.0 = stpl.0 + delta.0;
                stpl.1 = stpl.1 + delta.1;
                if is_inside(&map, &stpl){
                    new_stps.insert(stpl);
                }    
            }
        }

        //display_map2(&map, &new_stps);
        cache.push(stps.len() as i64);
        stps = new_stps;
        if (i - 65) % 131 == 0 {
            wsp.push(stps.len() as i64);
            println!("{}", stps.len());
        }
    }

    println!("Part 1: {}", cache[64]);

    //26501365 = 2023 * 13100 + 65
    //ala day 9
    let dx1 = wsp[1] - wsp[0];
    let mut dx2 = wsp[2] - wsp[1];
    let ddx1 = dx2 - dx1;
    let mut res = wsp[2];
    for _ in 0..(2023*100 - 2) {
        dx2 += ddx1;
        res += dx2;
    }
    println!("Part 2: {}", res);

}
// part 2
// 620360897855063 too high
// 620342498983424 too low

fn display_map(map: &Vec<Vec<char>>, new_stps: &HashSet<(i32, i32)>) {
    for i in 1..map[0].len()-1{
        for j in 1..map.len()-1{
            if new_stps.contains(&(i as i32, j as i32)){
                print!("{}", &'O');
            } else {
                print!("{}", &map[i][j]);
            }
        }
        println!();
    }
    println!("-----------------------");
}

fn display_map2(map: &Vec<Vec<char>>, new_stps: &HashSet<(i32, i32)>) {
    for i in 0..map[0].len(){
        for j in 0..map.len(){
            if new_stps.contains(&(i as i32, j as i32)){
                print!("{}", &'O');
            } else {
                print!("{}", &map[i][j]);
            }
        }
        println!();
    }
    println!("-----------------------");
}

fn is_inside(map: &[Vec<char>], pos: &(i32, i32)) -> bool {
    let mut y = pos.0 % (map.len() as i32);
    let mut x = pos.1 % (map[0].len() as i32);
    while y < 0 {
        y += map.len() as i32;
    }
    while x < 0 {
        x += map[0].len() as i32;
    }
    map[y as usize][x as usize] != '#'
}

