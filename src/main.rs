extern crate cairo;

use std::fs::File;
use std::env;
use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};

fn main() {
    let args: Vec<String> = env::args().collect();
    let msg = &args[1];
    let target = &args[2];

    let width = 3840;
    let height = 2160;

    let surface =
        ImageSurface::create(Format::ARgb32, width, height).expect("couldn't create a surface");
    let context = Context::new(&surface);

    // background
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint();

    // text
    context.select_font_face("Fira Code", FontSlant::Normal, FontWeight::Normal);
    context.set_font_size(300.0);
    context.move_to(100.0, 1000.0);
    context.set_source_rgb(255.0, 255.0, 255.0);
    context.show_text(msg);

    // write output file
    let mut file = File::create(target).expect("Couldn’t create file.");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to png");
}
