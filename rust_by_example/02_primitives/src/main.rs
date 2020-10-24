use std::fmt;
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Matrix {
    fn transpose(&mut self) {
        let tmp = self.1;
        self.1 = self.2;
        self.2 = tmp;
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "[{},\t{}]\n[{},\t{}]",
            &self.0, &self.1, &self.2, &self.3
        );
    }
}

fn transpose(input_mat: &Matrix) -> Matrix {
    return Matrix(input_mat.0, input_mat.2, input_mat.1, input_mat.3);
}

fn analyze_slice(slice: &[i32]) {
    println!("length of slice: {}", slice.len());
    println!("size (memory) of slice {}", mem::size_of_val(&slice));
    println!("slice: {:?}\n", slice);
}

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    // replacing 1i32 with 1u32 would throw an overflow error
    println!("1 - 2 = {}", 1i32 - 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // tuples with different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("{:?}", long_tuple);
    println!("first value {}", long_tuple.0);
    println!("second value {}", long_tuple.1);

    let test_pair = (345, false);
    println!("{:?}, reverse: {:?}", test_pair, reverse(test_pair));
    println!("");

    let mut mat = Matrix(43543.0, 5454.0, 3232.0, 787.0);
    println!("{:?}\n", mat);
    println!("{}", mat);
    mat.transpose();
    println!("transposed matrix method:\n{}\n", mat);
    println!("transposed matrix function:\n{}\n", transpose(&mat));
    println!("\n");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    analyze_slice(&xs);
    analyze_slice(&ys);
    analyze_slice(&ys[1..4]);
}
