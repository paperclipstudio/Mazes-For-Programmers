mod maze;
mod render;
use maze::Maze;
use std::io::Cursor;

fn main() {
    let mut maze = Maze::<50>::default().walker();
    let (pos1, _) = maze.calc_longest();
    maze = maze.calc_dist(pos1);
    maze = maze.shortist_path();
    maze = maze.clear_path();
    maze.print();
    render::print(&maze);
    let image = render::make_image(&maze);
    image.save("output.png").unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
}
