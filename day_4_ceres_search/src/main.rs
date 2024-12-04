use std::fs;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (i32, i32);
type SearchResult = (Position, &'static str);

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

fn get_search_directions() -> [(Direction, &'static str); 8] {
    [
        ((0, 1), "R"),
        ((1, 1), "DR"),
        ((1, 0), "D"),
        ((1, -1), "DL"),
        ((0, -1), "L"),
        ((-1, -1), "UL"),
        ((-1, 0), "U"),
        ((-1, 1), "UR"),
    ]
}

fn is_within_grid_bounds(
    pos: Position,
    dir: Direction,
    grid_size: Position,
    word_len: i32,
) -> bool {
    let (row, col) = pos;
    let (dy, dx) = dir;
    let (height, width) = grid_size;
    let end_row = row as i32 + dy * (word_len - 1);
    let end_col = col as i32 + dx * (word_len - 1);

    end_row >= 0 && end_row < height as i32 && end_col >= 0 && end_col < width as i32
}

fn extract_word(grid: &Grid, start: Position, dir: Direction, length: usize) -> String {
    let (row, col) = start;
    let (dy, dx) = dir;

    (0..length)
        .map(|i| {
            let new_row = (row as i32 + dy * i as i32) as usize;
            let new_col = (col as i32 + dx * i as i32) as usize;
            grid[new_row][new_col]
        })
        .collect()
}

fn find_all_xmas_instances(grid: &Grid) -> Vec<SearchResult> {
    let height = grid.len();
    let width = grid[0].len();
    let mut found = Vec::new();

    for row in 0..height {
        for col in 0..width {
            for &(dir, direction_name) in &get_search_directions() {
                if is_within_grid_bounds((row, col), dir, (height, width), 4) {
                    let word = extract_word(grid, (row, col), dir, 4);
                    if word == "XMAS" {
                        found.push(((row, col), direction_name));
                    }
                }
            }
        }
    }
    found
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_grid_from_file("input")?;
    validate_grid(&grid).map_err(|e| format!("error: {}", e))?;

    let results = find_all_xmas_instances(&grid);
    println!("{}", results.len());

    Ok(())
}
