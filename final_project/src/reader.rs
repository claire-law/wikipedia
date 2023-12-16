use std::error::Error;

pub fn read_file(path: &str, header: bool) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut result: Vec<(String, String)> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(header)
        .flexible(true)
        .from_path(path)?;

    for i in rdr.records() {
        let record = i.unwrap();
        
        //skip comments at top of file
        if &record[0].starts_with("#") == &true {continue;}
    
        let x = record[0].to_string();
        let y = record[1].to_string();
        result.push((x, y));
    }
    Ok(result)
}
