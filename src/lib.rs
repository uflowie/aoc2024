use std::collections::HashMap;

pub mod template;

pub fn rows<T>(v: &Vec<Vec<T>>) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    v.iter().map(|row| row.iter())
}

pub fn columns<T>(v: &Vec<Vec<T>>) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    let rows = v.len();
    let cols = if rows > 0 { v[0].len() } else { 0 };

    (0..cols)
        .map(|col_idx| v.iter().map(move |row| &row[col_idx]))
        .map(|x| x)
}

pub fn major_diagonals<T>(v: &Vec<Vec<T>>) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    let rows = v.len();
    let cols = if rows > 0 { v[0].len() } else { 0 };

    (0..rows + cols - 1).map(move |diag| {
        (0..rows)
            .filter_map(move |row| {
                let col = diag as isize - row as isize;
                if col >= 0 && (col as usize) < cols {
                    Some(&v[row][col as usize])
                } else {
                    None
                }
            })
            .map(|x| x)
    })
}

pub fn minor_diagonals<T>(v: &Vec<Vec<T>>) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    let rows = v.len();
    let cols = if rows > 0 { v[0].len() } else { 0 };

    (0..rows + cols - 1)
        .map(move |diag| {
            (0..rows).filter_map(move |row| {
                let col = diag as isize - (rows - 1 - row) as isize;
                if col >= 0 && (col as usize) < cols {
                    Some(&v[row][col as usize])
                } else {
                    None
                }
            })
        })
        .map(|x| x)
}

pub fn indexed_chars(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as i32, j as i32), ch))
        })
        .collect()
}
