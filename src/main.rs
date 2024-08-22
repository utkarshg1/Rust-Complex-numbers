use complex::Complex;
fn main() {
    let num1 = Complex::new(3.0, 4.0);
    println!("The num1 is {}", num1);
    println!("Modulus of num1 is {:.4}", num1.modulus());
    println!("Argument of num1 is {:.4}", num1.argument());

    let num2 = Complex::new(1.0, 2.0);
    println!("The num2 is {}", num2);
    println!("Modulus of num2 is {:.4}", num2.modulus());
    println!("Argument of num2 is {:.4}", num2.argument());
    println!("\n==================================================\n");

    let num3 = num1 + num2;
    println!("Addition of num1 and num2 is {}", num3);

    let num4 = num1 - num2;
    println!("Subtraction of num1 and num2 is {}", num4);

    let num5 = num1 * num2;
    println!("Multiplication of num1 and num2 is {}", num5);

    let num6 = num1 / num2;
    println!("Division of num1 and num2 is {}", num6);
}
