use std::{fs::File, io::{BufReader, BufRead}, cmp::Ordering};

pub fn day_10() {
    let file = File::open("input/input10_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut map:Vec<Vec<char>> = lines.iter().map(|ln| {
        let mut pom:Vec<char> = ln.chars().collect();
        pom.insert(0, '.');
        pom.push('.');
        pom
    }).collect();
    let mut dots_row:Vec<_> = lines[0].chars().map(|_| '.').collect();
    dots_row.insert(0, '.');
    dots_row.push('.');
    map.insert(0, dots_row.clone());
    map.push(dots_row.clone());
    //println!("{:?}", map);

    let s_row_str_vec:Vec<_> = lines.iter().filter(|ln| ln.contains('S')).collect();
    let s_row_str = s_row_str_vec.first().unwrap();
    // +1 for '.' in border
    let mut y = 1 + lines.iter().position(|ln| ln.cmp(s_row_str) == Ordering::Equal).unwrap();
    let mut x = 1 + s_row_str.chars().position(|c| c == 'S').unwrap();
    println!("s = {} {}", x, y);

    let mut map2:Vec<_> = map.iter().map(|_x| dots_row.clone()).collect();

    let mut dirc:(i32,i32) = (-1, 0);
    let mut step  = 0;
    loop {
        let c = map[y][x];
        map2[y][x] = c;
        let rght = ((y as i32 + dirc.1) as usize, (x as i32 -dirc.0) as usize);
        if map2[rght.0][rght.1] == '.' {
            map2[rght.0][rght.1] = 'I';
        }
        match c {
            'S' => {
                if step > 0 {                    
                    break;
                } else {
                    if map[y - 1][x] == '7' || map[y - 1][x] == '|'  || map[y - 1][x] == 'F'{
                        dirc = (-1, 0);
                    }
                    else if map[y + 1][x] == 'J' || map[y + 1][x] == '|' || map[y + 1][x] == 'L'{
                        dirc = (1, 0);
                    } 
                    else if map[y][x - 1] == 'L' || map[y][x - 1] == '-' || map[y][x - 1] == 'F'{
                        dirc = (0, -1);
                    } 
                    else if map[y][x + 1] == 'J' || map[y][x + 1] == '-' || map[y][x + 1] == '7'{
                        dirc = (0, 1);
                    } 
                }
            },
            '.' => {},
            '7' => {
                if dirc == (0, 1) {
                    dirc = (1, 0);
                } else if dirc == (-1, 0) {
                    dirc = (0, -1);
                }
            },
            'L' => {
                if dirc == (0, -1) {
                    dirc = (-1, 0);
                } else if dirc == (1, 0) {
                    dirc = (0, 1);
                }
            },
            'J' => {
                if dirc == (0, 1) {
                    dirc = (-1, 0);
                } else if dirc == (1, 0) {
                    dirc = (0, -1);
                }
            },
            'F' => {
                if dirc == (0, -1) {
                    dirc = (1, 0);
                } else if dirc == (-1, 0) {
                    dirc = (0, 1);
                }
            },
            '|' => {
            },
            '-' => {
            },
            _ => unreachable!(),
        }
        let rght = ((y as i32 + dirc.1) as usize, (x as i32 -dirc.0) as usize);
        if map2[rght.0][rght.1] == '.' {
            map2[rght.0][rght.1] = 'I';
        }

        x = (x as i32 + dirc.1) as usize;
        y = (y as i32 + dirc.0) as usize;
        step += 1;
    }

    println!("part 1: {}", step / 2);

    while flood(&mut map2) {        
    }

    // map2.iter().for_each(|ln| {
    //     println!("{:?}", String::from_iter(ln.iter()));
    // });

    let vert:i32 = map2.iter().map(|ln|{ln.iter().map(|c|{ if *c == 'I' {1} else {0} }).sum::<i32>() }).sum();

    println!("part 2: {}", vert); 
}

fn flood(map2:&mut Vec<Vec<char>>) -> bool {
    let mut fild = false;
    for y in 1..map2.len()-1 {
        let ln = &map2[y];
        for x in 1..ln.len()-1 {
            if map2[y][x] == '.' {
                if map2[y+1][x] == 'I' || map2[y][x+1] == 'I' || map2[y-1][x] == 'I' || map2[y][x-1] == 'I'{
                    map2[y][x] = 'I';
                    fild = true;
                }
            }
        }
    }
    fild
}