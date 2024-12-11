use nalgebra::{DMatrix, DVector};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_csv_matrix(path: &str) -> DMatrix<f32> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Vec<Vec<f32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let rows = data.len();
    let cols = data[0].len();

    DMatrix::from_vec(rows, cols, data.into_iter().flatten().collect())
}

pub fn load_csv_vector(path: &str) -> DVector<i32> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    DVector::from_vec(data)
}

#[test]
fn test_loadmnist() {
    // Load train images as a matrix (each row is a flattened image)
    let train_images = load_csv_matrix("./asset/dataset/train_images.csv");

    // Load train labels as a vector
    let train_labels = load_csv_vector("./asset/dataset/train_labels.csv");

    println!(
        "Train Images Shape: {} x {}",
        train_images.nrows(),
        train_images.ncols()
    );
    println!("Train Labels Shape: {}", train_labels.len());
}
