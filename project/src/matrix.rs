
// f(x) = 0;


fn determinant(matrix: &[Vec<f64>]) -> f64 {
    let n = matrix.len();
    
    if matrix.iter().any(|row| row.len() != n) {
        panic!("Matrix should be a square");
    }
    
    match n {
        0 => 0.0,
        1 => matrix[0][0],
        2 => matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0],
        _ => {
            let mut det = 0.0;
            for j in 0..n {
                let mut minor = Vec::with_capacity(n - 1);
                for i in 1..n {
                    let mut row = Vec::with_capacity(n - 1);
                    for k in 0..n {
                        if k != j {
                            row.push(matrix[i][k]);
                        }
                    }
                    minor.push(row);
                }
                let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
                det += sign * matrix[0][j] * determinant(&minor);
            }
            det
        }
    }
}

fn replace_column(matrix: &[Vec<f64>], column: &[f64], col_index: usize) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut result = matrix.to_vec();
    
    for i in 0..n {
        result[i][col_index] = column[i];
    }
    
    result
}

fn solve_cramer(a: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, String> {
    let n = a.len();
    
    if n == 0 {
        return Err("Matrix is empty".to_string());
    }
    
    if b.len() != n {
        return Err("Size difference".to_string());
    }
    
    if a.iter().any(|row| row.len() != n) {
        return Err("Matrix should be square".to_string());
    }
    
    let main_det = determinant(a);
    
    if main_det.abs() < 1e-10 {
        return Err("det = 0 ".to_string());
    }
    
    let mut solution = Vec::with_capacity(n);
    
    for j in 0..n {
        let modified_matrix = replace_column(a, b, j);
        let det_j = determinant(&modified_matrix);
        
        solution.push(det_j / main_det);
    }
    
    Ok(solution)
}

fn main() {
    println!("Example");
    
    let a = vec![
        vec![2.0, 1.0, 5.0, 11.0, 5.0],
        vec![1.0, 3.0, 8.0, 16.0, 23.0],
        vec![4.0, 3.0, 2.0, 28.0, 1.0],
        vec![7.0, 43.0, 2.0, 1.0, 8.0],
        vec![11.0, 2.0, 3.0, 6.0, 4.0]
    ];
    let b = vec![5.0, 10.0,7.0, 6.0, 11.0];
    
    match solve_cramer(&a, &b) {
        Ok(x) => {
            println!("Matrix A: {:?}", a1);
            println!("Vector b: {:?}", b1);
            println!("Solution x: {:?}", x);
        }
        Err(e) => println!("err : {}", e),
    }
}