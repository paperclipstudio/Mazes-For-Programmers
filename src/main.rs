#![allow(dead_code)]
use rand::prelude::*;

#[derive(Copy, Clone, Default)]
struct Cell {
    up: bool,
    right: bool,
}

#[derive(Copy, Clone)]
struct Maze<const S: usize> {
    cells: [[Cell; S]; S],
}

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
    fn at(&self, x: usize, y: usize) -> &Cell {
        //let index = y * S + x;
        // println!("{x} {y} {index}**");
        &self.cells[y][x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    fn print(&self) {
        for y in 0..S {
            // Top
            print!(" ");
            for x in 0..S {
                if self.at(x, S - y - 1).up {
                    print!("    ")
                } else {
                    print!("--- ");
                }
            }

            println!();
            print!("|");
            for x in 0..S {
                if self.at(x, S - y - 1).right {
                    print!("    ");
                } else {
                    print!("   |");
                }
            }
            println!()
        }
        print!(" ");
        for _ in 0..S {
            print!("--- ");
        }
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

                self.set(x, y, Cell { up, right })
            }
        }
        self
    }
}

fn main() {
    let mut maze: Maze<5> = Default::default();
    maze = maze.binary_tree();
    maze.print();
    println!("Hello, world!");
}
