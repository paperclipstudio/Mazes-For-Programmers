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

static CLOSED_CELL: Cell = Cell {
    up: false,
    right: false,
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

    fn print(&self) {
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
                if x == 0 && y == S - 1 && self.at(x, S - y - 1).right {
                    print!("END ");
                } else if x == 0 && y == S - 1 {
                    print!("END│");
                } else if x == S - 1 && y == 0 {
                    print!("STA║");
                } else if self.at(x, S - y - 1).right {
                    print!("    ");
                } else if x == S - 1 {
                    print!("   ║");
                } else {
                    print!("   │");
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

                self.set(x, y, Cell { up, right })
            }
        }
        self
    }
    fn random(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(
                    x,
                    y,
                    Cell {
                        right: rng.random(),
                        up: rng.random(),
                    },
                );
            }
        }
        self
    }

    fn rights(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(
                    x,
                    y,
                    Cell {
                        right: true,
                        up: rng.random(),
                    },
                );
            }
        }
        self
    }

    fn ups(mut self) -> Self {
        let mut rng = rand::rng();
        for y in 0..S {
            for x in 0..S {
                self.set(
                    x,
                    y,
                    Cell {
                        right: rng.random(),
                        up: true,
                    },
                );
            }
        }
        self
    }
}

fn main() {
    Maze::<10>::default().random().sidewinder().print();
}
