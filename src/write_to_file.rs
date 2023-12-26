use super::struct_mod::{ResponseResult, RqStruct};

use std::fs::File;
use std::io::Write;
use serde_json::to_writer_pretty;
use std::io::BufWriter;

pub fn _write_to_txt(results: Vec<ResponseResult>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("results.txt")?;

    // Add headers
    writeln!(file, "{:<20} {:<10} {}", "Description", "Success", "Message")?;

    // Add data
    for result in results {
        writeln!(file, "{:<20} {:<10} {}", result.desc, result.success, result.msg)?;
    }

    Ok(())
}



pub fn write_to_json(results: Vec<RqStruct>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("results.json")?;
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, &results)?;
    Ok(())
}