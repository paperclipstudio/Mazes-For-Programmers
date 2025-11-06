mod maze;
mod render;
use maze::Maze;
use std::io::Cursor;

fn main() {
    let mut maze = Maze::<10>::default();
    for x in 0..5 {
        for y in 0..5 {
            maze.at_mut(x, y).masked = true;
        }
    }

    maze = maze.walker();
    maze.all_cells_mut().for_each(|cell| {
        cell.up = true;
        cell.right = true;
    });
    let (pos1, _) = maze.calc_longest();
    maze = maze.calc_dist(pos1);
    maze = maze.shortist_path();
    maze = maze.clear_path();
    maze.print();
    let image = render::make_image(&maze);
    image.save("output.png").unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
}
