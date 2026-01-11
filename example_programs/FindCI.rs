use statrs::distribution::{ContinuousCDF, Normal};

fn areadist(a: f64, b: f64, alpha: f64, distribn: &Normal) -> f64 {
    distribn.cdf(b) - distribn.cdf(a) - alpha
}

// Find x such that cdf(x) = lambda using bisection on [a, b]
fn ivt(maxit: u32, mut a: f64, mut b: f64, lambda: f64, ts: f64, distribn: &Normal) -> f64 {
    let mut c = (a + b) / 2.0;
    for _ in 0..maxit {
        let fa = distribn.cdf(a) - lambda;
        let fc = distribn.cdf(c) - lambda;

        if fc == 0.0 || (b - a).abs() / 2.0 < ts {
            break;
        }
        if fa * fc > 0.0 {
            a = c;
        } else {
            b = c;
        }
        c = (a + b) / 2.0;
    }
    c
}

// Find lambda so that the probability mass between a and b is alpha
fn get_lambda(
    maxit: u32,
    mut max: f64,
    mut min: f64,
    a: &mut f64,
    b: &mut f64,
    ts: f64,
    distribn: &Normal,
    alpha: f64,
) -> f64 {
    let mut lambda = 0.0;

    for _ in 0..maxit {
        lambda = (max + min) / 2.0;

        // For a symmetric interval, use λ = CDF(0) - (α/2)
        // so that CDF(a) = λ and CDF(b) = 1 - λ.
        // Here we solve CDF(a) = lambda and CDF(b) = 1 - lambda.
        *a = ivt(maxit, -10.0, 0.0, lambda, ts, distribn);
        *b = ivt(maxit, 0.0, 10.0, 1.0 - lambda, ts, distribn);

        if (max - min).abs() < ts {
            break;
        }

        let area_diff = areadist(*a, *b, alpha, distribn);
        if area_diff < 0.0 {
            max = lambda;
        } else if area_diff > 0.0 {
            min = lambda;
        } else {
            break;
        }
    }

    lambda
}

fn main() {
    let normal_dist = Normal::new(0.0, 1.0).unwrap();
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    // alpha = 0.95 for 95% CI
    let lambda = get_lambda(
        1600,
        0.49,       // upper search bound for lambda
        0.01,       // lower search bound for lambda
        &mut a,
        &mut b,
        1e-8,
        &normal_dist,
        0.95,
    );

    println!("lambda = {}", lambda);
    println!("95% CI ~ [{}, {}]", a, b);
}

