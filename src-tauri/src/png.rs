use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use anyhow::Ok;

pub fn read_png<P: AsRef<Path>>(path: &P) -> Result<HashMap<String, String>> {
    // Opening a png file that has a zTXt chunk
    let decoder = png::Decoder::new(File::open(path)?);
    let reader = decoder.read_info()?;
    let mut result = HashMap::new();
    // If the text chunk is before the image data frames, `reader.info()` already contains the text.
    for text_chunk in &reader.info().uncompressed_latin1_text {
        println!("Found value for {:?}", text_chunk.keyword); // Prints the keyword
                                                              // println!("{:#?}", text_chunk); // Prints out the text chunk.
                                                              // To get the uncompressed text, use the `get_text` method.
        result.insert(text_chunk.keyword.clone(), text_chunk.text.clone());
    }
    Ok(result)
}
