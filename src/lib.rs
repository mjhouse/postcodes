use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};
use serde::{Deserialize, Deserializer};
use isocountry::CountryCode;
use serde;

include!(concat!(env!("OUT_DIR"), "/codes.rs"));

// // associate post code values with an index into the postal code collection
// static POSTCODES_CODE_INDEX: Lazy<HashMap<String,usize>> = Lazy::new(|| {

// });

// // associate country codes with a vec of coordinates into the postal code collection
// static POSTCODES_COUNTRY_INDEX: Lazy<HashMap<CountryCode,Vec<usize>>> = Lazy::new(|| {

// });

// // associate coordinates with an index into the postal code collection
// static POSTCODES_COORDINATE_INDEX: Lazy<Quadtree::<u64, usize>> = Lazy::new(|| {

// });

fn country<'de, D>(deserializer: D) -> Result<CountryCode, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .and_then(|s| CountryCode::for_alpha2(s)
            .map_err(serde::de::Error::custom))
}

#[derive(Deserialize, Debug)]
pub struct PostalCode {
    #[serde(deserialize_with = "country")]
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

    pub fn for_country() -> Vec<Self> {
        unimplemented!();
    }

    pub fn for_place() -> Vec<Self> {
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
