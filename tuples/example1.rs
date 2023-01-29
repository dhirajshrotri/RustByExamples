// The following struct is for the activity.
use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {}) \n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn reverse(matrix: Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    let Matrix(f1, f2, f3, f4) = matrix;

    Matrix(f1, f3, f2, f4)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix: \n{}", matrix);
    println!("\n");
    println!("Transpose: \n{}", reverse(matrix));
}

