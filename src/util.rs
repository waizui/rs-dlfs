use nalgebra::{DMatrix, DVector, Scalar};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

pub fn load_matrix_csv<T>(path: &str) -> DMatrix<T>
where
    T: FromStr + Copy + Debug + Scalar,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Vec<Vec<T>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    // col major
    let rows = data[0].len();
    let cols = data.len();

    DMatrix::from_vec(rows, cols, data.into_iter().flatten().collect())
}

pub fn save_matrix_csv<T>(path: &str, mat: &DMatrix<T>)
where
    T: FromStr + Copy + Debug + Scalar + std::fmt::Display,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let file = File::create(path).expect("Unable to open file");
    let mut writer = BufWriter::new(file);

    let rows = mat.nrows();
    let cols = mat.ncols();

    // col major
    for c in 0..cols {
        for r in 0..rows {
            write!(writer, "{}", mat[(r, c)]).expect("Failed to write element");
            if r < rows - 1 {
                write!(writer, ",").expect("Failed to write comma");
            }
        }
        writeln!(writer).expect("Failed to write newline");
    }

    writer.flush().expect("Failed to flush")
}

pub fn load_vector_csv<T>(path: &str) -> DVector<T>
where
    T: FromStr + Copy + Debug + Scalar,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let file = File::create(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Vec<T> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    DVector::from_vec(data)
}

pub fn save_vector_csv<T>(path: &str, vec: &DVector<T>)
where
    T: FromStr + Copy + Debug + Scalar + std::fmt::Display,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let file = File::open(path).expect("Unable to open file");
    let mut writer = BufWriter::new(file);

    for r in 0..vec.len() {
        write!(writer, "{}", vec[r]).expect("Failed to write element");
        writeln!(writer).expect("Failed to write newline");
    }

    writer.flush().expect("Failed to flush")
}

#[test]
fn test_csv_io() {
    use nalgebra::DMatrix;

    let mat = DMatrix::from_vec(
        3,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ],
    );

    let path = "target/matrix.csv";

    save_matrix_csv(path, &mat);

    let loaded = load_matrix_csv::<f32>(path);

    for r in 0..mat.nrows() {
        for c in 0..mat.ncols() {
            assert_eq!(mat[(r, c)], loaded[(r, c)]);
        }
    }

}

pub fn loadmnist() {
    // each row is a flattened image
    let train_images: DMatrix<f32> = load_matrix_csv("asset/dataset/train_images.csv");

    // train labels as a vector
    let train_labels: DVector<i32> = load_vector_csv("asset/dataset/train_labels.csv");

    println!(
        "Train Images Shape: {} x {}",
        train_images.nrows(),
        train_images.ncols()
    );
    println!("Train Labels Shape: {}", train_labels.len());
}
