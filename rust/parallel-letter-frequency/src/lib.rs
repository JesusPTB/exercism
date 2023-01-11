use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::str;

pub fn split_input(input: String, number: usize) -> Vec<Vec<u8>> {
    let split_len = input.len() / number;
    let mut split: Vec<Vec<u8>> = input.as_bytes().chunks(input.len()/number)
                                              .map(|x| x.to_vec()).collect();
    if split.last().unwrap().len() != input.len()/number {
        let mut drop = split.pop().unwrap();
        split.last_mut().unwrap().append(&mut drop);
    }
    split
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // unimplemented!(
    //     "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
    //     input,
    //     match worker_count {
    //         1 => "1 worker".to_string(),
    //         _ => format!("{} workers", worker_count),
    //     }
    // );
    // let counter = Arc::new(Mutex::new(HashMap { base: () }));
    let mut handles = vec![];
    let input_joined = input.join("");
    let split = split_input(input_joined, worker_count);
    
    for i in 0..worker_count {
        let thread_input = split[i].clone();
        // let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();
            let string = str::from_utf8(&thread_input).unwrap();
            println!("{:?}",  string);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    let sliced_input: HashMap<char, usize> = Default::default();
    sliced_input
}
