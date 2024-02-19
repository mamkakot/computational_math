use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let mut array = read_array_from_file(Path::new("src/input.txt"));
    println!("{}", check_for_symmetry(&array));

    let last_column_index = array.first().unwrap().len() - 1;

    let col = array.iter()
        .map(|s| s.iter().nth(last_column_index).unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", col);
}

// чтение матрицы из указанного файла
fn read_array_from_file(path: &Path) -> Vec<Vec<f64>> {
    let mut f = BufReader::new(File::open(path).unwrap());

    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    let arr: Vec<Vec<f64>> = f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
            .map(|number|  number.parse().unwrap_or(0.0))
            .collect())
        .collect();

    println!("{:?}", arr);
    arr
}

// проверка матрицы на симметричность
fn check_for_symmetry(array: &Vec<Vec<f64>>) -> bool {
    for i in 0..array.len() {
        for j in 0..array.len() {
            if array[i][j] != array[j][i] {
                return false;
            }
        }
    };
    true
}