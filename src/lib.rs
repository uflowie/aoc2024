use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

pub mod template;

pub static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[+-]?\d+").unwrap());

pub const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn get_index_neighbors(idx: (i32, i32)) -> [(i32, i32); 4] {
    [
        add(idx, DIRECTIONS[0]),
        add(idx, DIRECTIONS[1]),
        add(idx, DIRECTIONS[2]),
        add(idx, DIRECTIONS[3]),
    ]
}

pub fn add(left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
    (left.0 + right.0, left.1 + right.1)
}

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

pub fn indexed_chars_iter(input: &str) -> impl Iterator<Item = (i32, i32, char)> + '_ {
    input.lines().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .map(move |(j, ch)| (i as i32, j as i32, ch))
    })
}

pub fn indexed_chars(input: &str) -> HashMap<(i32, i32), char> {
    indexed_chars_iter(input)
        .map(|(i, j, ch)| ((i, j), ch))
        .collect()
}

pub fn find_char_index(chars: &HashMap<(i32, i32), char>, ch: char) -> Option<(i32, i32)> {
    chars
        .iter()
        .filter_map(|(idx, v)| if v == &ch { Some(*idx) } else { None })
        .next()
}

pub fn bounds(input: &str) -> (i32, i32) {
    let lines: Vec<_> = input.lines().collect();
    (lines.len() as i32, lines[0].len() as i32)
}
