mod maze;
mod render;
use image::*;
use maze::Maze;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::maze::Pos;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let image_path = if args.len() == 2 {
        args.get(1).unwrap()
    } else {
//        "./images/deadends.png"
"./images/small_blockout.png"
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
            // let pos = Pos::new(x,y);
            // if x < maze.self_size()/2 {
            //     maze.at_pos_mut(pos).masked = true;
            // }
        }
    }
    let mut max = 0;
    let mut max_i = 0;
    for i in 0..500{
    let mut rng = ChaCha8Rng::seed_from_u64(i);
        let mut tmp = maze.hunt_and_kill_seed(&mut rng);
        let start = tmp.calc_longest();
        tmp = tmp.calc_dist(start.0);
        let tmp_max = tmp.all_cells().filter_map(|x|x.dist).max().unwrap();
        if tmp_max > max {
            max_i = i;
            max = tmp_max;
            println!("Max {max} at {max_i}");
        }
    }

    let mut rng = ChaCha8Rng::seed_from_u64(max_i);
    maze = maze.hunt_and_kill_seed(&mut rng);
    let (start, end) = maze.calc_longest();
maze.all_cells_mut().for_each(|x|x.masked ^= true);
    // let rng = ChaCha8Rng::seed_from_u64(12345);
 maze = maze.hunt_and_kill_seed(&mut rng);
//maze.all_cells_mut().for_each(|x|x.masked ^= true);
    // maze.all_cells_mut().for_each(|x|x.masked ^= true);
    // //maze = maze.walker();
    //maze = maze.hunt_and_kill(Some());
    // maze.all_cells_mut().for_each(|x|x.masked ^= true);

    maze = maze.calc_dist(start);
    maze.end = end;
    maze = maze.shortist_path();
    maze = maze.clear_path();
maze.all_cells_mut().for_each(|x|x.masked = false);
    maze.print();
    let image = render::make_image(&maze);
    image.save("output.png").unwrap();

    //let mut bytes: Vec<u8> = Vec::new();
    //image
    //   .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
    //  .unwrap();
}
