use super::*;

#[test]
fn test_add() {
    let mut s1 = Frac::from(0);
    for i in 2..20 {
        s1 += Frac::new(1, i);
    }
    let ans = Frac::new(197698279, 77597520);
    assert_eq!(ans, s1);
}

#[test]
fn test_pow() {
    let mut s2 = Frac::new(1, 3);
    s2 = s2.pow(5);
    let ans = 1.0 / 243.0;
    assert_eq!(ans, f64::from(s2));
}
