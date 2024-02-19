extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::fs::File;
use ndarray::prelude::*;
use ndarray_csv::{Array2Reader};
use csv::ReaderBuilder;
use ndarray::Array2;

fn main() {
    let file = File::open("input.csv").unwrap();
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    println!("{}", array_read);

    // получение последнего столбца в отдельный массив
    let last_column_index = array_read.ncols() - 1;
    let last_column = array_read.column(last_column_index);
    println!("{}", last_column);

    // получение матрицы без последнего столбца
    let matrix = array_read.slice(s![.., 0..last_column_index]);

    println!("{}", matrix.column(1));
    println!("{}", check_for_symmetry(&matrix))
}

// проверка матрицы на симметричность
fn check_for_symmetry(matrix: &ArrayView<f64, Dim<[usize; 2]>>) -> bool {
    matrix == matrix.t()
}