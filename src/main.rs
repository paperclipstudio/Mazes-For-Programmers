mod maze;
mod render;
use maze::Maze;
use std::io::Cursor;

fn main() {
    let mut maze = Maze::<10>::default().walker();
    let (pos1, pos2) = maze.calc_longest();
    println!("{pos1} -> {pos2}");

    maze = maze.calc_dist(pos1);
    maze = maze.shortist_path();
    maze = maze.clear_path();
    render::print(&maze);
    let image = render::make_image(&maze);
    image.save("empty.png").unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
}
