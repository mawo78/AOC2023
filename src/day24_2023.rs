use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}, time::Instant};

#[derive(Debug, Clone, Copy, Default)]
struct Line3d{
    p0: (i128, i128, i128),
    v: (i128, i128, i128),
}

pub fn day_24() {
    let file = File::open("input/input24_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let lines3d:Vec<Line3d> = lines.iter().map(|ln|{
        //19, 13, 30 @ -2,  1, -2
        let spl:Vec<_> = ln.split(" @ ").collect();
        let p0v:Vec<i128> = spl[0].split(", ").map(|s|{ i128::from_str_radix(s.trim(), 10).unwrap()}).collect();
        let vv:Vec<i128> = spl[1].split(", ").map(|s|{ i128::from_str_radix(s.trim(), 10).unwrap()}).collect();
        Line3d { p0: (p0v[0], p0v[1], p0v[2]), v: (vv[0], vv[1], vv[2]) }
    }).collect();

    let mut res = 0;
    //lines3d.iter().enumerate().for_each(|ln_en|{
    for i in 0.. lines3d.len() - 1{
        let ln = &lines3d[i];
        for j in i+1..lines3d.len(){
            let ln2 = &lines3d[j];
            let crs = cross2d(ln, ln2);
            if crs.0 {
                let p = crs.1;
                if p.0 >= 200000000000000 && p.0 <= 400000000000000{
                    if p.1 >= 200000000000000 && p.1 <= 400000000000000{
                        if (p.0 - ln.p0.0).signum() == ln.v.0.signum() {
                            //println!("{}", (p.0 - ln.p0.0 as f64) / ln.v.0 as f64);
                            if (p.0 - ln2.p0.0).signum() == ln2.v.0.signum() {
                                //println!("{}", (p.0 - ln2.p0.0 as f64) / ln2.v.0 as f64);
                                // println!("{}", lines[i]);
                                // println!("{}", lines[j]);
                                // println!("{:?}", crs);
                                // println!();
                                res += 1;
                            }
                        }

                    }                        
                }
            }
        }
    };

    println!("Part 1: {}", res);

    //part 2
    let res_x_y = find_start_point(&lines3d);

    let lines32z:Vec<Line3d> = lines3d.iter().map(|p|{ Line3d{
        p0: (p.p0.0, p.p0.2, p.p0.1),
        v: (p.v.0, p.v.2, p.v.1)
    }}).collect();

    let res_x_z = find_start_point(&lines32z);

    println!("{}", res_x_y.1.0 + res_x_y.1.1 + res_x_z.1.1);

    // for i in 0.. lines3d.len() - 1{
    //     let ln = &lines3d[i];
    //     for j in i+1..lines3d.len(){
    //         let ln2 = &lines3d[j];
    //         if ln.v.0 == ln2.v.0 {
    //             println!("{}", ln2.v.0);
    //         }
    //     }
    // }

}

const MAX_V:i128 = 400;

fn find_start_point(lines3d: &Vec<Line3d>) -> ((i128,i128,i128), (i128,i128)){
    let start = Instant::now();
    let mut done = false;
    for i in -MAX_V..MAX_V {
        println!("{}", i);
        for j in -MAX_V..MAX_V {
            for k in -MAX_V..MAX_V {
                let v_r = (i,j,k);
                let trnsl_l32:Vec<_> = lines3d.iter().take(4).map(|ln| {
                    Line3d{ p0:ln.p0,
                    v:  (ln.v.0 - i, ln.v.1 - j, ln.v.2 - j)}
                }).collect();

                let mut ile = 0;
                let mut pnkts:HashSet<(i128,i128)> = HashSet::new();
                for l in 0..trnsl_l32.len()-1 {
                    let ln = trnsl_l32[l];
                    for m in l + 1..trnsl_l32.len(){
                        let ln2 = trnsl_l32[m];
                        let crs = cross2d(&ln, &ln2);
                        pnkts.insert(crs.1);
                        ile+=1;

                        if pnkts.len() > 1 {
                            break;
                        }
                    }
                    if pnkts.len() > 1 {
                        break;
                    }
                }
                if pnkts.len() == 1 { //ile == pnkts.len() * (pnkts.len() - 1) / 2 {                    
                    println!("{:?}, {} {:?}", v_r, ile, pnkts.iter().collect::<Vec<&(i128,i128)>>()[0]);
                    done = true;
                    let duration = start.elapsed();
                    println!("Time elapsed in calculation is: {:?}", duration);
                    return (v_r, *pnkts.iter().collect::<Vec<&(i128,i128)>>()[0]);
                }
                if done {break;}
            }
            if done {break;}                
        }
        if done {break;}    
    }
    ((0_i128,0,0), (0,0))
}

fn cross2d(lin1: &Line3d, lin2: &Line3d) -> (bool, (i128, i128)) {
    let p1 = (lin1.p0.0, lin1.p0.1);
    let p2 = (p1.0 + lin1.v.0, p1.1 + lin1.v.1);

    let l1 = (lin2.p0.0, lin2.p0.1);
    let l2 = (l1.0 + lin2.v.0, l1.1 + lin2.v.1);
    let t = (p1.0 - p2.0) * (l1.1 - l2.1) - (p1.1 - p2.1) * (l1.0 - l2.0);
    if t == 0 {
        return (false, (0, 0));
    }
    let s = (l2.0 - p2.0) *(l1.1 -l2.1) - (l2.1 - p2.1) *(l1.0 -l2.0);

    let s = (s) / t;
    let x = s * p1.0  + (1-s)*p2.0;
    let y = s * p1.1  + (1-s)*p2.1;

    (true, (x, y))
}
//part 1:
//12939 too high
