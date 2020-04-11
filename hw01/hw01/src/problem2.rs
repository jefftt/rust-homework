/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert!(mat2.len() > 0);
    assert!(mat1.len() == mat2[0].len());
    let mut out = Vec::new();
    for i in 0..mat1.len() {
        out.push(Vec::new());
        for j in 0..mat2.len() {
            let mut sum = 0.0;
            for k in 0..mat1.len() {
                sum = sum + (mat1[j][k] * mat2[j][k]);
            }
            out[i].push(sum);
        }
    }
    out
}
