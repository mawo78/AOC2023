use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet, time::Instant};

pub fn day_11() {
    let start = Instant::now();

    let file = File::open("input/input11_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let map:Vec<Vec<char>> = lines.iter().map(|ln| {
        let pom:Vec<char> = ln.chars().collect();
        pom
    }).collect();

    let mut gals:Vec<(usize, usize)> = Vec::new();
    let mut rows:HashSet<usize> = HashSet::from_iter(0..map.len());
    let mut cols:HashSet<usize> = HashSet::from_iter(0..map[0].len());

    let mut i = 0;
    map.iter().for_each(|rw| { 
        let mut j = 0;
        rw.iter().for_each(|p| {
            if '#' == *p {
                gals.push((i, j));
                rows.remove(&i);
                cols.remove(&j);
            }
            j += 1;
        });
        i += 1;
    });

    let mut res = 0;
    let mut res2 = 0;
    for i in 0..gals.len() - 1 {
        let g1 = gals[i];
        for j in i+1..gals.len() {
            let g2 = gals[j];
            let d = (g2.0.abs_diff(g1.0) + g2.1.abs_diff(g1.1)) as u64;
            let dx = (rows.iter().filter(|x| g1.0.min(g2.0) < **x && **x < g1.0.max(g2.0)).count()) as u64;
            let dy = (cols.iter().filter(|x| g1.1.min(g2.1) < **x && **x < g1.1.max(g2.1)).count()) as u64;
            res += d + dx + dy;
            res2 += d + (dx + dy)  * 999999;
            //println!("{:?} - {:?}  d = {}", g1, g2, d);
        }
    }
    let duration = start.elapsed();

    println!("Time elapsed in calculation is: {:?}", duration);

    println!("{}", res);
    println!("{}", res2);
}
