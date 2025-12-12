use std::collections::HashSet;
use std::fs;

fn read_txt_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open file: {}", path))
}

enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, Clone)]
struct Shape {
    point_diff: HashSet<(usize,usize)>,// relative position
}

#[derive(Debug, Clone)]
struct Matrix {
    occupied_points: HashSet<(usize,usize)>,
    max_x: usize,
    max_y: usize
}

#[derive(Debug)]
struct Puzzle {
    matrix_size: (usize, usize),
    shape_counts: Vec<usize>,
}

impl Shape {
    fn from_lines(lines: &[&str]) -> Self {
        let mut point_diff = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    point_diff.insert((x, y));
                }
            }
        }
        Shape { point_diff }
    }

    fn rotate(&self, rotations: usize) -> HashSet<(usize, usize)> {
        let mut result = self.point_diff.clone();

        for _ in 0..(rotations % 4) {
            // Rotate 90 degrees clockwise: (x, y) -> (y, -x)
            // First, we need to normalize coordinates
            result = result.iter().map(|&(x, y)| {
                (y, x)
            }).collect();

            // Normalize to start from (0, 0)
            if let Some(&min_x) = result.iter().map(|(x, _)| x).min() {
                if let Some(&min_y) = result.iter().map(|(_, y)| y).min() {
                    result = result.iter().map(|&(x, y)| {
                        (x - min_x, y - min_y)
                    }).collect();
                }
            }
        }

        result
    }

    fn place(&self, rotations: usize, position: &(usize,usize), matrix: &Matrix) -> Option<Matrix> {
        let rotated = self.rotate(rotations);
        place_shape(&rotated, position, matrix)
    }
}

impl Matrix {
    fn new(max_x: usize, max_y: usize) -> Self {
        Matrix {
            occupied_points: HashSet::new(),
            max_x,
            max_y
        }
    }

    fn print(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.occupied_points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Puzzle>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut shapes = Vec::new();
    let mut puzzles = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Parse shape definitions (format: "N:")
        if line.ends_with(':') && line.len() >= 2 {
            // Shape definition starts
            let mut shape_lines = Vec::new();
            i += 1;

            // Collect all lines until we hit an empty line or another shape/puzzle definition
            while i < lines.len() {
                let current = lines[i].trim();
                if current.is_empty() || current.ends_with(':') {
                    break;
                }
                shape_lines.push(lines[i]);
                i += 1;
            }

            if !shape_lines.is_empty() {
                shapes.push(Shape::from_lines(&shape_lines));
            }
        }
        // Parse puzzle definitions (format: "WxH: n1 n2 n3...")
        else if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                // Parse dimensions
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                if dims.len() == 2 {
                    let width = dims[0].parse::<usize>().unwrap();
                    let height = dims[1].parse::<usize>().unwrap();

                    // Parse shape counts
                    let counts: Vec<usize> = parts[1]
                        .trim()
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();

                    puzzles.push(Puzzle {
                        matrix_size: (width, height),
                        shape_counts: counts,
                    });
                }
            }
            i += 1;
        }
        else {
            i += 1;
        }
    }

    (shapes, puzzles)
}

fn solve_puzzle(shapes: &[Shape], puzzle: &Puzzle) -> Option<Matrix> {
    let matrix = Matrix::new(puzzle.matrix_size.0, puzzle.matrix_size.1);

    // Create a list of shapes to place based on counts
    let mut shapes_to_place = Vec::new();
    for (shape_idx, &count) in puzzle.shape_counts.iter().enumerate() {
        for _ in 0..count {
            shapes_to_place.push(shape_idx);
        }
    }

    // Pre-compute all rotations for each shape
    let mut shape_rotations = Vec::new();
    for shape in shapes {
        let mut rotations = Vec::new();
        for r in 0..4 {
            rotations.push(shape.rotate(r));
        }
        shape_rotations.push(rotations);
    }

    println!("Solving puzzle with {} shapes to place in {}x{} matrix",
             shapes_to_place.len(), puzzle.matrix_size.0, puzzle.matrix_size.1);

    backtrack(&shape_rotations, &shapes_to_place, 0, matrix)
}

fn backtrack(shape_rotations: &[Vec<HashSet<(usize, usize)>>], shapes_to_place: &[usize], idx: usize, matrix: Matrix) -> Option<Matrix> {
    // Base case: all shapes placed
    if idx >= shapes_to_place.len() {
        return Some(matrix);
    }

    // Progress indicator
    if idx % 10 == 0 && idx > 0 {
        println!("Placed {}/{} shapes...", idx, shapes_to_place.len());
    }

    let shape_idx = shapes_to_place[idx];
    let rotations = &shape_rotations[shape_idx];

    // Find first empty cell to constrain search space
    let start_pos = find_first_empty(&matrix);

    // Try all rotations (0, 90, 180, 270 degrees)
    for rotation in 0..4 {
        let rotated = &rotations[rotation];

        // Try positions starting from the first empty cell
        for y in start_pos.1..matrix.max_y {
            let start_x = if y == start_pos.1 { start_pos.0 } else { 0 };
            for x in start_x..matrix.max_x {
                if let Some(new_matrix) = place_shape(rotated, &(x, y), &matrix) {
                    // Recursively try to place remaining shapes
                    if let Some(solution) = backtrack(shape_rotations, shapes_to_place, idx + 1, new_matrix) {
                        return Some(solution);
                    }
                }
            }
        }
    }

    None
}

fn find_first_empty(matrix: &Matrix) -> (usize, usize) {
    for y in 0..matrix.max_y {
        for x in 0..matrix.max_x {
            if !matrix.occupied_points.contains(&(x, y)) {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn place_shape(rotated: &HashSet<(usize, usize)>, position: &(usize, usize), matrix: &Matrix) -> Option<Matrix> {
    let points: HashSet<(usize, usize)> = rotated
        .iter()
        .map(|p| (p.0 + position.0, p.1 + position.1))
        .collect();

    // Check bounds
    for &(x, y) in &points {
        if x >= matrix.max_x || y >= matrix.max_y {
            return None;
        }
    }

    // Check if disjoint (no overlap)
    if points.is_disjoint(&matrix.occupied_points) {
        return Some(Matrix {
            occupied_points: matrix.occupied_points.union(&points).cloned().collect(),
            max_x: matrix.max_x,
            max_y: matrix.max_y
        });
    }
    None
}

fn main() {
    let input: String = read_txt_file("input.txt");
    let (shapes, puzzles) = parse_input(&input);

    println!("Parsed {} shapes", shapes.len());
    for (idx, shape) in shapes.iter().enumerate() {
        println!("Shape {}: {:?}", idx, shape.point_diff);
    }

    println!("\nParsed {} puzzles\n", puzzles.len());

    for (idx, puzzle) in puzzles.iter().enumerate() {
        println!("=== Puzzle {} ===", idx + 1);
        println!("Size: {}x{}", puzzle.matrix_size.0, puzzle.matrix_size.1);
        println!("Shape counts: {:?}", puzzle.shape_counts);

        match solve_puzzle(&shapes, puzzle) {
            Some(solution) => {
                println!("Solution found:");
                solution.print();
            }
            None => {
                println!("No solution found!");
            }
        }
        println!();
    }
}