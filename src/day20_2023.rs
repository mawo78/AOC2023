use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap, VecDeque}};

struct Node {
    typ:char,
    state:bool,
    prev_inputs:HashMap<String,bool>,
}

pub fn day_20() {
    let file = File::open("input/input20_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut ins:HashMap<String, Vec<String>> = HashMap::new();
    let mut outs:HashMap<String, Vec<String>> = HashMap::new();
    let mut nodes:HashMap<String, Node> = HashMap::new();

    lines.iter().for_each(|ln| {
        // %dj -> zm, rd
        // &rd -> np, rp, fc
        // broadcaster -> hd, gs, fc, sx
        let psd:Vec<&str> = ln.split(" -> ").collect();
        if psd[0].starts_with("&") || psd[0].starts_with("%") {
            let name = psd[0].get(1..psd[0].len()).unwrap();
            parse_in_out(&psd, &mut outs, name, &mut ins);

            let n = Node {
                typ:psd[0].chars().next().unwrap(),
                state:false,
                prev_inputs:HashMap::new(),
            };
            nodes.insert(name.to_string(), n);

        } else {
            //broadcaster
            let name = psd[0];
            parse_in_out(&psd, &mut outs, name, &mut ins);
            let n = Node {
                typ:'b',
                state:false,
                prev_inputs:HashMap::new(),
            };
            nodes.insert(name.to_string(), n);
        }


    });

    //init to false
    nodes.iter_mut().for_each(|n|{
        let name = n.0;
        let node = n.1;
        if node.typ == '&' {
            println!("inputs for: {}", name);
            let inputs = ins.get(name).unwrap();
            println!("{:?}", &inputs);
            inputs.iter().for_each(|i| {
                node.prev_inputs.insert(i.clone(), false);
            });
        } else if node.typ == '%' {
            
        }
    });
    println!("------");

    let mut res = (0 ,0);
    let mut first_tms:HashMap<String, i64>  = HashMap::new();
    for iter in 0.. 10000 {
        let pom = push_button(&mut ins, &mut outs, &mut nodes, iter+1, &mut first_tms);
        if iter < 1000 {
            res.0 += pom.0;
            res.1 += pom.1;
        }

        for n in ["cl", "rp", "lb", "nj"] {
            let node = nodes.get(&n.to_string()).unwrap();
            if !node.prev_inputs.iter().all(|x| *x.1) {
                println!("1")
            } else {
                //print!("0")
            }
        }
        //println!();
    }

    println!("Part 1: {:?}, {}", res, res.0 * res.1);

    //
    //&cl -> lx|
    //&rp -> lx|
    //&lb -> lx-> &lx -> rx
    //&nj -> lx|

    println!("Part 2: {}", first_tms.values().product::<i64>());


}

fn push_button(ins:&mut HashMap<String, Vec<String>>, outs: &mut  HashMap<String, Vec<String>>, 
    nodes: &mut HashMap<String, Node>, iter:i32, first_tms:&mut HashMap<String, i64>) -> (i32, i32) {
    let mut res = (1, 0);
    let mut qq:VecDeque<(String, bool, String)> = VecDeque::new();
    qq.push_back(("button".to_string(), false, "broadcaster".to_string()));
    while !qq.is_empty() {
        let cmd = qq.pop_front().unwrap();
        let name_from = cmd.0;
        let sig = cmd.1;
        let name_to = cmd.2;
        if !nodes.contains_key(&name_to){
            //println!("no node  --{}--> {}", &sig, &name_to);
            continue;
        }
        let node:&mut Node = nodes.get_mut(&name_to).unwrap();

        if node.typ == '%' {
            if !sig {
                node.state = !node.state;
                let out = outs.get(&name_to).unwrap();
                out.iter().for_each(|n| {
                    qq.push_back((name_to.to_string(), node.state, n.clone()));
                    //println!("{} --{}--> {}", name_to, node.state, n);
                    if node.state {
                        res.1 += 1;
                    } else {
                        res.0 += 1;
                    }
                });
            }
        }
        else if node.typ == '&' {
            node.prev_inputs.insert(name_from.clone(), sig);
            let to_send = !node.prev_inputs.iter().all(|x| *x.1);
            let out = outs.get(&name_to).unwrap();

            if name_to == "lx" && sig {
                //println!("{} --- {}", &name_from, iter);
                let node_lx = nodes.get("lx").unwrap();
                if node_lx.prev_inputs.iter().any(|x| *x.1){
                    println!("{}", iter);
                    println!("{:?}", &node_lx.prev_inputs);
                    println!("{} --{}--> {}", name_from, sig, name_to);

                    if !first_tms.contains_key(&name_from) {
                        first_tms.insert(name_from, iter as i64);
                    }
                }
            }

            out.iter().for_each(|n| {
                qq.push_back((name_to.to_string(), to_send, n.clone()));
                //println!("{} --{}--> {}", name_to, to_send, n);
                if to_send {
                    res.1 += 1;
                } else {
                    res.0 += 1;
                }
            })
        }
        else if name_to == "broadcaster" {
            let out = outs.get(&name_to).unwrap();
            out.iter().for_each(|n| {
                qq.push_back((name_to.to_string(), false, n.clone()));
                //println!("{} --{}--> {}", name_to, false, n);
                res.0 += 1;
            })
            
        }
    }
    res
}

fn parse_in_out(psd: &Vec<&str>, outs:&mut HashMap<String, Vec<String>>, name: &str, ins:&mut HashMap<String, Vec<String>>) {
    let out_nms:Vec<String> = psd[1].split(", ").map(|x| x.to_string()).collect();
    outs.insert(name.to_string(), out_nms.clone());
    out_nms.iter().for_each(|y| {
        let mut yy_opt = ins.get_mut(y);
        if yy_opt.is_none() {
            ins.insert(y.clone(), Vec::new());
            yy_opt = ins.get_mut(y);
        }
        let yy = yy_opt.unwrap();
        yy.push(name.to_string());
    });
}