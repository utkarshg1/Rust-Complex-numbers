use complex::Complex;
use std::io;
fn main() {
    let num1 = get_complex_number("Please enter Complex number 1");
    println!("The num1 is {}", num1);
    println!("Modulus of num1 is {:.4}", num1.modulus());
    println!("Argument of num1 is {:.4}", num1.argument());

    let num2 = get_complex_number("Please enter complex number 2:");
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

fn get_complex_number(prompt: &str) -> Complex {
    println!("\n{}\n", prompt);

    println!("Enter real part : ");
    let mut real = String::new();
    io::stdin()
        .read_line(&mut real)
        .expect("Unable to get string");
    let real: f64 = real.trim().parse().expect("Please enter valid float");

    println!("Enter Imaginary part : ");
    let mut imag = String::new();
    io::stdin()
        .read_line(&mut imag)
        .expect("Unable to get string");
    let imag: f64 = imag.trim().parse().expect("Please enter valid float");

    Complex::new(real, imag)
}
