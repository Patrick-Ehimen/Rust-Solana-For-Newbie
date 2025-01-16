use std::fs::File;
use std::io::Write;

pub fn store_data(data: &[String], format: &str, output_file_name: &str) {
    match format {
        "json" => {
            let json_data = serde_json::to_string(data).expect("Unable to serialize data to JSON");
            let mut file = File::create(output_file_name).expect("Unable to create file");
            file.write_all(json_data.as_bytes())
                .expect("Unable to write data to file");
        }
        "csv" => {
            let mut file = File::create(output_file_name).expect("Unable to create file");
            for row in data {
                file.write_all(row.as_bytes())
                    .expect("Unable to write data to file");
                file.write_all(b"\n").expect("Unable to write data to file");
            }
        }
        _ => panic!("Unsupported format"),
    }
}
