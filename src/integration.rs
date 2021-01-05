


include!("./roots.rs");

pub fn integrate<F>(f: F, range: std::ops::Range<f64>) -> f64
    where 
        F: Fn(f64) -> f64 
{
    let mut sum = 0.0f64;

    for (x, w) in LEGENDRE_POINTS_AND_WEIGHTS.iter() {
        let x = ((range.start + range.end) + x * (range.end - range.start)) / 2.0f64;
        sum += w * f(x);
    }
    
    sum * (range.end - range.start) / 2.0f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intergate() {
        let val = integrate(|x| x, 0f64..1f64);

        assert!((val - 0.5f64).abs() < 1e-12f64);
    }


    #[test]
    fn test_intergate2() {
        use std::f64::consts::PI;
        
        let val = integrate(|x| x.powi(4), 0f64..PI);
        assert!((val - 61.203936957056290f64).abs() < 1e-12f64);
    }

}
