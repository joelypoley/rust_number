use std::mem::swap;
use uint::UInt;
use int::Int;

pub fn gcd(x: &UInt, y: &UInt) -> UInt {
    let mut a = x.clone();
    let mut b = y.clone();
    while !b.is_zero() {
        a %= &b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn xgcd(x: &Int, y: &Int) -> (Int, Int, Int) {
    let a = x;
    let b = y;
    let mut u = Int::from(1);
    let mut v = Int::from(0);
    let mut d = x.clone();
    if !b.is_zero() {
        let mut v_1 = Int::from(0);
        let mut v_3 = b.clone();
        while !v_3.is_zero() {
            let q = &d / &v_3;
            let t_3 = &d % &v_3;
            let t_1 = &u - q * &v_1;
            u = v_1.clone();
            d = v_3.clone();
            v_1 = t_1;
            v_3 = t_3;
        }
        v = (&d - a * &u) / b;
    }

    (u, v, d)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gcd() {
        let x = UInt::from(14175);
        let y = UInt::from(756315);
        let ans = UInt::from(315);
        assert_eq!(gcd(&x, &y), ans);
        assert_eq!(gcd(&y, &x), ans);
        assert_eq!(gcd(&ans, &UInt::from(1)), UInt::from(1));
    }

    #[test]
    fn test_xgcd() {
        let x = Int::from(14175);
        let y = Int::from(756315);
        let ans = Int::from(315);
        let (u, v, d) = xgcd(&x, &y);
        assert_eq!(d, ans);
        assert_eq!(x*u + y*v, d);
    }
}
