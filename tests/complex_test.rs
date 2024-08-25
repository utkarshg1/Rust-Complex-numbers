use complex::Complex;
use libm::{atan2, log}; // Adjust this to match your crate name

const TOLERANCE: f64 = 1e-6;

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_new() {
    let c = Complex::new(3.0, 4.0);
    assert_eq!(c.real, 3.0);
    assert_eq!(c.imag, 4.0);
}

#[test]
fn test_modulus() {
    let c = Complex::new(3.0, 4.0);
    assert!(approx_eq(c.modulus(), 5.0, TOLERANCE));
}

#[test]
fn test_argument() {
    let c = Complex::new(1.0, 1.0);
    assert!(approx_eq(
        c.argument(),
        std::f64::consts::FRAC_PI_4,
        TOLERANCE
    ));
}

#[test]
fn test_conjugate() {
    let c = Complex::new(3.0, 4.0);
    let conj = c.conjugate();
    assert_eq!(conj.real, 3.0);
    assert_eq!(conj.imag, -4.0);
}

#[test]
fn test_square_root() {
    let c = Complex::new(3.0, 4.0);
    let sqrt_c = c.square_root();
    assert!(approx_eq(sqrt_c.real, 2.0, TOLERANCE));
    assert!(approx_eq(sqrt_c.imag, 1.0, TOLERANCE));
}

#[test]
fn test_logarithm() {
    let c = Complex::new(3.0, 4.0);
    let log_c = c.logarithm();
    assert!(approx_eq(log_c.real, log(5.0), TOLERANCE));
    assert!(approx_eq(log_c.imag, atan2(4.0, 3.0), TOLERANCE));
}

#[test]
fn test_add() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(2.0, 3.0);
    let sum = c1 + c2;
    assert_eq!(sum.real, 3.0);
    assert_eq!(sum.imag, 5.0);
}

#[test]
fn test_sub() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(2.0, 3.0);
    let diff = c1 - c2;
    assert_eq!(diff.real, -1.0);
    assert_eq!(diff.imag, -1.0);
}

#[test]
fn test_mul() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let product = c1 * c2;
    assert_eq!(product.real, -5.0);
    assert_eq!(product.imag, 10.0);
}

#[test]
fn test_div() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let quotient = c1 / c2;
    assert!(approx_eq(quotient.real, 0.44, TOLERANCE));
    assert!(approx_eq(quotient.imag, 0.08, TOLERANCE));
}

#[test]
#[should_panic(expected = "Attempt to divide by zero complex number")]
fn test_div_by_zero() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(0.0, 0.0);
    let _ = c1 / c2;
}
