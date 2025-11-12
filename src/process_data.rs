use serde::{Deserialize, Serialize};
use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::Path;

/// Represents a frame in the call stack, which can be either a C frame or a Python frame.
#[derive(Debug, Deserialize, Serialize, Clone)]
enum Frame {
    CFrame(CFrame),
    PyFrame(PyFrame),
}

/// Represents a C frame in the call stack.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct CFrame {
    file: String,
    func: String,
    ip: String,
    lineno: u32,
}

/// Represents a Python frame in the call stack.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct PyFrame {
    file: String,
    func: String,
    lineno: u32,
    locals: serde_json::Value,
}

/// Process call stacks from a JSON file and write the processed stacks to a text file.
pub fn process_callstacks(input_path: &str, output_path: &str) -> io::Result<()> {
    // Read and parse JSON file
    let mut file = File::open(input_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open input file '{}': {}", input_path, e)))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read input file '{}': {}", input_path, e)))?;

    // Parse JSON data with better error message
    let frames: Vec<Vec<Frame>> = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(
            io::ErrorKind::InvalidData, 
            format!("Failed to parse JSON from '{}': {}. Expected array of frame arrays.", input_path, e)
        ))?;

    // Process call stacks
    let mut out_stacks = Vec::new();
    for trace in frames.iter() {
        let mut local_stack = Vec::new();
        for frame in trace {
            match frame {
                Frame::CFrame(_cframe) => {
                    // CFrame processing (currently no-op)
                }
                Frame::PyFrame(_pyframe) => {
                    // PyFrame processing (currently no-op)
                }
            }
            local_stack.push(frame.clone());
        }
        local_stack.reverse();
        out_stacks.push(local_stack);
    }

    // Prepare output data
    let mut prepare_stacks = Vec::new();
    for rank in out_stacks {
        if !rank.is_empty() {
            let data = rank
                .iter()
                .map(|entry| match entry {
                    Frame::CFrame(frame) => format!("{} ({}:{})", frame.func, frame.file, frame.lineno),
                    Frame::PyFrame(frame) => format!("{} ({}:{})", frame.func, frame.file, frame.lineno),
                })
                .collect::<Vec<String>>()
                .join(";");
            prepare_stacks.push(data);
        }
    }

    // Ensure output directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        create_dir_all(parent)
            .map_err(|e| io::Error::new(e.kind(), format!("Failed to create output directory: {}", e)))?;
    }

    // Write stack data to output file
    let mut output_file = File::create(output_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to create output file '{}': {}", output_path, e)))?;
    for stack in prepare_stacks {
        writeln!(output_file, "{}", stack)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn test_process_callstacks() {
        let input_path = "./output/output.json"; 
        let output_path = "./output/processed_stacks.txt";
        let result = process_callstacks(input_path, output_path);
        assert!(result.is_ok(), "Processing call stacks should succeed");
        assert!(std::fs::metadata(output_path).is_ok(), "Output file should exist");
        let output_content = std::fs::read_to_string(output_path).expect("Failed to read output file");
        assert!(!output_content.is_empty(), "Output file should not be empty"); 
    }

    #[test]
    fn test_process_callstacks_invalid_json() {
        use std::io::Write;
        
        // Create a temporary invalid JSON file
        let input_path = "./output/test_invalid.json";
        let output_path = "./output/test_invalid_output.txt";
        
        let mut file = File::create(input_path).expect("Failed to create test file");
        file.write_all(b"{invalid json}").expect("Failed to write test data");
        
        let result = process_callstacks(input_path, output_path);
        assert!(result.is_err(), "Should fail with invalid JSON");
        
        // Check error message contains useful information
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(error_msg.contains("Failed to parse JSON"), 
                "Error message should indicate JSON parsing failure: {}", error_msg);
        }
        
        // Cleanup
        let _ = std::fs::remove_file(input_path);
    }

    #[test]
    fn test_process_callstacks_missing_file() {
        let input_path = "./output/nonexistent_file.json";
        let output_path = "./output/test_output.txt";
        
        let result = process_callstacks(input_path, output_path);
        assert!(result.is_err(), "Should fail with missing file");
        
        // Check error message contains useful information
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(error_msg.contains("Failed to open input file"), 
                "Error message should indicate file opening failure: {}", error_msg);
        }
    }
}
