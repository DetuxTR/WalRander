use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize,Serialize)]
pub struct Json
{
    pub(crate) directories : Vec<String>
}

pub struct jsoned
{
    pub(crate) directories : Vec<String>
}

pub fn initjson() -> Json
{
    let jsonfile=&fs::read_to_string("mdirs.json").unwrap() as &str;
    let json : Json = serde_json::from_str(jsonfile).unwrap();
    json
}

pub fn loadjson(unljson : Json) -> jsoned
{
    let mut dirs : Vec<String> = vec![];


    for dirent in unljson.directories
    {

        dirs.push(dirent);
    }
    let fnl_json = jsoned
    {
        directories:dirs
    };
    fnl_json
}