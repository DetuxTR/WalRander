use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize,Serialize)]
pub struct Json
{
    directories : String,
}

pub struct jsoned
{
    pub (crate) directories : String
}
pub fn read_json_file(json_path : &str) -> Json
{
    let raw_json = &fs::read_to_string(json_path).unwrap() as &str;
    let json:Json = serde_json::from_str(raw_json).unwrap();
    json
}

pub fn load_json_file(rwjsn : Json) -> jsoned
{
    let fnl_json = jsoned
    {
        directories: rwjsn.directories
    };

    println!("Directory reading :{}", fnl_json.directories);
    fnl_json
}