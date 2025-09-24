use std::io;
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
fn tsin(x: f64, epsilon: f64) -> f64
{
    //ryad teilora

    let mut res = x;
    let mut term = x;
    let mut n = 1;
     
    while term.abs() > epsilon
    {
        term *=-(x*x) /((2 * n) * (2 * n +1)) as f64;
        res += term;
        n+=1;
    }
    res
}

fn main() {

    //ln(100500)*sin(10)
    let x = 100500.0;
    let y = 10.0;
    let epsilon = 1e-6;
    
    let result = taylor(x, epsilon) * tsin(y,epsilon);
    let tsylor = taylor(x, epsilon);
    let tsin = tsin(y,epsilon);
    let exact = x.ln();
    let tet = y.sin();
    println!("real sin = ({})",tet);
    println!("tsin= ({:.16})", tsin);
    println!("real ln = {:.16}", exact);
    println!("taylor= ({:.16})", tsylor);
    println!("result= ({:.16})", result);

    // marker keroka
    // task machine vision ()
}