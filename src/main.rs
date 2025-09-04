#![allow(dead_code)]
use rand::prelude::*;

#[derive(Copy, Clone, Default)]
struct Cell {
    up: bool,
    right: bool,
    dist: Option<u32>,
    path: Option<bool>,
}

impl Cell {
    fn blank() -> Self {
        Cell {
            up: false,
            right: false,
            dist: None,
            path: None,
        }
    }

    fn new(up: bool, right: bool) -> Self {
        Cell {
            up,
            right,
            dist: None,
            path: None,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
struct Maze<const S: usize> {
    cells: [[Cell; S]; S],
}

static CLOSED_CELL: Cell = Cell {
    up: false,
    right: false,
    dist: None,
    path: None,
};

impl<const S: usize> Default for Maze<S> {
    fn default() -> Self {
        Maze {
            cells: [[Cell::default(); S]; S],
        }
    }
}

impl<const S: usize> Maze<S> {
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        //let index = y * S + x;
        // println!("{x} {y} {index}**");
        self.cells[y][x] = cell;
    }
    fn at_opt(&self, x: usize, y: usize) -> Option<&Cell> {
        //let index = y * S + x;
        // println!("{x} {y} {index}**");
        if x < S && y < S {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }
    fn at(&self, x: usize, y: usize) -> &Cell {
        //let index = y * S + x;
        // println!("{x} {y} {index}**");
        &self.cells[y][x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    fn can_go(&self, x: usize, y: usize, direction: Direction) -> bool {
        match direction {
            Direction::North => {
                if y >= S - 1 {
                    false
                } else {
                    self.at(x, y).up
                }
            }
            Direction::East => {
                if x >= S - 1 {
                    false
                } else {
                    self.at(x, y).right
                }
            }
            Direction::South => {
                if y == 0 {
                    false
                } else {
                    self.at_opt(x, y - 1).unwrap_or(&CLOSED_CELL).up
                }
            }
            Direction::West => {
                if x == 0 {
                    false
                } else {
                    self.at_opt(x - 1, y).unwrap_or(&CLOSED_CELL).right
                }
            }
        }
    }

    fn print(&self) {
        let has_path = self.cells.iter().flatten().any(|cell| cell.path.is_some());
        for y in 0..S {
            // Top
            if y == 0 {
                print!("╔");
            } else if self.at(0, S - y - 1).up {
                print!("║");
            } else {
                print!("╟");
            }
            for x in 0..S {
                let left = self.at(x, S - y - 1).up;
                let bottom = self.at(x, S - y - 1).right;
                let right = self.at_opt(x + 1, S - y - 1).unwrap_or(&CLOSED_CELL).up;
                let top = self.at_opt(x, S - y).unwrap_or(&CLOSED_CELL).right;
                if y == 0 && x == S - 1 {
                    print!("═══╗")
                } else if y == 0 {
                    if bottom {
                        print!("════")
                    } else {
                        print!("═══╤")
                    }
                } else if x == S - 1 {
                    if left {
                        print!("   ║");
                    } else {
                        print!("───╢");
                    }
                } else {
                    match (left, bottom, right, top) {
                        (false, false, false, false) => print!("───┼"),
                        (false, false, false, true) => print!("───┬"),
                        (false, false, true, false) => print!("───┤"),
                        (false, false, true, true) => print!("───╮"),
                        (false, true, false, false) => print!("───┴"),
                        (false, true, false, true) => print!("────"),
                        (false, true, true, false) => print!("───╯"),
                        (false, true, true, true) => print!("─── "),
                        (true, false, false, false) => print!("   ├"),
                        (true, false, false, true) => print!("   ╭"),
                        (true, false, true, false) => print!("   │"),
                        (true, false, true, true) => print!("   │"),
                        (true, true, false, false) => print!("   ╰"),
                        (true, true, false, true) => print!("   ─"),
                        (true, true, true, false) => print!("   │"),
                        (true, true, true, true) => print!("    "),
                    };
                }
            }

            println!();
            print!("║");
            for x in 0..S {
                let dist: Option<u32> = self.at(x, S - y - 1).dist;
                let path: Option<bool> = self.at(x, S - y - 1).path;
                let dist_char: String = if has_path && path == Some(true) {
                    //
                    //
                    //
                    format!("{: >2}", dist.unwrap())
                } else if has_path {
                    "  ".to_string()
                } else if dist.is_some() {
                    format!("{: >2}", dist.unwrap())
                } else {
                    "  ".to_string()
                };

                if x == 0 && y == S - 1 && self.at(x, S - y - 1).right {
                    print!("END ");
                } else if x == 0 && y == S - 1 {
                    print!("END│");
                } else if x == S - 1 && y == 0 {
                    print!("STA║");
                } else if self.at(x, S - y - 1).right {
                    print!("{dist_char}  ");
                } else if x == S - 1 {
                    print!("{dist_char} ║");
                } else {
                    print!("{dist_char} │");
                }
            }
            println!()
        }
        print!("╚");
        for x in 0..S {
            if x == S - 1 {
                print!("═══╝");
            } else if self.at(x, 0).right {
                print!("════");
            } else {
                print!("═══╧");
            }
        }
        println!();
    }

    fn binary_tree(mut self) -> Self {
        let mut rng = rand::rng();
        for x in 0..S {
            for y in 0..S {
                let (up, right) = if x == S - 1 && y == S - 1 {
                    (false, false)
                } else if x == S - 1 {
                    (true, false)
                } else if y == S - 1 {
                    (false, true)
                } else if rng.random::<bool>() {
                    (true, false)
                } else {
                    (false, true)
                };

                self.set(x, y, Cell::new(up, right))
            }
        }
        self
    }

    fn sidewinder(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            let mut run: usize = 0;
            for x in 0..S {
                run += 1;
                let (up, right) = if x == S - 1 && y == S - 1 {
                    // Top Right -> No more
                    (false, false)
                } else if x == S - 1 {
                    // Right Wall -> Only Up
                    (true, false)
                } else if y == S - 1 || rng.random::<bool>() {
                    // Top Wall or Continue run
                    (false, true)
                } else {
                    // End run and pick random location to go up
                    let pick: usize = rng.random_range(0..run);
                    run = 0;
                    if pick == 0 {
                        // This is the cell to go up
                        (true, false)
                    } else {
                        // Open up top of chosen cell
                        self.at_mut(x - pick, y).up = true;
                        // Close off current cell
                        (false, false)
                    }
                };

                self.set(x, y, Cell::new(up, right))
            }
        }
        self
    }
    fn random(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(x, y, Cell::new(rng.random(), rng.random()));
            }
        }
        self
    }

    fn rights(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(x, y, Cell::new(rng.random(), true))
            }
        }
        self
    }

