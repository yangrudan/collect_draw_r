use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::io::Write;

mod stack_collector;
mod framegraph_generator;
mod stack_merger;
mod process_data;

use stack_collector::fetch_and_save_urls;
use framegraph_generator::draw_frame_graph;

use stack_merger::merge_stacks;
use process_data::process_callstacks;

/**
 # Steps Description
 
 - collect call stacks from URLs and save them to a json file(output.json)
 - process the call stacks and save them to a text file(processed_stacks.txt)
 - merge the call stacks and save them to a text file(merged_stacks_4ranks.txt)
 - draw the frame graph from the merged call stacks(frame_graph.png)
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./output/urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;


    let input_path = "./output/output.json"; 
    let output_path = "./output/processed_stacks.txt";
    process_callstacks(input_path, output_path)?;

    println!("Processed call stacks have been written to {}", output_path);


    let file = File::open("./output/processed_stacks.txt")?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n'); // 保留换行符
    }

    let stacks: Vec<&str> = content.lines().collect();
    let trie = merge_stacks(stacks);

    let mut output = File::create("./output/merged_stacks_4ranks.txt")?;
    for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
        writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
    }


    draw_frame_graph("./output/merged_stacks_4ranks.txt");

    Ok(())

}