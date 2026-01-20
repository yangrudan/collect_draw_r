use std::fs::{File, create_dir_all};
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
    // Ensure output directory exists
    create_dir_all("./output")?;

    let mut file = File::open("./output/urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    // Async fetch step stays in the async runtime and is awaited
    fetch_and_save_urls(urls).await?;

    // Process call stacks (blocking work) in a dedicated blocking thread
    let input_path = "./output/output.json".to_string();
    let output_path = "./output/processed_stacks.txt".to_string();
    let output_path_clone = output_path.clone();

    tokio::task::spawn_blocking(move || {
        process_callstacks(&input_path, &output_path_clone)
    }).await??;

    println!("Processed call stacks have been written to {}", output_path);

    // Merge stacks, write merged file, and draw flamegraph all in a blocking task
    tokio::task::spawn_blocking(|| {
        // Stream process the file to build the trie without loading everything into memory in the async runtime
        let file = File::open("./output/processed_stacks.txt")?;
        let reader = BufReader::new(file);

        // Collect stacks into a vector, but process line by line
        let mut stacks = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                stacks.push(line);
            }
        }

        // Convert to string references for processing
        let stack_refs: Vec<&str> = stacks.iter().map(|s| s.as_str()).collect();
        let trie = merge_stacks(stack_refs);

        let mut output = File::create("./output/merged_stacks_4ranks.txt")?;
        for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
            writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
        }

        // Drawing the frame graph is also blocking; keep it in the blocking task
        draw_frame_graph("./output/merged_stacks_4ranks.txt");

        Ok::<(), std::io::Error>(())
    }).await??;

    Ok(())

}