    fn ups(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(x, y, Cell::new(true, rng.random()));
            }
        }
        self
    }

    fn shortist_path(mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Self {
        let mut x = end_x;
        let mut y = end_y;
        let mut limit = 0;
        while x != start_x || y != start_y {
            limit += 1;
            if limit > 1000 {
                println!("Looping in shortest path");
                return self;
            }
            let current = self.at_mut(x, y);
            current.path = Some(true);
            let dist = current.dist.expect("Maze should have a current distance");

            let nexts = [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1), //
            ];
            let nexts_dir = [
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::North,
            ];
            let next_step = nexts
                .iter()
                .zip(nexts_dir)
                .filter(|(_, dir)| self.can_go(x, y, *dir))
                .map(|(pos, _)| pos)
                .filter(|(x, y)| self.at_opt(*x, *y).is_some())
                .find(|(x, y)| self.at(*x, *y).dist.unwrap() == dist.saturating_sub(1));
            if next_step.is_none() {
                println!("Failed to find shortest path");
                return self;
            }
            x = next_step.unwrap().0;
            y = next_step.unwrap().1;
        }
        self
    }

    fn calc_dist(mut self, x: usize, y: usize) -> Self {
        self.cells.iter_mut().flatten().for_each(|cell| cell.dist = None);
        let mut next = vec![(x, y)];
        self.at_mut(x, y).dist = Some(0);
        let mut _count = 0;
        while let Some((x, y)) = next.pop() {
            //dbg!(&next);
            //dbg!(&next);
            let cell = self.at_opt(x, y);
            if cell.is_none() {
                continue;
            }
            if cell.unwrap().dist.is_none() {
                continue;
            }
            let dist = cell.unwrap().dist.unwrap();
            //println!("{x} {y} {dist}");
            if x >= S {
                continue;
            }
            if y >= S {
                continue;
            }
            if self.can_go(x, y, Direction::North) && self.at(x, y + 1).dist.is_none() {
                self.at_mut(x, y + 1).dist = Some(dist + 1);
                next.push((x, y + 1));
            }

            if self.can_go(x, y, Direction::South) && self.at(x, y - 1).dist.is_none() {
                self.at_mut(x, y - 1).dist = Some(dist + 1);
                next.push((x, y - 1));
            }

            if self.can_go(x, y, Direction::West) && self.at(x - 1, y).dist.is_none() {
                self.at_mut(x - 1, y).dist = Some(dist + 1);
                next.push((x - 1, y));
            }

            if self.can_go(x, y, Direction::East) && self.at(x + 1, y).dist.is_none() {
                self.at_mut(x + 1, y).dist = Some(dist + 1);
                next.push((x + 1, y));
            }
        }

        self
    }

    fn calc_longest(&mut self) -> (usize, usize, usize, usize) {
        *self = self.calc_dist(0, 0);
        self.print();
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max = 0;
        for x in 0..S {
            for y in 0..S {
                if self.at(x, y).dist.unwrap_or_default() > max {
                    max_x = x;
                    max_y = y;
                    max = self.at(x, y).dist.unwrap_or_default();
                }
            }
        }
        println!("first pass: {max_x}, {max_y}");
        *self = self.calc_dist(max_x, max_y);
        self.print();
        max = 0;
        for x in 0..S {
            for y in 0..S {
                if self.at(x, y).dist.unwrap_or_default() > max {
                    max_x = x;
                    max_y = y;
                    max = self.at(x, y).dist.unwrap_or_default();
                }
            }
        }
        let start_x = max_x;
        let start_y = max_y;

        *self = self.calc_dist(max_x, max_y);
        self.print();
        max = 0;
        for x in 0..S {
            for y in 0..S {
                if self.at(x, y).dist.unwrap_or_default() > max {
                    max_x = x;
                    max_y = y;
                    max = self.at(x, y).dist.unwrap_or_default();
                }
            }
        }
        println!("Result: {start_x}, {start_y}, {max_x}, {max_y}");
        (start_x, start_y, max_x, max_y)
    }
}

fn main() {
    let mut maze = Maze::<15>::default().sidewinder();

    let (x1, y1, x2, y2) = maze.calc_longest();
    println!("{x1}, {y1} -> {x2}, {y2}");

    maze = maze.calc_dist(x1, y1);
    println!("{x1}, {y1} -> {x2}, {y2}");
    maze.shortist_path(x1, y1, x2, y2).print();
}
