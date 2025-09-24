fn optimized_ln(x: f64, epsilon: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    
    // Для больших x используем ln(x) = -ln(1/x) для лучшей сходимости
    let (value, sign) = if x > 1.5 {
        (1.0 / x, -1.0)
    } else {
        (x, 1.0)
    };
    
    let u = value - 1.0;
    let u_squared = u * u;
    let mut result = 0.0;
    let mut term = u;
    let mut n = 1;
    
    // Используем итерацию с предвычисленным u² для ускорения
    while term.abs() > epsilon {
        result += term / n as f64;
        term *= -u_squared;
        n += 2;
        
        // Добавляем следующий нечетный член
        if term.abs() > epsilon {
            result += term / n as f64;
            term *= -u_squared;
            n += 2;
        }
    }
    
    sign * result
}

fn main() {
    let x = 100500.0;
    let epsilon = 1e-6;
    
    let result = optimized_ln(x, epsilon);
    let exact = x.ln();
    
    println!("ln({}) ≈ {:.8}", x, result);
    println!("Точное значение: {:.8}", exact);
    println!("Погрешность: {:.2e}", (result - exact).abs());
    println!("Количество итераций: {}", ((n as f64 - 1.0) / 2.0) as usize);
}