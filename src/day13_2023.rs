use std::{fs::File, io::{BufReader, BufRead}};

pub fn day_13() {
    let file = File::open("input/input13_2023.txt").unwrap();
    let mut lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut mp:Vec<&mut String> = Vec::new();
    let mut res1 = 0;
    let mut res2 = 0;
    lines.iter_mut().for_each(|ln| {
        if ln.is_empty() {
            let res0 = process_map(&mp);
            res1 += res0[0];

            let mut done = false;
            for x in 0..mp.len(){
                //let mut pom = String::new();
                for y in 0..mp[x].len(){
                    let ch = if mp[x].chars().nth(y).unwrap() == '.' {'#'} else {'.'};
                    mp[x].remove(y);
                    mp[x].insert(y, ch);

                    //println!("{:#?}", mp);

                    let res3 = process_map(&mp);
                    if res3.len() != 0 && res3 != res0 {
                        ///////res2 += res3;
                        //println!("{:#?}", mp);
                        if res3.len() == 1 {
                            res2 += res3[0];
                        } else if res3.len() == 2 {
                            if res0[0] == res3[0] {
                                res2 += res3[1];
                            } else {
                                res2 += res3[0];
                            }
                        } else {
                            !unreachable!();
                        }
                        //println!("({}, {}) {:?} -> {:?}", x+1, y+1, res0, res3);
                        done = true;
                        break;
                    }
                    let ch = if mp[x].chars().nth(y).unwrap() == '.' {'#'} else {'.'};
                    mp[x].remove(y);
                    mp[x].insert(y, ch);
                }
                if done {break;};
            }
            if !done {println!("{:#?} {}", mp, res0[0]);}
                    

            mp.clear();
        } else {
            mp.push(ln);
        }
    });
//    res += process_map(&mp);


    println!("part 1: {}", res1);
    println!("part 2: {}", res2);
}

fn process_map(mp:&Vec<&mut String>) -> Vec<i32> {
    let mut res:Vec<i32> = Vec::new();
    res = find_mrrs(mp).iter().map(|x| x * 100).collect();
    //if res != 0 { return res;}
    let mut flpd =  flip_mat(mp);
    //println!("{:?}", flpd);
    let mut flp_ref:Vec<&mut String> = Vec::new();
    flpd.iter_mut().for_each(|x| {
        flp_ref.push(x);
    });
    res.append(&mut find_mrrs(&flp_ref));
    res
}

fn flip_mat(mat:&Vec<&mut String>) -> Vec<String> {
    let chrmat:Vec<Vec<char>> = mat.iter().map(|ln| ln.chars().collect::<Vec<char>>()).collect();
    let mut res:Vec<String> = Vec::new();
    for x in 0..mat[0].len(){
        let mut pom = String::new();
        for y in 0..mat.len(){
            pom.push(chrmat[y][x]);
        }
        res.push(pom);
    }

    res
}

fn find_mrrs(mp: &[&mut String]) -> Vec<i32> {
    let mut res:Vec<i32> = Vec::new();
    let nums:Vec<i64> = mp.iter().map(|ln| {
        let mut w = 1<<(ln.len() - 1);
        ln.chars().map(|c| {
            let x = match c {
            '#' => w,
            '.' => 0,
            _ => unreachable!()
            };
            w >>= 1;
            x
        }).sum()
    }).collect();
    //println!("{:?}", nums);

    for i in 0..nums.len() {
        let mut x = 0;
        if i < nums.len() / 2 {
            for j in 0..i+1 {
                if nums[i - j] == nums[i + 1 + j] { x += 1} else { break; };
            }
            //println!("|{}, {}|>", x, i + 1);
            if i + 1 == x { res.push((i + 1) as i32);}
        } else {
            //println!("<>");
            for j in 0..(nums.len() - i) {
                if (i as i32 - 1 - j as i32) < 0 {break;}
                if nums[i + j] == nums[i - 1 - j] { x += 1} else { break; };
            }
            //println!("<{}>", x);
            if nums.len() - i == x { res.push(i as i32);}
        }
    }
    //println!("{}", res);
    res
}

// 11012 too low
// 18259 too low
// 29213 ok
//part 2:
// 27250 too low