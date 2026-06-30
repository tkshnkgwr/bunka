#[cfg(not(feature = "gui"))]
pub mod cli;

#[cfg(feature = "gui")]
pub mod gui;

/// 連分数展開アルゴリズムによる分数近似
pub fn approximate_fraction(value: f64, max_denominator: u64, tolerance: f64) -> (i64, u64, f64) {
    if value == 0.0 {
        return (0, 1, 0.0);
    }

    let sign = if value < 0.0 { -1 } else { 1 };
    let target = value.abs();

    let mut h1 = 1i64;
    let mut h2 = 0i64;
    let mut k1 = 0u64;
    let mut k2 = 1u64;

    let mut r = target;
    let mut a = r.floor() as i64;
    let mut step = 0;

    loop {
        let h = a * h1 + h2;
        let k = (a as u64) * k1 + k2;

        if k > max_denominator {
            break;
        }

        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;

        let approx_value = (h1 as f64 / k1 as f64) * (sign as f64);
        let error = (value - approx_value).abs();

        if error <= tolerance || (r - a as f64).abs() < 1e-11 {
            break;
        }

        let diff = r - a as f64;
        if diff.abs() < 1e-11 {
            break;
        }
        r = 1.0 / diff;
        a = r.floor() as i64;

        step += 1;
        if step > 50 {
            break;
        }
    }

    let final_approx = (h1 as f64 / k1 as f64) * (sign as f64);
    (h1 * sign, k1, (value - final_approx).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_approximate_fraction_positive() {
        let (num, den, _) = approximate_fraction(0.142857, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 7);

        let (num, den, _) = approximate_fraction(0.333333, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 3);

        let (num, den, _) = approximate_fraction(3.14159265, 100000, 1e-6);
        assert_eq!(num, 355);
        assert_eq!(den, 113);
    }

    #[test]
    fn test_approximate_fraction_zero() {
        let (num, den, _) = approximate_fraction(0.0, 100000, 1e-6);
        assert_eq!(num, 0);
        assert_eq!(den, 1);
    }

    #[test]
    fn test_approximate_fraction_negative() {
        let (num, den, _) = approximate_fraction(-0.5, 100000, 1e-6);
        assert_eq!(num, -1);
        assert_eq!(den, 2);
    }
}
