use serde_json;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Json
{
    pub(crate) directories: Vec<String>,
    pub(crate) lastwp: Vec<String>
}

#[derive(Debug)]
pub struct jsoned
{
    pub(crate) directories : Vec<String>,
    pub(crate) lastwp : Vec<String>
}

pub fn initjson() -> Json
{
    let jsonfile=&fs::read_to_string("mdirs.json").unwrap() as &str;
    let mut json : Json = serde_json::from_str(jsonfile).unwrap();
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
        directories:dirs,
        lastwp:unljson.lastwp
    };


    fnl_json
}
pub fn set_lastwp(wpdir:&str, mut unljson:Json)
{
    // println!("{:?}", unljson.lastwp);
    let dir = vec![wpdir];
    //
    // unljson.lastwp.insert(dir.len(), wpdir.parse().unwrap());


    unljson.lastwp.insert(dir.len(),dir[0].parse().unwrap());
    unljson.lastwp.remove(0);



    fs::write(
        "mdirs.json",
        serde_json::to_string_pretty(&unljson).unwrap()
    ).unwrap();
}