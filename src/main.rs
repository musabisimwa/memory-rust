// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

use slint::Model;
use rand::seq::SliceRandom;

slint::include_modules!();
fn main() {
    let main_window = MainWindow::new().unwrap();

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_is_pair_solved(
        move ||{
            let mut flipped_tiles = tiles_model.iter().enumerate()
            .filter(|(_,tile)| tile.is_visible && !tile.is_solved);
            
            if let(Some((t1_idx,mut t1)), Some((t2_idx,mut t2)))= 
            (flipped_tiles.next(),flipped_tiles.next()){
                let _is_pair_solved =t1 ==t2;
                if _is_pair_solved{
                    t1.is_solved =true;
                    tiles_model.set_row_data(t1_idx,t1);
                    t2.is_solved = true;
                    tiles_model.set_row_data(t2_idx, t2);
                }else{
                    let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.is_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.is_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                    });
                }
            }
        }
    );
    main_window.run().unwrap();
}
