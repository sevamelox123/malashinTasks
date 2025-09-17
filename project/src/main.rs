fn taylor(x: f64, epsilon: f64) -> f64 {
    //ln(x) = -ln(1/x)
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

fn main() {
    let x = 100500.0;
    let epsilon = 1e-6;
    
    let result = taylor(x, epsilon);
    let exact = x.ln();
    
    println!("ln({}) ≈ {:.16}", x, result);
    println!("{:.16}", exact);
}