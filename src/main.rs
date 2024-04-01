extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::fs::File;
use ndarray::prelude::*;
use ndarray_csv::Array2Reader;
use csv::ReaderBuilder;
use ndarray::{Array2, Ix, OwnedRepr};
use num::complex::{Complex, ComplexFloat};

fn main() {
    let file = File::open("input.csv").unwrap();
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();

    // получение последнего столбца в отдельный массив
    let last_column_index = array_read.ncols() - 1;

    let (matrix, last_column) = array_read.view().split_at(Axis(1), last_column_index);

    let last_column = last_column.to_shape(3).unwrap();

    // проверка на квадратность
    if !matrix.is_square() {
        println!("Matrix should be square-shaped!");
        return;
    }

    // проверка матрицы на симметричность
    if matrix != matrix.t() {
        println!("Matrix should be symmetric!");
        return;
    }

    // промежуточная матрица
    let mut g: Array2<Complex<f64>> = Array2::zeros((last_column.len(), last_column.len()));

    // непосредственно нахождение промежуточной матрицы
    for ((i, j), value) in matrix.indexed_iter() {
        if i == j {
            if i == 0 {
                let g_1_1 = if *value > 0.0 { Complex::new(value.sqrt(), 0.0) } else { Complex::new(0.0, value.sqrt()) };
                g[[i, j]] = g_1_1;
                continue;
            }
            let g_i_i = (*value - compute_squared_column_sum(&g, i)).sqrt();
            g[[i, i]] = g_i_i;
            continue;
        }
        if i < j {
            g[[i, j]] = (Complex::new(*value, 0.0) - compute_column_sum(&g, i, j)) / g[[i, i]];
            continue;
        }

        g[[i, j]] = Complex::new(0.0, 0.0);
    }

    let len: usize = last_column.len();
    let mut y = Array::zeros(len);
    for (i, value) in last_column.indexed_iter() {
        y[i] = (Complex::new(*value, 0.0) - compute_y_sum(&g, i, &y)) / g[[i, i]];
    }

    let mut x = Array::zeros(len);
    for i in (0..len).rev() {
        x[i] = (y[i] - compute_x_sum(&g, i, &x, len)) / g[[i, i]];
    }

    println!("x array: {}", x.map(|el| { el.re() }));

    let error_vector = compute_error(&last_column, &x, &matrix);
    println!("error vector: {}", error_vector);
}


fn compute_squared_column_sum(matrix: &Array2<Complex<f64>>, index: Ix) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for i in 0..index {
        println!("{}", matrix[[i, index]]);
        sum += matrix[[i, index]].powi(2);
    }

    println!("diag {}", sum);
    sum
}

fn compute_column_sum(matrix: &Array2<Complex<f64>>, index: Ix, second_index: Ix) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for i in 0..index {
        sum += matrix[[i, index]] * matrix[[i, second_index]];
    }
    sum
}

fn compute_y_sum(matrix: &Array2<Complex<f64>>, index: Ix, second_matrix: &ArrayBase<OwnedRepr<Complex<f64>>, Ix1>) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for k in 0..index {
        sum += matrix[[k, index]] * second_matrix[[k]];
    }
    sum
}

fn compute_x_sum(g: &Array2<Complex<f64>>, index: Ix, x: &ArrayBase<OwnedRepr<Complex<f64>>, Ix1>, len: Ix) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for k in index..len {
        sum += g[[index, k]] * x[[k]];
    }
    sum
}

fn compute_error(
    actual_solution: &CowArray<f64, Ix1>,
    x_computed: &ArrayBase<OwnedRepr<Complex<f64>>, Ix1>,
    coefficients_matrix: &ArrayView<f64, Ix2>,
) -> Array<f64, Ix1> {
    let mut computed_answer: ArrayBase<OwnedRepr<f64>, Ix1> = Array::zeros(actual_solution.len());

    for i in 0..actual_solution.len() {
        for j in 0..actual_solution.len() {
            computed_answer[j] += coefficients_matrix[[i, j]].re() * x_computed[i].re();
        }
    }
    println!("{}", computed_answer);

    actual_solution - &computed_answer
}