use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

pub fn day_16() {
    let file = File::open("input/input16_2020.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut i = 0;
    let mut correct = vec![0; 1000];

    while i < lines.len() && !lines[i].is_empty(){
        //departure location: 32-842 or 854-967
        let spl:Vec<_> = lines[i].split(": ").collect();
        let spl1:Vec<_> = spl[1].split(" or ").collect();
        let spl2:Vec<_> = spl1[0].split("-").collect();
        for j in usize::from_str_radix(spl2[0], 10).unwrap()..=usize::from_str_radix(spl2[1], 10).unwrap(){
            correct[j] = 1;
        }
        let spl3:Vec<_> = spl1[1].split("-").collect();
        for j in usize::from_str_radix(spl3[0], 10).unwrap()..=usize::from_str_radix(spl3[1], 10).unwrap(){
            correct[j] = 1;
        }
        i += 1;
    }

    i+=1;
    while i < lines.len() && !lines[i].is_empty(){
        i+=1;
    }

    let mut sum = 0;
    i+=2;
    while i < lines.len(){
        // if lines[i].starts_with("nearby"){
        //     i += 1;
        //     continue;
        // }

        lines[i].split(",").map(|x| usize::from_str_radix(x, 10).unwrap()).for_each(|n|{
            if correct[n] == 0 {
                sum += n;
                print!("{} ",n);
            }
        });
        println!();
        i+=1;
    }

    println!("Part 1: {}", sum);

}

pub fn day_16_part2() {
    let file = File::open("input/input16_2020.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut i = 0;
    //let mut correct = vec![0; 1000];
    let mut correct_map:HashMap<String, Vec<i32>> = HashMap::new();

    while i < lines.len() && !lines[i].is_empty(){
        //departure location: 32-842 or 854-967
        let spl:Vec<_> = lines[i].split(": ").collect();
        let name = spl[0];
        let spl1:Vec<_> = spl[1].split(" or ").collect();
        let spl2:Vec<_> = spl1[0].split("-").collect();
        let mut correct = vec![0; 1000];
        for j in usize::from_str_radix(spl2[0], 10).unwrap()..=usize::from_str_radix(spl2[1], 10).unwrap(){
            correct[j] = 1;
        }

        let spl3:Vec<_> = spl1[1].split("-").collect();
        for j in usize::from_str_radix(spl3[0], 10).unwrap()..=usize::from_str_radix(spl3[1], 10).unwrap(){
            correct[j] = 1;
        }
        correct_map.insert(name.to_string(), correct);
        i += 1;
    }

    i+=2;
    // my ticket
    let mut my_ticket: Vec<usize>=Vec::new();
    while i < lines.len() && !lines[i].is_empty(){
        my_ticket = lines[i].split(",").map(|x| usize::from_str_radix(x, 10).unwrap()).collect();
        i+=1;
    }

    i+=2;
    let mut near_tickets: Vec<Vec<usize>> = Vec::new();
    while i < lines.len(){
        let mut pom:Vec<usize> = Vec::new();
        let mut bad_tckt = false;
        lines[i].split(",").map(|x| usize::from_str_radix(x, 10).unwrap()).for_each(|n|{
            
            if correct_map.iter().any(|x|x.1[n] == 1) {
                pom.push(n);
            }
            else {
                bad_tckt = true;
            }
        });
        if !bad_tckt{
            near_tickets.push(pom);
        }
        i+=1;
    }

    let mut col_names:HashMap<usize, String> = HashMap::new();
    let mut fnd_col_nms:HashSet<String> = HashSet::new();

    while fnd_col_nms.len() != correct_map.keys().len() {
        for col in 0..near_tickets[0].len(){
            let mut colnms:HashSet<&String> = correct_map.keys().collect();
            fnd_col_nms.iter().for_each(|x| {colnms.remove(x);});

            near_tickets.iter().for_each(|v| {
                correct_map.iter().for_each(|n|{
                    if n.1[v[col]] == 0 {
                        colnms.remove(n.0);
                    }
                });
            });
            println!("{:?}", colnms);
            if colnms.len() == 1 {
                let name = colnms.iter().collect::<Vec<&&String>>()[0];
                col_names.insert( col, (*name).clone());
                fnd_col_nms.insert((*name).clone());
                println!("{} {}", col, name);
            }
        }
    }

    println!("{:?}", col_names);

    let res = my_ticket.iter().enumerate().map(|x| {
        if col_names.get(&x.0).unwrap().starts_with("departure") {
            x.1
        } else {
            &1
        }
    }).product::<usize>();
    println!("Part 2: {}", res);

}
