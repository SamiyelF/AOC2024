use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (rows, cols) = (input.len(), input[0].len());
    let mut xmas_count = 0;
    let mut x_mas_count = 0;

    // Count XMAS patterns
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == 'X' {
                xmas_count += count_xmas_patterns(&input, (i, j), (rows, cols));
            }
            // Check X-MAS patterns (only for inner cells)
            if (i > 0 && i < rows - 1) && (j > 0 && j < cols - 1) && (input[i][j] == 'A') {
                x_mas_count += check_x_pattern(&input, (i, j));
            }
        }
    }

    println!("Number of XMAS occurrences: {xmas_count}");
    println!("Number of X-MAS occurrences: {x_mas_count}");
}

fn count_xmas_patterns(grid: &[Vec<char>], index: (usize, usize), size: (usize, usize)) -> i32 {
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
    ];
    let target = ['X', 'M', 'A', 'S'];

    directions
        .iter()
        .map(|&(di, dj)| {
            (0..4).all(|k| {
                let (ni, nj) = (
                    index.0 as i32 + k as i32 * di,
                    index.1 as i32 + k as i32 * dj,
                );
                ni >= 0
                    && ni < size.0 as i32
                    && nj >= 0
                    && nj < size.1 as i32
                    && grid[ni as usize][nj as usize] == target[k as usize]
            }) as i32
        })
        .sum()
}

fn check_x_pattern(grid: &[Vec<char>], index: (usize, usize)) -> i32 {
    let is_mas =
        |first: char, last: char| (first == 'M' && last == 'S') || (first == 'S' && last == 'M');

    let diag1 = is_mas(
        grid[index.0 - 1][index.1 - 1],
        grid[index.0 + 1][index.1 + 1],
    );
    let diag2 = is_mas(
        grid[index.0 - 1][index.1 + 1],
        grid[index.0 + 1][index.1 - 1],
    );

    (diag1 && diag2 && grid[index.0][index.1] == 'A') as i32
}
