use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Read the exif data from a file and return a hashmap of the tags and values using kamadak-exif
pub fn get_exif<P: AsRef<Path>>(path: &P) -> Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let exif = exif::Reader::new().read_from_container(&mut reader)?;
    let mut exif_data = HashMap::new();
    for field in exif.fields() {
        let tag = field.tag;
        let value = field.display_value();
        exif_data.insert(format!("{:?}", tag), value.to_string());
    }
    Ok(exif_data)
}
