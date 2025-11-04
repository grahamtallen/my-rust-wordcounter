use std::fs;

fn main() {
    let files = ["file1.txt", "file2.txt", "file3.txt"];

    // spawn a thread per file;
    let mut handles = vec![];
    for file in files {
        let handle = std::thread::spawn(move || {
            let file = fs::read_to_string(file);
            match file {
                Ok(file_text) => file_text.split_whitespace().count(),
                Err(e) => {
                    println!("Error reading file: {}", e);
                    0
                }
            }
        });
        handles.push(handle);
    }
    let mut total: usize = 0;
    // in each thread
    for handle in handles {
        total += handle.join().unwrap();
    }
    dbg!(total);
}
