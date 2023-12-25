use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}, borrow::BorrowMut};

pub fn day_25() {
    let file = File::open("input/input25_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut graf :Vec<Vec<usize>> = Vec::new();
    let mut name2id:HashMap<String, usize> = HashMap::new();
    let mut id2name:HashMap<usize, String> = HashMap::new();
    lines.iter().for_each(|ln| {
        // jqt: rhn xhk nvd
        let psd:Vec<_> = ln.split(":").collect();

        if !name2id.contains_key(psd[0]) {
            let id = graf.len();
            graf.push(Vec::new());
            name2id.insert(psd[0].to_string(), id);
            id2name.insert(id, psd[0].to_string());
        }

        psd[1].trim().split(" ").for_each(|k|{
            if !name2id.contains_key(k) {
                let id = graf.len();
                graf.push(Vec::new());
                name2id.insert(k.to_string(), id);
                id2name.insert(id, k.to_string());
        
            }
        });
    });

    println!("{:?}", &name2id);

    lines.iter().for_each(|ln| {
        // jqt: rhn xhk nvd
        let psd:Vec<_> = ln.split(":").collect();
        psd[1].trim().split(" ").for_each(|k|{
            make_edge(&mut graf, psd[0], k, &name2id);
            make_edge(&mut graf, k, psd[0], &name2id);
        });
    });

    graf.iter().for_each(|k| {println!("{:?} {}", k, k.len())});
    println!("{:?}", &graf);
    tarjan(&graf);

    // remove_edge(&mut graf, "hfx", "pzl", &name2id);
    remove_edge(&mut graf, "pzl", "hfx", &name2id);
    remove_edge(&mut graf, "bvb", "cmg", &name2id);
    remove_edge(&mut graf, "cmg", "bvb", &name2id);
    remove_edge(&mut graf, "nvd", "jqt", &name2id);
    remove_edge(&mut graf, "jqt", "nvd", &name2id);

    graf.iter().for_each(|k| {println!("{:?} {}", k, k.len())});
    println!("{:?}", &graf);
    tarjan(&graf);


}

fn make_edge(graf: &mut Vec<Vec<usize>>, start:&str, cur:&str, name2id:&HashMap<String, usize>) {
    println!("{:?} -> {:?}", start, cur);
    let start_id = name2id.get(start).unwrap();
    let end_id = name2id.get(cur).unwrap();
    make_edge_id(graf, start_id, end_id);
}

fn make_edge_id(graf: &mut Vec<Vec<usize>>, start:&usize, cur:&usize) {
    graf[*start].push(*cur);
}

fn remove_edge(graf: &mut Vec<Vec<usize>>, start:&str, cur:&str, name2id:&HashMap<String, usize>){
    println!("{:?} -/-> {:?}", start, cur);
    let start_id = name2id.get(start).unwrap();
    let end_id = name2id.get(cur).unwrap();
    remove_edge_id(graf, start_id, end_id);
}

fn remove_edge_id(graf: &mut Vec<Vec<usize>>, start:&usize, cur:&usize){
    println!("{:?} -/-> {:?}", start, cur);
    let pos = graf[*start].iter().position(|x| x == cur).unwrap();
    graf[*start].remove(pos);
}

fn dfs(u:usize, graf: &Vec<Vec<usize>>, vis:&mut HashSet<usize>, inn:&mut HashMap<usize, i32>,
    low:&mut HashMap<usize, i32>, st:&mut Vec<usize>, cnt:&mut i32, sccs:&mut i32){
    inn.insert(u, *cnt);
    low.insert(u, *cnt);
    *cnt += 1;
    vis.insert(u);
    st.push(u);

    for v in &graf[u] {
        if !vis.contains(v) {
            dfs(*v, graf, vis, inn, low, st, cnt, sccs);
            let u_val = low.get(&u).unwrap_or(&i32::MAX);
            let v_val = low.get(&v).unwrap_or(&i32::MAX);
            low.insert(u, *u_val.min(v_val));
        } else {
            let u_val = low.get(&u).unwrap_or(&i32::MAX);
            let v_in = inn.get(&v).unwrap_or(&i32::MAX);
            low.insert(u, *u_val.min(v_in));
        }        
    }
    let u_val = low.get(&u).unwrap_or(&i32::MAX);
    let u_in = inn.get(&u).unwrap_or(&i32::MAX);
    if u_val == u_in {
        println!("{} -> {}", u, u);
        *sccs += 1;
        //let mut x = 0;
        loop {
            let x = st.pop().unwrap();
            inn.insert(x, i32::MAX);
            if x == u {
                break;
            }
        } 
    }
}

fn tarjan(graf: &Vec<Vec<usize>>){
    let mut vis:HashSet<usize> = HashSet::new();
    let mut inn:HashMap<usize, i32> = HashMap::new();
    let mut low:HashMap<usize, i32> = HashMap::new();
    let mut st:Vec<usize> = Vec::new();
    let mut cnt:i32 = 0;
    let mut sccs:i32 = 0;

    for v in 0..graf.len() {
        if !vis.contains(&v) {
            dfs(v, graf, &mut vis, &mut inn, &mut low, &mut st, &mut cnt, &mut sccs);
        }
    }

    println!("{} {}", sccs, cnt);
}