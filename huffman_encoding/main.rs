extern crate common;
use common::measure_and_print;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;

// Type of the reference to the node
type Link = Option<Box<Node>>;

// Huffman tree node struct
#[derive(Debug,Eq)]
struct Node {
    freq: u32,
    letter: char,
    left: Link,
    right: Link,
}

impl Ord for Node {
    // reverse order to make Min-Heap
    fn cmp(&self, b: &Self) -> Ordering {
        b.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

impl PartialEq for Node {
    fn eq(&self, b: &Self) -> bool {
        self.freq == b.freq
    }
}

impl Node {
    // create leaf-node with particular letter
    fn new_leaf(freq: u32, letter: char) -> Self {
        Node { freq: freq, letter: letter, left: None, right: None }
    }

    // create new node on top of `left` and `right`
    fn combine(left: Self, right: Self) -> Self {
        Node { freq: left.freq + right.freq, letter: ' ',  left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }

    // traverse tree building letter->code `map`
    fn build_map(&self, map: &mut HashMap<char, String>, prefix: String) {
        match self.right {
            Some(ref leaf) => leaf.build_map(map, prefix.clone() + "1"),
            _ => { },
        }
        match self.left {
            Some(ref leaf) => { leaf.build_map(map, prefix + "0"); },
            _ => { map.insert(self.letter, prefix); },
        }
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    // 1.Read message
    let sentence = read_line();

    let mut letter_freq = HashMap::<char, u32>::new();
    let mut code_string = String::new();
    let mut table = HashMap::<char, String>::new();

    measure_and_print(||
        {
            // 2. Calculate frequancy map for letters
            for c in sentence.chars() {
                let freq = letter_freq.entry(c).or_insert(0);
                *freq += 1;
            }

            // 3. Build Huffman tree & table
            let mut heap = BinaryHeap::<Node>::new();
            for (c, count) in &letter_freq {
                heap.push(Node::new_leaf(*count, *c));
            }
            if heap.len() > 1 {
                while heap.len() > 1 {
                    // take two least frequent nodes and build new one on top of it
                    let a = heap.pop().unwrap();
                    let b = heap.pop().unwrap();
                    heap.push(Node::combine(a, b));
                }
                // take head of the tree and traverse it build letter->code map
                heap.peek().unwrap().build_map(&mut table, String::from(""));
            }
            else if heap.len() == 1 {
                heap.peek().unwrap().build_map(&mut table, String::from("0"));
            }

            // 4. Encode message
            for c in sentence.chars() {
                code_string.push_str(table.get(&c).unwrap());
            }
        });

    // 5. Output
    println!("{} {}", letter_freq.len(), code_string.len());
    for (letter, code) in &table {
        println!("{}: {}", letter, code);
    }
    println!("{}", code_string);
}
