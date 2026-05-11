use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Define the input and output file names
    let input_path = "flashcards.csv";
    let output_path = "flashcards.md";

    // Initialize the CSV reader. 
    // We configure it to `has_headers(false)` because your data starts on the first line.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(input_path)?;

    // Create the output Markdown file
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // Iterate through each row in the CSV
    for result in reader.records() {
        let record = result?;
        
        // Ensure the row has at least 2 columns
        if record.len() >= 2 {
            let front = &record[0];
            let back = &record[1];
            
            // Write to the markdown file in the requested format
            // The extra `\n` at the end adds a blank line between flashcards for readability
            writeln!(writer, "{} #fc\n{}\n", front, back)?;
        }
    }

    println!("Successfully converted '{}' to '{}'!", input_path, output_path);

    Ok(())
}
