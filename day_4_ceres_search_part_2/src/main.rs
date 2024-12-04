use std::fs;

type Grid = Vec<Vec<char>>;

fn read_grid_from_file(filepath: &str) -> Result<Grid, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filepath)?;
    Ok(contents
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn validate_grid(grid: &Grid) -> Result<(), String> {
    if grid.is_empty() {
        return Err("empty grid".to_string());
    }

    let width = grid[0].len();
    if !grid.iter().all(|row| row.len() == width) {
        return Err("inconsistent row lengths".to_string());
    }

    Ok(())
}

fn check_mas_at_position(grid: &Grid, row: usize, col: usize, down_right: bool) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    if row + 2 >= height || col + 2 >= width {
        return false;
    }

    let word: String = (0..3)
        .map(|i| {
            let new_row = row + i;
            let new_col = if down_right { col + i } else { col + 2 - i };
            grid[new_row][new_col]
        })
        .collect();

    word == "MAS" || word == "SAM"
}

fn count_x_mas_patterns(grid: &Grid) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for row in 0..height.saturating_sub(2) {
        for col in 0..width.saturating_sub(2) {
            if check_mas_at_position(grid, row, col, true)
                && check_mas_at_position(grid, row, col, false)
            {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_grid_from_file("input")?;
    validate_grid(&grid).map_err(|e| format!("error: {}", e))?;
    println!("{}", count_x_mas_patterns(&grid));
    Ok(())
}
