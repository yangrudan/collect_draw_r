use std::collections::{HashMap, BTreeSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Represents a node in the Trie structure for stack traces.
#[derive(Debug, Clone)]
pub struct TrieNode {
    children: HashMap<String, TrieNode>,
    is_end_of_stack: bool,
    ranks: BTreeSet<u32>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_stack: false,
            ranks: BTreeSet::new(),
        }
    }

    fn add_rank(&mut self, rank: u32) {
        self.ranks.insert(rank);
    }
}

/// Represents a Trie structure for merging stack traces.
pub struct StackTrie {
    pub root: TrieNode,
    all_ranks: BTreeSet<u32>,
}

impl StackTrie {
    fn new(all_ranks: Vec<u32>) -> Self {
        let all_ranks_set: BTreeSet<_> = all_ranks.into_iter().collect();
        StackTrie {
            root: TrieNode::new(),
            all_ranks: all_ranks_set,
        }
    }

    fn insert(&mut self, stack: Vec<&str>, rank: u32) {
        // Skip empty stacks
        if stack.is_empty() {
            return;
        }
        
        let mut node = &mut self.root;
        for frame in stack {
            // Skip empty frame names
            if frame.is_empty() {
                continue;
            }
            node = node.children.entry(frame.to_string()).or_insert_with(TrieNode::new);
            node.add_rank(rank);
        }
        node.is_end_of_stack = true;
        node.add_rank(rank);
    }

    fn format_rank_str(&self, ranks: &BTreeSet<u32>) -> String {
        let ranks_vec: Vec<_> = ranks.iter().cloned().collect();
        
        let leak_ranks: Vec<_> = self.all_ranks
            .difference(ranks)
            .cloned()
            .collect();

        fn inner_format(ranks: &[u32]) -> String {
            let mut str_buf = String::new();
            let mut low = 0;
            let mut high = 0;
            if ranks.is_empty() {
                return str_buf;
            }
            while high < ranks.len() - 1 {
                let low_value = ranks[low];
                let mut high_value = ranks[high];
                while high < ranks.len() - 1 && high_value + 1 == ranks[high + 1] {
                    high += 1;
                    high_value = ranks[high];
                }
                low = high + 1;
                high += 1;
                if low_value != high_value {
                    str_buf.push_str(&format!("{}-{}", low_value, high_value));
                } else {
                    str_buf.push_str(&low_value.to_string());
                }
                if high < ranks.len() {
                    str_buf.push('/');
                }
            }
            if high == ranks.len() - 1 {
                str_buf.push_str(&ranks[high].to_string());
            }
            str_buf
        }

        let has_stack_ranks = inner_format(&ranks_vec);
        let leak_stack_ranks = inner_format(&leak_ranks);
        format!("@{}|{}", has_stack_ranks, leak_stack_ranks)
    }

    pub fn traverse_with_all_stack<'a>(&'a self, node: &'a TrieNode, path: Vec<&str>) -> Vec<(Vec<String>, String)> {
        let mut result = Vec::new();
        for (frame, child) in &node.children {
            let rank_str = self.format_rank_str(&child.ranks);
            if child.is_end_of_stack {
                let path_str = path.join(";");
                result.push((vec![path_str, frame.to_string()], rank_str.clone()));
            }
            let mut child_path = path.clone();
            let frame_rank = format!("{}{}", frame, rank_str);
            child_path.push(&frame_rank[..]);
            // child_path.push(rank_str.as_str());
            result.extend(self.traverse_with_all_stack(child, child_path));
        }
        result
    }
}

/// Merges multiple stack traces into a single StackTrie.
pub fn merge_stacks(stacks: Vec<&str>) -> StackTrie {
    let all_ranks: Vec<u32> = (0..stacks.len() as u32).collect();
    let mut trie = StackTrie::new(all_ranks);
    for (rank, stack) in stacks.iter().enumerate() {
        let stack_frames: Vec<&str> = stack.split(';').collect();
        trie.insert(stack_frames, rank as u32);
    }
    trie
}

#[allow(dead_code)]
fn read_file_to_list(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stack_handling() {
        let stacks = vec!["", "main;func1", ""];
        let trie = merge_stacks(stacks);
        // Empty stacks should be skipped, so we should only have 1 stack
        let results = trie.traverse_with_all_stack(&trie.root, Vec::new());
        assert!(!results.is_empty(), "Should have at least one result for non-empty stack");
    }

    #[test]
    fn test_empty_frame_handling() {
        let stacks = vec!["main;;func1", "main;func2"];
        let trie = merge_stacks(stacks);
        let results = trie.traverse_with_all_stack(&trie.root, Vec::new());
        // Should handle empty frames gracefully
        assert!(!results.is_empty(), "Should process stacks despite empty frames");
    }

    #[test]
    fn test_merge_stacks_basic() {
        let stacks = vec![
            "main;func1;func2",
            "main;func1;func3",
        ];
        let trie = merge_stacks(stacks);
        let results = trie.traverse_with_all_stack(&trie.root, Vec::new());
        assert_eq!(results.len(), 2, "Should have 2 distinct paths");
    }
}

    //////////////////////////////////////////////////////////////////////////

    // let stacks = vec![
    //     "main;func1;func2;func3",
    //     "main;func1;func2;func4",
    //     "main;func1;func3;func5",
    //     "main;func1;func3;func6",
    // ];

    // let trie = merge_stacks(stacks);

    // let mut output = File::create("./output/merged_stacks.txt")?;
    // for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
    //     writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
    // }

    ////////////////////////////////////////////////////////////////////////////////