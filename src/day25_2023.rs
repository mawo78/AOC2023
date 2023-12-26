use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap, VecDeque}, borrow::BorrowMut, time::Instant, process::exit};

pub fn day_25() {
    let file = File::open("input/input25_2023.txt").unwrap();
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

    //println!("{:?}", &name2id);

    lines.iter().for_each(|ln| {
        // jqt: rhn xhk nvd
        let psd:Vec<_> = ln.split(":").collect();
        psd[1].trim().split(" ").for_each(|k|{
            make_edge(&mut graf, psd[0], k, &name2id);
            //make_edge(&mut graf, k, psd[0], &name2id);
        });
    });

    //graf.iter().for_each(|k| {println!("{:?} {}", k, k.len())});
    //println!("{:?}", &graf);

    let mut trav:HashMap<(usize, usize), i32> = HashMap::new(); 
    for v in 0..graf.len(){
        let mut vis:HashSet<usize> = HashSet::new();
        bfs(v, &graf, &mut trav, &mut vis);
    }

    let mut tsort:Vec<(&(usize, usize), &i32)> = trav.iter().collect();
    tsort.sort_by(|a,b| b.1.cmp(a.1));
    //println!("{:?}", tsort);

    // remove_edge(&mut graf, "hfx", "pzl", &name2id);
    // remove_edge(&mut graf, "bvb", "cmg", &name2id);
    // remove_edge(&mut graf, "nvd", "jqt", &name2id);

    // graf.iter().for_each(|k| {println!("{:?} {}", k, k.len())});
    // println!("{:?}", &graf);

    for i in 0..200 {
        let start = Instant::now();
        println!("{}", i);
        let edge = tsort[i].0;
        remove_edge_id(&mut graf, &edge.0, &edge.1);
        for j in i+1..201 {
            let edge2 = tsort[j].0;
            remove_edge_id(&mut graf, &edge2.0, &edge2.1);
            for k in j+1..202 {
                let edge3 = tsort[k].0;
                remove_edge_id(&mut graf, &edge3.0, &edge3.1);
                
                let ctd:Vec<i32> = find_ctd(&graf); 
                if ctd.len() == 2 {
                    println!("{:?} {}", ctd, ctd[0]*ctd[1]);
                    exit(0);
                }

                make_edge_id(&mut graf, &edge3.0, &edge3.1);
            }            
            make_edge_id(&mut graf, &edge2.0, &edge2.1);
        }
        make_edge_id(&mut graf, &edge.0, &edge.1);
        let duration = start.elapsed();
        println!("Time elapsed in calculation is: {:?}", duration);
    }

}

fn find_ctd(graf: &Vec<Vec<usize>>) -> Vec<i32> {
    let mut vis:HashSet<usize> = HashSet::new();
    let mut res:Vec<i32> = Vec::new();
    let mut trav:HashMap<(usize, usize), i32> = HashMap::new(); 
    for v in 0..graf.len(){
        if !vis.contains(&v){
            bfs(v, &graf, &mut trav, &mut vis);
            if res.is_empty(){
                res.push(vis.len() as i32);
            } else {
                let n = (vis.len() as i32) - res.last().unwrap();
                res.push(n);
            }
        }
    }
    res
}

fn bfs(v: usize, graf: &Vec<Vec<usize>>, trav: &mut HashMap<(usize, usize), i32>, vis:&mut HashSet<usize>)  {
    let mut qq:VecDeque<usize> = VecDeque::new();
    qq.push_back(v);
    while !qq.is_empty() {
        let u = qq.pop_front().unwrap();
        vis.insert(u);
        for w in &graf[u]{
            if !vis.contains(&w){
                let k = (u.min(*w), u.max(*w));
                let val = trav.get(&k).unwrap_or(&0);
                trav.insert(k, val + 1);
                qq.push_back(*w);
            }
        }
    }
}

fn make_edge(graf: &mut Vec<Vec<usize>>, start:&str, cur:&str, name2id:&HashMap<String, usize>) {
    //println!("{:?} -> {:?}", start, cur);
    let start_id = name2id.get(start).unwrap();
    let end_id = name2id.get(cur).unwrap();
    make_edge_id(graf, start_id, end_id);
}

fn make_edge_id(graf: &mut Vec<Vec<usize>>, start:&usize, cur:&usize) {
    graf[*start].push(*cur);
    graf[*cur].push(*start);
}

fn remove_edge(graf: &mut Vec<Vec<usize>>, start:&str, cur:&str, name2id:&HashMap<String, usize>){
    //println!("{:?} -/-> {:?}", start, cur);
    let start_id = name2id.get(start).unwrap();
    let end_id = name2id.get(cur).unwrap();
    remove_edge_id(graf, start_id, end_id);
}

fn remove_edge_id(graf: &mut Vec<Vec<usize>>, start:&usize, cur:&usize){
    //println!("{:?} -/-> {:?}", start, cur);
    let pos = graf[*start].iter().position(|x| x == cur).unwrap();
    graf[*start].remove(pos);
    let pos = graf[*cur].iter().position(|x| x == start).unwrap();
    graf[*cur].remove(pos);
}
