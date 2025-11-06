#![allow(dead_code)]

use crate::maze;
use image::*;
use maze::Direction;
use maze::Maze;
use maze::Pos;

const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
const T_GRAY: Rgba<u8> = Rgba([128, 128, 128, 128]);

pub fn make_image<const S: usize>(maze: &Maze<S>) -> RgbaImage {
    let tee = ImageReader::open("images/tee.png")
        .unwrap()
        .decode()
        .unwrap();
    let tip = ImageReader::open("images/tip.png")
        .unwrap()
        .decode()
        .unwrap();
    let corner = ImageReader::open("images/corner.png")
        .unwrap()
        .decode()
        .unwrap();
    let line = ImageReader::open("images/line.png")
        .unwrap()
        .decode()
        .unwrap();
    let line_boarder = ImageReader::open("images/line_boarder.png")
        .unwrap()
        .decode()
        .unwrap();
    let four_way = ImageReader::open("images/four_way.png")
        .unwrap()
        .decode()
        .unwrap();
    let line_vert = imageops::rotate90(&line);
    let line_boarder_vert = imageops::rotate90(&line_boarder);
    const SCALE: u32 = 50;
    const BOARDER: u32 = 5;
    const CELL: u32 = SCALE - BOARDER;
    let mut image = RgbaImage::new(SCALE * S as u32 + BOARDER, SCALE * S as u32 + BOARDER);
    for (_, _, pix) in image.enumerate_pixels_mut() {
        *pix = Rgba([255, 255, 255, 255]);
    }
    // Intersections
    for pos in Maze::<S>::all_pos().filter(|pos| pos.x != S && pos.y != S) {
        let cell_root = Pos::new(
            pos.x * SCALE as usize + BOARDER as usize,
            pos.y * SCALE as usize + BOARDER as usize,
        );
        let north = pos
            .shift(Direction::North)
            .and_then(|pos| maze.at_pos_opt(pos))
            .map(|cell| cell.right)
            .unwrap_or(false);
        let east = pos
            .shift(Direction::East)
            .and_then(|pos| maze.at_pos_opt(pos))
            .map(|cell| cell.up)
            .unwrap_or(false);
        let south = maze.at_pos(pos).right;
        let west = maze.at_pos(pos).up;

        let this_image = match (north, east, south, west) {
            // None
            (true, true, true, true) => continue,
            // One
            (false, true, true, true) => &tip,
            (true, false, true, true) => &tip.rotate270(),
            (true, true, false, true) => &tip.rotate180(),
            (true, true, true, false) => &tip.rotate90(),
            // Line
            (true, false, true, false) => &line,
            (false, true, false, true) => &line.rotate90(),
            // Corner
            (true, true, false, false) => &corner,
            (false, true, true, false) => &corner.rotate270(),
            (false, false, true, true) => &corner.rotate180(),
            (true, false, false, true) => &corner.rotate90(),
            // Tee
            (true, false, false, false) => &tee,
            (false, true, false, false) => &tee.rotate270(),
            (false, false, true, false) => &tee.rotate180(),
            (false, false, false, true) => &tee.rotate90(),
            (false, false, false, false) => &four_way,
        };

        image::imageops::overlay(
            &mut image,
            this_image,
            cell_root.x as i64 + CELL as i64,
            cell_root.y as i64 + CELL as i64,
        );
    }

    for pos in Maze::<S>::all_pos() {
        let cell_root = Pos::new(
            pos.x * SCALE as usize + BOARDER as usize,
            pos.y * SCALE as usize + BOARDER as usize,
        );
        let cell = maze.at_pos(pos);
        if !cell.right
            && (!maze
                .at_pos_opt(pos.shift(Direction::East).unwrap())
                .map(|c| c.masked)
                .unwrap_or_default()
                || !cell.masked)
        {
            image::imageops::overlay(
                &mut image,
                &line_vert,
                cell_root.x as i64 + CELL as i64,
                cell_root.y as i64,
            );
        }
        if !cell.up
            && (!maze
                .at_pos_opt(pos.shift(Direction::North).unwrap())
                .map(|c| c.masked)
                .unwrap_or_default()
                || !cell.masked)
        {
            image::imageops::overlay(
                &mut image,
                &line,
                cell_root.x as i64,
                cell_root.y as i64 + CELL as i64,
            );
        }
        if pos == maze.start {
            for x in 0..CELL {
                for y in 0..CELL {
                    image.put_pixel(cell_root.x as u32 + x, cell_root.y as u32 + y, GREEN);
                }
            }
        }
        if pos == maze.end {
            for x in 0..CELL {
                for y in 0..CELL {
                    image.put_pixel(cell_root.x as u32 + x, cell_root.y as u32 + y, RED);
                }
            }
        }
    }
    // BORDER
    for pos in Maze::<S>::all_pos().filter(|pos| pos.x == 0 || pos.x == S - 1) {
        let cell_root = Pos::new(pos.x * SCALE as usize, pos.y * SCALE as usize);
        if pos.x == 0 {
            image::imageops::overlay(
                &mut image,
                &line_boarder_vert,
                cell_root.x as i64,
                cell_root.y as i64,
            );
        } else {
            image::imageops::overlay(
                &mut image,
                &line_boarder_vert,
                cell_root.x as i64 + SCALE as i64,
                cell_root.y as i64,
            );
        }
    }
    for pos in Maze::<S>::all_pos().filter(|pos| pos.y == 0 || pos.y == S - 1) {
        let cell_root = Pos::new(pos.x * SCALE as usize, pos.y * SCALE as usize);
        if pos.y == 0 {
            image::imageops::overlay(
                &mut image,
                &line_boarder,
                cell_root.x as i64,
                cell_root.y as i64,
            )
        } else {
            image::imageops::overlay(
                &mut image,
                &line_boarder,
                cell_root.x as i64,
                cell_root.y as i64 + SCALE as i64,
            );
        }
    }

    image = imageops::flip_horizontal(&image);
    imageops::rotate180(&image)
}

pub fn print<const S: usize>(maze: &Maze<S>) {
    let has_path = maze.cells.iter().flatten().any(|cell| cell.path.is_some());
    for y in 0..S {
        // Flip so re render from the top down.
        let y = S - 1 - y;
        // Top
        if y == S - 1 {
            print!("╔");
        } else if maze.at(0, y).up {
            print!("║");
        } else {
            print!("╟");
        }
        for x in 0..S {
            let left = maze.at(x, y).up;
            let bottom = maze.at(x, y).right;
            let right = maze.at_opt(x + 1, y).unwrap_or(&maze::CLOSED_CELL).up;
            let top = maze.at_opt(x, y + 1).unwrap_or(&maze::CLOSED_CELL).right;
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
            let dist: Option<u32> = maze.at(x, y).dist;
            let path: Option<bool> = maze.at(x, y).path;
            let dist_char: String = if has_path && path == Some(true) {
                //
                //
                //
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

            if x == maze.end.x && y == maze.end.y && maze.at(x, y).right {
                print!("END ");
            } else if x == maze.end.x && y == maze.end.y && x == S - 1 {
                print!("END║");
            } else if x == maze.end.x && y == maze.end.y {
                print!("END│");
            } else if x == maze.start.x && y == maze.start.y && maze.can_go(x, y, Direction::East) {
                print!("STA ");
            } else if x == maze.start.x && y == maze.start.y && y == S - 1 {
                print!("STA║");
            } else if x == maze.start.x && y == maze.start.y {
                print!("STA│");
            } else if maze.at(x, y).right {
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
        } else if maze.at(x, 0).right {
            print!("════");
        } else {
            print!("═══╧");
        }
    }
    println!();
}
