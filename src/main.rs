mod modules;
mod app;

use app::MteToolBox;
use eframe::{run_native, NativeOptions};

fn main() {
    //APP LAUCHING IS HERE
    let win_option = NativeOptions::default(); //DEFINE OPTIONS FOR THE APP
    run_native(
        "MteToolBox",
        win_option,
        Box::new(|cc| Ok(Box::new(MteToolBox::new(cc)))),
    )
    .unwrap(); //NEED TO MANAGE THIS ERROR BETTER
}
