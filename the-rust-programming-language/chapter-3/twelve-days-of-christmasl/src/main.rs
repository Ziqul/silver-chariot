extern crate yaml_rust;

use std::fs;
use yaml_rust::YamlLoader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lyrics =
        fs::read_to_string("lyrics.yaml")?;

    let lyrics =
        &YamlLoader::load_from_str(&lyrics)?[0];

    for elem in lyrics["lyrics"].as_vec().unwrap() {
        println!(
            "On the {} day of Christmas {}",
            elem["day"].as_str().unwrap(),
            elem["continuation"].as_str().unwrap());
    }

    Ok(())
}
