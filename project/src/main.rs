
use std::io;
use std::f64::consts::PI;
fn taylor(x: f64, epsilon: f64) -> f64 {
    //ln(x) = -ln(1/x) ryad Merkatora
    let (value, sign) = if x > 2.0 {
        (1.0 / x, -1.0)
    } else {
        (x, 1.0)
    };
    
    let u = value - 1.0;
    let mut result = 0.0;
    let mut term = u;
    let mut n = 1;
    
    loop {
        result += term / n as f64;
        term *= -u;
        n += 1;
        
        if term.abs() < epsilon {
            break;
        }
    }
    
    sign * result
}
fn tsin(x: f64, epsilon: f64) -> f64 {
    // Нормализуем угол к диапазону [-π, π] для быстрой сходимости
    let x_normalized = x % (2.0 * PI);
    
    // Ряд Тейлора для sin(x)
    let mut res = x_normalized;
    let mut term = x_normalized;
    let mut n = 1;
    let x_squared = x_normalized * x_normalized;
     
    while term.abs() > epsilon {
        term *= -x_squared / ((2 * n) * (2 * n + 1)) as f64;
        res += term;
        n += 1;
    }
    res
}

fn main() {
    // rustc main.rs && ./main
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения строки");
    
    let number: f64 = input.trim().parse().expect("Введите корректное число!");


    //ln(100500)*sin(10)
    println!("number= ({:.16})", number);
    let epsilon = 1e-6;
    

    
    let result = taylor(number, epsilon) * tsin(number,epsilon);
    let tsylor = taylor(number, epsilon);
    let tsin = tsin(number,epsilon);
    // let exact = x.ln();
    // let tet = y.sin();
    // println!("real sin = ({})",tet);
    println!("tsin= ({:.16})", tsin);
    // println!("real ln = {:.16}", exact);
    println!("taylor= ({:.16})", tsylor);
    println!("result= ({:.16})", result);

    // marker keroka
    // task machine vision ()
}
