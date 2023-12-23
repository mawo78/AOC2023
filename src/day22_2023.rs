use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};
#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Point3D{
    x:i32,y:i32,z:i32
}
#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Brick {
    strt:Point3D,
    end:Point3D,
}
impl Point3D {
    fn parse(mut self, pnkt_str:&str)->Point3D{
        let psd:Vec<&str> = pnkt_str.split(",").collect();
        self.x = i32::from_str_radix(psd[0], 10).unwrap();
        self.y = i32::from_str_radix(psd[1], 10).unwrap();
        self.z = i32::from_str_radix(psd[2], 10).unwrap();
        self
    }
}
impl Brick {
    fn parse(mut self, pnkt_str:&str) ->Brick{
        // 6,0,285~6,1,285
        let psd:Vec<&str> = pnkt_str.split("~").collect();
        self.strt = self.strt.parse(psd[0]);
        self.end = self.end.parse(psd[1]);        
        self
    }

    fn insert(self, wll:&mut HashMap<Point3D, i32>, id:i32){
        let l3d = self.len3d();
        let l = self.len();
        let d = 
            if l3d.x == l { (1, 0, 0)}
            else if l3d.y == l { (0, 1, 0)}
            else  { (0, 0, 1)};
        let mut pnkt = self.strt.clone();
        for _ in 0..l {
            wll.insert(pnkt.clone(), id);
            pnkt.x += d.0;
            pnkt.y += d.1;
            pnkt.z += d.2;
        }
    }

    fn remove(self, wll:&mut HashMap<Point3D, i32>){
        let l3d = self.len3d();
        let l = self.len();
        let d = 
            if l3d.x == l { (1, 0, 0)}
            else if l3d.y == l { (0, 1, 0)}
            else  { (0, 0, 1)};
        let mut pnkt = self.strt.clone();
        for _ in 0..l {
            wll.remove(&pnkt);
            pnkt.x += d.0;
            pnkt.y += d.1;
            pnkt.z += d.2;
        }
    }

    fn len3d(self) -> Point3D {
        Point3D{
            x:(self.end.x - self.strt.x + 1),
            y:(self.end.y - self.strt.y + 1),
            z:(self.end.z - self.strt.z + 1)    
        }
    }

    fn len(self) ->i32{
        let pom = self.len3d();
        pom.x.max(pom.y.max(pom.z))
    }

    fn get_bottom(&self) -> Vec<Point3D> {
        let l3d = self.len3d();
        let mut res:Vec<Point3D> = Vec::new();
        if l3d.z > 1 {
            res.push(Point3D{x:self.strt.x, y:self.strt.y, z:self.strt.z.min(self.end.z)});
            res
        } else {
            if l3d.x > 1 {
                for i in self.strt.x.min(self.end.x)..=self.strt.x.max(self.end.x){
                    res.push(Point3D{x:i, y:self.strt.y, z:self.strt.z});
                }
                res
            } else {
                for i in self.strt.y.min(self.end.y)..=self.strt.y.max(self.end.y){
                    res.push(Point3D{x:self.strt.x, y:i, z:self.strt.z});
                }
                res
            }
        }
    }
}
pub fn day_22() {
    let file = File::open("input/input22_2023.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut brks:Vec<Brick> = Vec::new(); // id -> Brick
    lines.iter().for_each(|ln|{
        let mut brk = Brick { strt:Point3D{x:0,y:0,z:0}, end:Point3D{x:0,y:0,z:0}};
        brk = brk.parse(ln);
        brks.push(brk.clone());
    });

    brks.sort_by(|a,b| (a.strt.z.min(a.end.z)).cmp(&b.strt.z.min(b.end.z)));

    let mut wll:HashMap<Point3D, i32> = HashMap::new();
    brks.iter().enumerate().for_each(|par| {
        //println!("{:?}", &par);
        par.1.insert(&mut wll, par.0 as i32);
    });
    //println!("{:?}", &wll);
    //println!("{}", &wll.len());
    while try_to_fall(&mut brks, &mut wll, false) > 0{
        print!("X");
    }

    //display_lvls(&wll);


    let bckp_brks = brks.clone();
    let bckp_wll = wll.clone();
    let mut res = 0;

    bckp_brks.iter().enumerate().for_each(|en_brk|{
        let id = en_brk.0 as i32;
        let brk = en_brk.1;

        brk.remove(&mut wll);
        brks.remove(id as usize);

//        display_lvls(&wll);       
        let fln = try_to_fall(&mut brks, &mut wll, false);
        if fln > 0 {
            res += fln;
            println!("{}  {}", id, fln);
        } else {
            println!("cant explode: {}", id);
        }

        brks = bckp_brks.clone();
        wll = bckp_wll.clone();
    });

    println!("{}", res);
}

fn display_lvls(wll: &HashMap<Point3D, i32>) {
    for z in 0..7{
        println!("{}", z);
        for y in 0..3 {
            for x in 0..3 {
                print!("{:2}", wll.get(&Point3D { x: x, y: y, z: z }).unwrap_or(&-1))
            }
            println!();    
        }
        //println!();    
    }
}

fn try_to_fall(brks:&mut Vec<Brick>, mut wll: &mut HashMap<Point3D, i32>, only_check:bool) -> i32 {
    let mut fall = 0;
    //brks.iter_mut().enumerate().for_each(|enum_brk|
    for id in 0..brks.len()    {
        //let id = enum_brk.0 as i32;
        let  brk:&mut Brick = &mut brks[id];// enum_brk.1;
        let btm:Vec<Point3D> = brk.get_bottom();
        if btm[0].z > 0 {
            let mut lo_z:Vec<i32> = Vec::new();
            for vxl in &btm {
                let mut p = vxl.clone();
                for z in (0..vxl.z).rev() {
                    p.z = z;
                    if z == 0 || wll.contains_key(&p){
                        break;
                    }
                }
                lo_z.push(p.z);
            }
            let max_z = lo_z.iter().max().unwrap();

            if max_z < &(btm[0].z - 1)  { //move brick down
                if only_check{
                    return 1;
                }
                //remove voxels
                brk.remove(&mut wll);
                //move brick
                let dz = brk.strt.z.min(brk.end.z) - *max_z - 1;
                brk.strt.z -= dz; //*max_z + 1;
                brk.end.z -= dz;// *max_z + 1;
                //insert voxels
                brk.insert(wll, id as i32);

                fall += 1;
            }

        }
    };

    fall
}
// part 1
// 504 too high
// 498 too high
// 480 OK