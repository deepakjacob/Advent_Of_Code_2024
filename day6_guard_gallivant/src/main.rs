use std::collections::HashSet;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self { cells }
    }

    fn find_start(&self) -> Pos {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    return Pos::new(x as i32, y as i32);
                }
            }
        }
        panic!("no start position found");
    }

    fn is_valid(&self, pos: &Pos) -> bool {
        pos.y >= 0
            && pos.y < self.cells.len() as i32
            && pos.x >= 0
            && pos.x < self.cells[0].len() as i32
    }

    fn is_blocked(&self, pos: &Pos) -> bool {
        self.cells[pos.y as usize][pos.x as usize] == '#'
    }

    fn visualize_path(&self, visited: &HashSet<(i32, i32)>) -> String {
        let mut result = String::new();
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                if self.cells[y][x] == '#' {
                    result.push('#');
                } else if visited.contains(&(x as i32, y as i32)) {
                    result.push('X');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        result
    }

    fn count_visited_positions(&self) -> usize {
        let mut visited = HashSet::new();
        let mut pos = self.find_start();
        let mut dir = Direction::Up;

        visited.insert((pos.x, pos.y));

        loop {
            let next_pos = pos.next(dir);

            if !self.is_valid(&next_pos) {
                // println!("Final path:");
                // print!("{}", self.visualize_path(&visited));
                break;
            }

            if self.is_blocked(&next_pos) {
                dir = dir.turn_right();
            } else {
                pos = next_pos;
                visited.insert((pos.x, pos.y));
            }
        }

        visited.len()
    }
}

fn main() {
    // verify with example first
    let example = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let grid = Grid::new(example);
    let count = grid.count_visited_positions();
    println!("Example visited {} positions", count);
    assert_eq!(count, 41);

    // process actual input
    let input = std::fs::read_to_string("input").expect("failed to read input file");
    let grid = Grid::new(&input);
    let result = grid.count_visited_positions();
    println!(
        "The guard visited {} positions before leaving the mapped area.",
        result
    );
}
