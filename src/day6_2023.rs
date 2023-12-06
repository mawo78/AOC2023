pub fn day_6_part1() {
    // Time:      7  15   30
    // Distance:  9  40  200    
    println!("{}", calc(7.0, 9.0) * calc(15.0, 40.0) * calc(30.0, 200.0));

    //Time:        54     70     82     75
    //Distance:   239   1142   1295   1253
    println!("{}", calc(54.0, 239.0) * calc(70.0, 1142.0) * calc(82.0, 1295.0) * calc(75.0, 1253.0));
}

pub fn day_6_part2() {
    // Time:      7  15   30
    // Distance:  9  40  200    
    println!("{}", calc(71530.0, 940200.0));
    //Time:        54     70     82     75
    //Distance:   239   1142   1295   1253
    println!("{}", calc(54708275.0, 239114212951253.0));
}

fn calc(tm:f64, s:f64) -> i64 {
    let d = tm*tm - 4.0 * s;
    let x0 = (tm - f64::sqrt(d)) / 2.0;
    let x1 = (tm + f64::sqrt(d)) / 2.0;
    println!("{} {}", x0, x1);

    let res = f64::trunc(x1) - f64::ceil(x0) + 1.0;
    println!("{}", res);
    return res as i64;
}