/*
* Finds \sqrt{2}.
*/

fn g(a: f64, x: &f64) -> f64 {
    0.5 * ((a / *x) - *x)
}

fn iterate(maxit: u32, a: f64, x: &mut f64, it: &mut u32) -> f64 {
    if *it >= maxit {return *x;}
    *it += 1;
    *x = g(a, x) + *x;
    iterate(maxit, a, x, it)
}

fn main() {
    let mut x: f64 = 1.4;
    let mut it: u32 = 0;

    println!("{}", iterate(4, 2.0, &mut x, &mut it));
}