use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use csv::ReaderBuilder;

fn codes(path: PathBuf) -> String {
    let mut code = "
        const CODES: Vec<PostalCode> = vec![
    ".into();

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)
        .expect("could not load csv data");

    for item in reader.records() {
        let data = item.expect("could not parse csv record");

        let country_code = data.get(0).unwrap();
        let postal_code = data.get(1).unwrap();
        let place_name = data.get(2).unwrap();
        let admin_name1 = data.get(3).unwrap();
        let admin_code1 = data.get(4).unwrap();
        let admin_name2 = data.get(5).unwrap();
        let admin_code2 = data.get(6).unwrap();
        let admin_name3 = data.get(7).unwrap();
        let admin_code3 = data.get(8).unwrap();
        let latitude = data.get(9).unwrap();
        let longitude = data.get(10).unwrap();

        let accuracy = match data.get(8) {
            Some(v) => format!("Some({})",v),
            None => "None".into()
        };

        code += format!("PostalCode {{
            country_code: {},
            postal_code: \"{}\",
            place_name: \"{}\",
            admin_name1: \"{}\",
            admin_code1: \"{}\",
            admin_name2: \"{}\",
            admin_code2: \"{}\",
            admin_name3: \"{}\",
            admin_code3: \"{}\",
            latitude: {},
            longitude: {},
            accuracy: {},
        }},",
            country_code,
            postal_code,
            place_name,
            admin_name1,
            admin_code1,
            admin_name2,
            admin_code2,
            admin_name3,
            admin_code3,
            latitude,
            longitude,
            accuracy
        ).as_str();
    }

    code += "];";

    code
}

fn main() {
    let root_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = env::var_os("OUT_DIR").unwrap();

    let output = Path::new(&build_dir).join("codes.rs");
    let input = Path::new(&root_dir).join("data/postcodes.csv");

    fs::write(&output,codes(input)).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/postcodes.csv");
}