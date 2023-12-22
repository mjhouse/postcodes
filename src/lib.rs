use serde::{Serialize,Deserialize};
use isocountry::CountryCode;
use serde;

pub mod codes;
pub mod indices;

type CodeReference = (
    CountryCode,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    Option<f64>,
    Option<f64>,
    Option<u8>
);

#[derive(Serialize,Deserialize, Debug)]
pub struct PostalCode {
    country_code: CountryCode,
    postal_code: String,
    place_name: String,
    admin_name1: String,
    admin_code1: String,
    admin_name2: String,
    admin_code2: String,
    admin_name3: String,
    admin_code3: String,
    latitude: f64,
    longitude: f64,  
    #[serde(default)]
    accuracy: Option<u8>
}

impl PostalCode {

    pub fn for_code(code: String) -> Self {
        unimplemented!();
    }

    pub fn for_country(country: CountryCode) -> Vec<Self> {
        unimplemented!();
    }

    pub fn for_region(longitude: f64, latitude: f64) -> Vec<Self> {
        unimplemented!();
    }

    pub fn for_search(value: String) -> Vec<Self> {
        unimplemented!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postal_codes_deserialize() {
        // let code = POSTCODES.get(0);
        // dbg!(code);
    }
}
