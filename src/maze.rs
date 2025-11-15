#![allow(dead_code)]
use std::fmt::Display;

use rand::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct Cell {
    pub up: bool,
    pub right: bool,
    pub dist: Option<u32>,
    pub path: Option<bool>,
    pub masked: bool,
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    pub fn shift(self, direction: Direction) -> Option<Pos> {
        Some(match direction {
            Direction::North => Pos::new(self.x, self.y + 1),
            Direction::East => Pos::new(self.x + 1, self.y),
            Direction::South => Pos::new(self.x, self.y.checked_sub(1)?),
            Direction::West => Pos::new(self.x.checked_sub(1)?, self.y),
        })
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl Cell {
    pub fn blank() -> Self {
        Cell {
            up: false,
            right: false,
            dist: None,
            path: None,
            masked: false,
        }
    }

    pub fn new(up: bool, right: bool) -> Self {
        Cell {
            up,
            right,
            dist: None,
            path: None,
            masked: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {}

const ALL: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

#[derive(Copy, Clone)]
pub struct Maze<const S: usize> {
    pub cells: [[Cell; S]; S],
    pub start: Pos,
    pub end: Pos,
}

pub static CLOSED_CELL: Cell = Cell {
    up: false,
    right: false,
    dist: None,
    path: None,
    masked: false,
};

impl<const S: usize> Default for Maze<S> {
    fn default() -> Self {
        Maze {
            cells: [[Cell::default(); S]; S],
            start: Pos::default(),
            end: Pos::new(S - 1, S - 1),
        }
    }
}

impl<const S: usize> Maze<S> {
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.at_opt(x, y).expect("Looking to set cell at ({x},{y})");
        self.cells[y][x] = cell;
    }
    pub fn at_opt(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y)?.get(x)
    }

    pub fn at_pos(&self, pos: Pos) -> &Cell {
        self.at_pos_opt(pos).expect("Looking to set cell at {pos}");
        &self.cells[pos.y][pos.x]
    }

    pub fn at_pos_mut(&mut self, pos: Pos) -> &mut Cell {
        &mut self.cells[pos.y][pos.x]
    }

    pub fn at_pos_opt(&self, pos: Pos) -> Option<&Cell> {
        self.cells.get(pos.y)?.get(pos.x)
    }
    pub fn at(&self, x: usize, y: usize) -> &Cell {
        //let index = y * S + x;
        // println!("{x} {y} {index}**");
        &self.cells[y][x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    pub fn all_pos() -> impl Iterator<Item = Pos> {
        (0..S * S).map(|value| Pos::new(value % S, value / S))
    }

    pub fn pos_of(&self, cell: &Cell) -> Pos {
        Self::all_pos()
            // [TODO] Make a contant time method of getting pos from cell
            .find(|pos| std::ptr::eq(self.at_pos(*pos), cell))
            .unwrap()
    }

    /* Steps into valid unmasked cell else None */
    fn step(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let new_location = match direction {
            Direction::South => (x, y.checked_sub(1)?),
            Direction::West => (x.checked_sub(1)?, y),
            Direction::North => {
                if y >= S - 1 {
                    return None;
                } else {
                    (x, y + 1)
                }
            }
            Direction::East => {
                if x >= S - 1 {
                    return None;
                } else {
                    (x + 1, y)
                }
            }
        };
        if self.at_pos(Pos::new(new_location.0, new_location.1)).masked {
            return None;
        }
        Some(new_location)
    }

    pub fn can_go(&self, x: usize, y: usize, direction: Direction) -> bool {
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

    pub fn print(&self) {
        let has_path = self.cells.iter().flatten().any(|cell| cell.path.is_some());
        for y in 0..S {
            // Flip so re render from the top down.

            let y = S - 1 - y;
            // Top
            if y == S - 1 {
                print!("╔");
            } else if self.at(0, y).up {
                print!("║");
            } else {
                print!("╟");
            }
            for x in 0..S {
                let masked_sw = self.at(x, y).masked;

                let masked_se = self.at_opt(x + 1, y).unwrap_or(&CLOSED_CELL).masked;
                let masked_ne = self.at_opt(x + 1, y + 1).unwrap_or(&CLOSED_CELL).masked;
                let masked_nw = self.at_opt(x, y + 1).unwrap_or(&CLOSED_CELL).masked;

                let left = self.at(x, y).up || (masked_nw && masked_sw);
                let bottom = self.at(x, y).right || (masked_se && masked_sw);
                let right =
                    self.at_opt(x + 1, y).unwrap_or(&CLOSED_CELL).up || (masked_se && masked_ne);
                let top =
                    self.at_opt(x, y + 1).unwrap_or(&CLOSED_CELL).right || (masked_nw && masked_ne);
                if y == S - 1 && x == S - 1 {
                    print!("═══╗")
                } else if y == S - 1 {
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
                let dist: Option<u32> = self.at(x, y).dist;
                let path: Option<bool> = self.at(x, y).path;
                let masked = self.at(x, y).masked;

                let nexted_masked = self.at_opt(x + 1, y).unwrap_or(&CLOSED_CELL).masked;

                let dist_char: String = if masked {
                    "  ".to_string()
                } else if has_path && path == Some(true) {
                    if let Some(dist) = dist {
                        format!("{: >2}", dist)
                    } else {
                        "<>".to_string()
                    }
                } else if has_path {
                    "  ".to_string()
                } else if dist.is_some() {
                    format!("{: >2}", dist.unwrap())
                } else {
                    "  ".to_string()
                };

                if masked && nexted_masked {
                    print!("    ");
                } else if x == self.end.x && y == self.end.y && self.at(x, y).right {
                    print!("END ");
                } else if x == self.end.x && y == self.end.y && x == S - 1 {
                    print!("END║");
                } else if x == self.end.x && y == self.end.y {
                    print!("END│");
                } else if x == self.start.x
                    && y == self.start.y
                    && self.can_go(x, y, Direction::East)
                {
                    print!("STA ");
                } else if x == self.start.x && y == self.start.y && x == S - 1 {
                    print!("STA║");
                } else if x == self.start.x && y == self.start.y {
                    print!("STA│");
                } else if self.at(x, y).right {
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

    pub fn binary_tree(mut self) -> Self {
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

    pub fn sidewinder(mut self) -> Self {
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

    pub fn hunt_and_kill(mut self) -> Self {
        let mut _rng = rand::rng();
        // Hold list of all visited cells
        let mut visited_cells: [[bool; S]; S] = [[false; S]; S];
        Self::all_pos().for_each(|pos| visited_cells[pos.x][pos.y] |= self.at_pos(pos).masked);
        // Pick a first valid cells
        let _current = self.all_pos().find(|pos| !self.at_pos(pos).masked);
        visited_cells[_current.x][_current.y] = true;
        
        // Walk
        loop {
        let mut directions = ALL
            .iter()
            .filter(|direction| self.step(currentx, currenty, **direction).is_some())
            .collect::<Vec<_>>();
            directions.shuffle();



            if directions.is_empty() {
                break;
            }
        }
        // Add path


        directions.shuffle(&mut rng);


        self
    }

    pub fn walker(mut self) -> Self {
        self = self.clear();
        let mut known_cells: [[bool; S]; S] = [[false; S]; S];
        // Make once cell known
        let mut rng = rand::rng();
        known_cells[rng.random_range(0..S)][rng.random_range(0..S)] = true;

        // Set all masked cells as known
        Self::all_pos().for_each(|pos| known_cells[pos.x][pos.y] |= self.at_pos(pos).masked);

        let mut limit = 0;
        while known_cells.as_flattened().iter().any(|x| !*x) {
            limit += 1;
            if limit > 20 {
                //break;
            }
            let sta = known_cells
                .iter()
                .flatten()
                .enumerate()
                .map(|(index, known)| ((index / S, index % S), known))
                .filter(|(_, known)| !**known)
                .map(|(pos, _)| pos)
                .next()
                .unwrap();

            let mut currentx: usize = sta.0;
            let mut currenty: usize = sta.1;
            let current = (currentx, currenty);
            let mut path = vec![current];
            loop {
                let mut directions = ALL
                    .iter()
                    .filter(|direction| self.step(currentx, currenty, **direction).is_some())
                    .collect::<Vec<_>>();
                directions.shuffle(&mut rng);

                if directions.is_empty() {
                    println!("at ({}, {}) deadend", currentx, currenty);
                    break;
                }

                // [TODO] This will get stuck if it has a 1 cell wide masked dead end.
                let direction = directions.first().unwrap();
                //dbg!(direction, i);
                if let Some(next) = self.step(currentx, currenty, **direction) {
                    if let Some(index) = path.iter().position(|x| *x == next) {
                        path.truncate(index);
                    }
                    path.push(next);
                    if known_cells[next.0][next.1] {
                        break;
                    }
                    (currentx, currenty) = next;
                }
            }
            path.iter()
                .for_each(|(x, y)| self.at_mut(*x, *y).path = Some(true));
            let path_directions = path
                .iter()
                .as_slice()
                .windows(2)
                .map(|x| match x {
                    [] => panic!("Empty"),
                    [_] => panic!("One"),
                    [from, to, ..] => (to.0 + 10 - from.0, to.1 + 10 - from.1),
                })
                .map(|dir| match dir {
                    (10, 11) => Direction::North,
                    (11, 10) => Direction::West,
                    (10, 9) => Direction::South,
                    (9, 10) => Direction::East,
                    (x, y) => panic!("Unexpected ({x} {y})"),
                })
                .collect::<Vec<_>>();

            path.iter().zip(path_directions).for_each(|(pos, dir)| {
                known_cells[pos.0][pos.1] = true;
                match dir {
                    Direction::North => self.at_mut(pos.0, pos.1).up = true,
                    Direction::West => self.at_mut(pos.0, pos.1).right = true,
                    Direction::South => self.at_mut(pos.0, pos.1 - 1).up = true,
                    Direction::East => self.at_mut(pos.0 - 1, pos.1).right = true,
                }
            });
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

    pub fn shortist_path(mut self) -> Self {
        let mut x = self.end.x;
        let mut y = self.end.y;
        let current = self.end;
        let mut limit = 0;
        while current != self.start {
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

    pub fn calc_dist(mut self, start: Pos) -> Self {
        self.cells
            .iter_mut()
            .flatten()
            .for_each(|cell| cell.dist = None);
        let mut next = vec![(start.x, start.y)];
        self.at_pos_mut(start).dist = Some(0);
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

    pub fn calc_longest(&mut self) -> (Pos, Pos) {
        let mut start = Self::all_pos()
            .find(|pos| !self.at_pos(*pos).masked)
            .expect("Should be at lease one valid position");
        for pass in 0..3 {
            // Calculate distances from max
            *self = self.calc_dist(start);
            // Find cell farthest from max
            start = self.pos_of(self.all_cells().fold(self.at_pos(start), |max, cell| {
                if cell.dist > max.dist { cell } else { max }
            }));
            match pass {
                0 => {}
                1 => self.start = start,
                2 => self.end = start,
                _ => panic!("pass shouldn't get this large"),
            }
        }
        println!("Result: {}, {} ", self.start, self.end);
        (self.start, self.end)
    }

    pub fn clear_path(mut self) -> Self {
        self.cells.iter_mut().flatten().for_each(|cell| {
            cell.dist = None;
            cell.path = None;
        });
        self
    }

    pub fn all_cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flatten()
    }

    pub fn all_cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().flatten()
    }

    pub fn clear(mut self) -> Self {
        self.start = Pos::new(0, 0);
        self.end = Pos::new(S - 1, S - 1);
        for cell in self.all_cells_mut() {
            cell.up = false;
            cell.right = false;
            cell.path = None;
            cell.dist = None;
        }
        self
    }
}
