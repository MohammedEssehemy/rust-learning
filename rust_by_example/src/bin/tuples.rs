use std::fmt::{Display, format};

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        writeln!(f, "( {} {} )", self.2, self.3)
    }
}


fn transpose(matrix: Matrix) -> String {
    let m = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    format!("{m}")
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
    /*
        Matrix:
        ( 1.1 1.2 )
        ( 2.1 2.2 )
        Transpose:
        ( 1.1 2.1 )
        ( 1.2 2.2 )
    */
}

