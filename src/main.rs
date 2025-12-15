mod maze;
mod render;
use image::*;
use maze::Maze;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let image_path = if args.len() == 2 {
        args.get(1).unwrap()
    } else {
        "./images/deadends.png"
    };

    let tee = ImageReader::open(image_path)
        .unwrap()
        .decode()
        .unwrap()
        .rotate180()
        .fliph();
    let mut maze = Maze::<50>::default();
    for x in 0..maze.self_size().min(tee.width() as usize) {
        for y in 0..maze.self_size().min(tee.height() as usize) {
            let luma = tee.grayscale().get_pixel(x as u32, y as u32).to_luma();
            if luma.0[0] < 128 {
                maze.at_mut(x, y).masked = true;
            }
        }
    }
    maze.all_cells_mut().for_each(|x| x.masked=false);
    maze = maze.hunt_and_kill();

    //maze = maze.walker();
    //maze = maze.hunt_and_kill(Some());
    let rng = ChaCha8Rng::seed_from_u64(1236);
    maze = maze.hunt_and_kill_seed(rng);
    let (pos1, _) = maze.calc_longest();

    maze = maze.calc_dist(pos1);
    maze = maze.shortist_path();
    //maze = maze.clear_path();
    maze.print();
    let image = render::make_image(&maze);
    image.save("output.png").unwrap();

    //let mut bytes: Vec<u8> = Vec::new();
    //image
    //   .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
    //  .unwrap();
}
