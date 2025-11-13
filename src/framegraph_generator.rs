use std::fs::{File, create_dir_all};
use std::io::BufReader;
use inferno::flamegraph::{self, Options, Palette};

/// Generates a flamegraph from a stack trace file and saves it as an SVG file.
pub fn draw_frame_graph(file_path: &str) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut options = Options::default();
    options.colors = Palette::Multi(flamegraph::color::MultiPalette::Java);

    // Ensure output directory exists
    create_dir_all("./output").expect("Failed to create output directory");

    let mut output_file = File::create("./output/flamegraph222.svg").expect("Failed to create SVG file");
    flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

    println!("Flamegraph generated and saved as flamegraph.svg");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_draw_frame_graph() {
        // draw_frame_graph("/home/yang/worksapce/collect_stack_draw/debug/debug_4stacks.txt");
        draw_frame_graph("./output/stacks.txt");
        assert!(fs::metadata("./output/flamegraph222.svg").is_ok(), "SVG file should exist");
    }
}
