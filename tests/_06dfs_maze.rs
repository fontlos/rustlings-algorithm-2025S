use std::collections::HashSet;

struct Maze {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };
        Maze { grid, rows, cols }
    }

    fn solve(&self, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        if self.dfs(start, end, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    fn dfs(
        &self,
        current: (usize, usize),
        end: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        path: &mut Vec<(usize, usize)>,
    ) -> bool {
        if current == end {
            path.push(current);
            return true;
        }

        if visited.contains(&current) || self.grid[current.0][current.1] == '#' {
            return false;
        }

        visited.insert(current);
        path.push(current);

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions.iter() {
            let r = current.0 as i32 + dr;
            let c = current.1 as i32 + dc;

            if r >= 0 && r < self.rows as i32 && c >= 0 && c < self.cols as i32 {
                let next = (r as usize, c as usize);
                if self.dfs(next, end, visited, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }
}

#[test]
fn test_dfs_maze() {
    let grid = vec![
        vec!['S', '.', '.', '#', '.'],
        vec!['#', '#', '.', '#', '.'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '#', '#', '#', '.'],
        vec!['.', '.', '.', '.', 'E'],
    ];

    let maze = Maze::new(grid);
    if let Some(path) = maze.solve((0, 0), (4, 4)) {
        println!("Find:");
        for (i, (r, c)) in path.iter().enumerate() {
            println!("Step {}: ({}, {})", i, r, c);
        }
    } else {
        println!("Failed to find a path.");
    }
}
