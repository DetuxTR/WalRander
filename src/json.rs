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
fn read_json_file(filename : &str ) -> Json
{
    let raw_json = fs::read_to_string(filename).unwrap();
    let json:Json = serde_json::from_str(&*raw_json).unwrap();
    json
}

fn load_json_file(rwjsn : Json) -> jsoned
{
    let fnl_json = jsoned
    {
        directories: rwjsn.directories.to_string(),
    };
    fnl_json
}