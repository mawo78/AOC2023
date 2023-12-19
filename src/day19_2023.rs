use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug)]
struct Part {
    x:i64,
    m:i64,
    a:i64,
    s:i64
}

#[derive(Debug,Clone, Copy)]
struct PartRange {
    x:(i64, i64),
    m:(i64, i64),
    a:(i64, i64),
    s:(i64, i64)
}

#[derive(Debug)]
struct Workflow {
    el:char, 
    cmp:char, 
    val:i64, 
    nxt:String
}

impl PartRange {
    fn cmb(self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.m.1 - self.m.0 + 1) * (self.a.1 - self.a.0 + 1) * (self.s.1 - self.s.0 + 1)
    }
}
pub fn day_19() {
    let file = File::open("input/input19_2023tst.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut parts:Vec<Part> = Vec::new();
    let mut rules:HashMap<String, Vec<Workflow>> = HashMap::new();

    lines.iter().for_each(|ln|{
        if ln.starts_with("{") {
            //{x=787,m=2655,a=1222,s=2876}
            let pom = ln.get(1..ln.len()-1).unwrap();
            let psd:Vec<&str> = pom.split(",").collect();
            let part = Part{ 
                x:parse_val(psd[0]),
                m:parse_val(psd[1]),
                a:parse_val(psd[2]),
                s:parse_val(psd[3]),
            };
            parts.push(part);
        } else if !ln.is_empty() {
            let (name, wfs) = parse_workflow(ln);
            rules.insert(name.to_string(), wfs);
        }
    });

//    println!("{:?}", rules);
//    println!("{:?}", parts);

    let res = parts.iter().filter(|prt| {
        check_part(&rules, prt)
    }).map(|prt| {
        prt.x + prt.m +prt.a + prt.s
    }).sum::<i64>();

    println!("Part 1: {}", res);

    let part_range = PartRange{
        x:(1,4000),
        m:(1,4000),
        a:(1,4000),
        s:(1,4000),
    };
    let res = req(&rules, "in", &part_range);
    println!("Part 2: {}", res)
}

fn req(all_rules: &HashMap<String, Vec<Workflow>>, name: &str, part_range0: &PartRange) -> i64 {
    if "A" == name {
        return part_range0.cmb();
    } else if "R" == name {
        return 0;
    } else {
        let mut part_range = part_range0.clone();
        let mut res = 0;
        let rules = all_rules.get(name).unwrap();
        for rule in rules {
            if rule.el == '*' {
                res += req(all_rules, &rule.nxt, &part_range);
                break;
            }            

            let fld_val = match rule.el {
                'x' => part_range.x,
                'm' => part_range.m,
                'a' => part_range.a,
                's' => part_range.s,
                _ => unreachable!(),
            };
            match rule.cmp {
                '<' => {
                    let prz0 = (1, rule.val - 1);
                    let prz1 = (rule.val, 4000_i64);
                    //condition met, call req
                    let rng0 = (prz0.0.max(fld_val.0), prz0.1.min(fld_val.1));
                    let mut rq_range = part_range.clone();
                    match rule.el {
                        'x' => rq_range.x = rng0,
                        'm' => rq_range.m = rng0,
                        'a' => rq_range.a = rng0,
                        's' => rq_range.s = rng0,
                        _ => unreachable!(),
                    };
                    res += req(all_rules, &rule.nxt, &rq_range);

                    //update used range
                    let rng1 = (prz1.0.max(fld_val.0), prz1.1.min(fld_val.1));
                    match rule.el {
                        'x' => part_range.x = rng1,
                        'm' => part_range.m = rng1,
                        'a' => part_range.a = rng1,
                        's' => part_range.s = rng1,
                        _ => unreachable!(),
                    };
                },
                '>' => {
                    let prz0 = (rule.val + 1, 4000_i64);
                    let prz1 = (1, rule.val);
                    //condition met, call req
                    let rng0 = (prz0.0.max(fld_val.0), prz0.1.min(fld_val.1));
                    let mut rq_range = part_range.clone();
                    match rule.el {
                        'x' => rq_range.x = rng0,
                        'm' => rq_range.m = rng0,
                        'a' => rq_range.a = rng0,
                        's' => rq_range.s = rng0,
                        _ => unreachable!(),
                    };
                    res += req(all_rules, &rule.nxt, &rq_range);

                    //update used range
                    let rng1 = (prz1.0.max(fld_val.0), prz1.1.min(fld_val.1));
                    match rule.el {
                        'x' => part_range.x = rng1,
                        'm' => part_range.m = rng1,
                        'a' => part_range.a = rng1,
                        's' => part_range.s = rng1,
                        _ => unreachable!(),
                    };

                },
                _ => unreachable!()
            }
        }
        return res;
    }
}


fn parse_workflow(ln: &String) -> (&str, Vec<Workflow>) {
    //px{a<2006:qkq,m>2090:A,rfg}
    let psd0:Vec<_> = ln.split("{").collect();
    let name = psd0[0];
    let pom = psd0[1].get(0..psd0[1].len()-1).unwrap();
    let psd1:Vec<_> = pom.split(",").collect();
    //println!("{:?}", psd1);
    let wfs:Vec<Workflow> = psd1.iter().map(|wf|{
        if wf.contains(":") {
            let psd2:Vec<_> = wf.split(":").collect();

            Workflow{
                el:psd2[0].chars().nth(0).unwrap(), 
                cmp:psd2[0].chars().nth(1).unwrap(), 
                val:parse_val(psd2[0]), 
                nxt:psd2[1].to_string(),
            }
        } else {
            Workflow{
                el:'*', 
                cmp:'*', 
                val:-1, 
                nxt:wf.to_string(),
            }
        }
    }).collect();
    (name, wfs)
}

#[test]
fn test_parse_wrkflw(){
    let tst1:String = "px{a<2006:qkq,m>2090:A,rfg}".to_string();
    let (name, wf) = parse_workflow(&tst1);
    assert_eq!(name, "px");
    println!("{:?}", wf);
}

fn check_part(all_rules: &HashMap<String, Vec<Workflow>>, part: &Part) -> bool {
    let mut name = "in";
    while name != "A" && name != "R" {
        let rules = all_rules.get(name).unwrap();
        for i in 0..rules.len(){
            if rules[i].el == '*' {
                name = &rules[i].nxt;
                break;
            }
            let fld_val = match rules[i].el {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!(),
            };
            if match rules[i].cmp {
                '<' => fld_val < rules[i].val,
                '>' => fld_val > rules[i].val,
                _ => unreachable!()
            } {
                name = &rules[i].nxt;
                break;
            }
        }
    }

    name == "A"
}

fn parse_val(psd: &str) -> i64 {
    i64::from_str_radix(psd.get(2..psd.len()).unwrap(), 10).unwrap()
}