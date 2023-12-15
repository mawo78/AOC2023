use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn day_14() {
    let file = File::open("input/input14_2023tst.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines.insert(0, "#".repeat(lines[0].len()));
    lines.push("#".repeat(lines[0].len()));

    let mut map:Vec<Vec<char>> = lines.iter().map(|ln| { 
        let mut pom = "#".to_string();
        pom.push_str(ln);
        pom.push('#');
        pom.chars().collect()
    }).collect();

    println!("Part 1:");
    slide_rocks(&mut map);
    total_load(&map);    

    //println!("{:#?}", map);
    let mut cache:HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    cache.insert(map.clone(), 0);

    for i in 1..200 {
        for _ in 0..4 {
            slide_rocks(&mut map);
            map = flip_mat(&map);    
        }
        //display_map(&map);
        //total_load(&map);
        if cache.contains_key(&map) {
            let m = i - cache.get(&map).unwrap();
            println!("step {} prev: {} m = {}", i, cache.get(&map).unwrap(), m);

            let cycle = 1000000000;
            let pos = (cycle - i) % m;

            for _ in 0..pos {
                for _ in 0..4 {
                    slide_rocks(&mut map);
                    map = flip_mat(&map);    
                }    
            }
            println!("Part :2");
            total_load(&map);
    
            break;
        }    
        cache.insert(map.clone(), i);
    }

}

fn total_load(map: &Vec<Vec<char>>) ->i32 {
    let mut val:i32 = map.len() as i32 - 1;
    let res:i32 = map.iter().map(|ln| {
        let sr:i32 = ln.iter().map(|c| if *c == 'O' {1} else {0}).sum::<i32>() * val;
        val -= 1;
        sr
    }).sum();
    println!("total load: {}", res);
    res
}

fn slide_rocks(map: &mut Vec<Vec<char>>) {
    for i in 1..map.len()-1{
        for j in 1..map[i].len()-1{
            if map[i][j] == 'O' {
                let mut ny:i32 = i as i32 - 1;
                while ny >= 0 && map[ny as usize][j] == '.' {
                    ny -= 1;
                }
                if ny >= 0 {
                    map[i][j] = '.';
                    map[ny as usize + 1][j] = 'O';    
                }
            }
        }

    }
}

fn display_map(map: &Vec<Vec<char>>) {
    map.iter().for_each(|ln|{
        let s:String = String::from_iter(ln.iter());
        println!("{}", s);
    });
    println!();
}

fn flip_mat(mat:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    //let chrmat:Vec<Vec<char>> = mat.iter().map(|ln| ln.chars().collect::<Vec<char>>()).collect();
    let mut res:Vec<Vec<char>> = Vec::new();
    for x in 0..mat[0].len(){
        let mut pom:Vec<char> = Vec::new();
        for y in 0..mat.len(){
            pom.push(mat[mat.len() - y - 1][x]);
        }
        res.push(pom);
    }

    res
}

// 96064 too high
// 96063 too high
//96061