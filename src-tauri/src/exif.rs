use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Read the exif data from a file and return a hashmap of the tags and values
// Returns an error if the file doesn't exist or if the exif data is invalid
pub fn get_exif<P: AsRef<Path>>(path: &P) -> Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    let exif_reader = Reader::new().read_from_container(&mut reader)?;
    let mut exif_data = HashMap::new();

    for field in exif_reader.fields() {
        let tag = field.tag.to_string();
        let value = match &field.value {
            Value::Ascii(ref vec) => vec
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            _ => field.value.display_as(field.tag).to_string(),
        };
        exif_data.insert(tag, value);
    }

    Ok(exif_data)
}
