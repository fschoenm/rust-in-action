use num::complex::Complex;

fn main() {
    let a = Complex { re: 1.0, im: 2.0 };
    let b = Complex { re: 2.0, im: 3.0 };

    let result = a + b;
    println!("{} + {} = {}", a, b, result);
}
