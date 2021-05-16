extern crate getopts;
extern crate image;

use getopts::Options;

use std::env;
use std::fs;
use std::path::Path;

use minibrowser_rs::{*};

fn main() {
    // Parse command-line options:
    let args: Vec<String> = env::args().skip(1).collect();
    let mut opts = Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => std::panic::panic_any(f.to_string())
    };

    // Read input files:
    let read_source = |arg_filename: Option<String>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => filename,
            None => default_filename,
        };
        fs::read_to_string(Path::new(path)).unwrap()
    };
    let html = read_source(matches.opt_str("h"), "examples/test.html");
    let css  = read_source(matches.opt_str("c"), "examples/test.css");

    // Since we don't have an actual window, hard-code the "viewport" size.
    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    // Parsing and rendering:
    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, initial_containing_block);
    let canvas = painting::paint(&layout_root, initial_containing_block.content);

    // Create the output file:
    let filename = matches.opt_str("o").unwrap_or("output.png".to_string());

    // Save an image:
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, Box::new(|x: u32, y: u32| buffer[(y * w + x) as usize]));

    // let result = ImageRgba8(img).save(file, image::PNG);
    let result = img.save(Path::new(&filename));
    match result {
        Ok(_) => println!("Saved output as {}", filename),
        Err(_) => println!("Error saving output as {}", filename)
    }

    // Debug output:
    println!("{:?}", layout_root.dimensions);
}