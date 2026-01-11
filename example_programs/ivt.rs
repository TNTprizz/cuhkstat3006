fn g(x: f32) -> f32 {3.0 * x.powf(2.0) - 5.0} // You may modify the equation here

fn ivt(maxit: u32, mut a: f32, mut b: f32, ts: f32) -> f32 {
    let mut c: f32 = (a + b) / 2.0;
    for _ in 1..=maxit {
        if g(c) == 0.0 || (((b - a) / 2.0)) < ts {break;}
        if g(c)*g(a) > 0.0 {a=c;} else {b=c}
        c = (a + b) / 2.0;
    }
    c
}

fn main() {
    println!("{}", ivt(20, 0.0, 2.0, 0.000000001));
}