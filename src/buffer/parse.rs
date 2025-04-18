use std::fs;

use super::buffer::Buffer;

pub fn parse(file_name: String) -> Buffer {
    let mut buffer = Buffer::empty_buffer();
    let file_res = fs::read_to_string(&file_name);
    let file_content = match file_res {
        Ok(content) => content,
        Err(_) => {
            println!("Error when opening file: Couldn't find file {}", file_name);
            String::new()
        }
    };

    file_content
        .lines()
        .for_each(|line| buffer.rows.push(String::from(line)));
    buffer
}
