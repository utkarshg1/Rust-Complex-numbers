# Complex numbers functionality in RUST

How to use this lib.rs in main

```rust
use complex::Complex;
fn main() {
    // Can create complex numbers and check modulus and argument
    let num1: Complex = Complex::new(3.0, 4.0);
    let mod1: f64 = num1.modulus();
    let arg1: f64 = num1.argument();

    // Can perform addition, subtraction, multiplication, division
    let num2: Complex = Complex::new(1.0, 2.0);
    let add: Complex = num1 + num2;
    let sub: Complex = num1 - num2;
    let mul: Complex = num1*num2;
    let div: Comples = num1/num2;

    // Can print complex numbers directly
    println!("num1 : {}, num2 : {}",num1, num2);
    println!(
        "add : {}, sub : {}, mul : {}, div :{}",
        add, sub, mul, div
    );
}
```
