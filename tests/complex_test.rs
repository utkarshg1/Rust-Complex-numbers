use complex::Complex;
const EPSILON: f64 = 1e-6; // Define a small tolerance value

#[test]
fn test_addition() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(1.0, 2.0);
    let result = a + b;
    assert_eq!(result, Complex::new(4.0, 6.0));
}

#[test]
fn test_subtraction() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(1.0, 2.0);
    let result = a - b;
    assert_eq!(result, Complex::new(2.0, 2.0));
}

#[test]
fn test_multiplication() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(1.0, 2.0);
    let result = a * b;
    assert_eq!(result, Complex::new(-5.0, 10.0));
}

#[test]
fn test_division() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(1.0, 2.0);
    let result = a / b;
    assert!(
        (result.real - 2.2).abs() < EPSILON,
        "Real part: {} is not approximately equal to 2.2",
        result.real
    );
    assert!(
        (result.imag - (-0.4)).abs() < EPSILON,
        "Imaginary part: {} is not approximately equal to -0.4",
        result.imag
    );
}

#[test]
#[should_panic(expected = "Attempt to divide by zero complex number")]
fn test_division_by_zero() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(0.0, 0.0);
    let _result = a / b; // This should panic
}

#[test]
fn test_modulus() {
    let a = Complex::new(3.0, 4.0);
    let result = a.modulus();
    assert_eq!(result, 5.0); // sqrt(3^2 + 4^2) = 5
}

#[test]
fn test_argument() {
    let a = Complex::new(3.0, 4.0);
    let result = a.argument();
    assert!((result - 0.927295).abs() < EPSILON);
}
