use serde_json::json;

mod config;
mod getwps;
mod json;
mod walbackend;
mod mjson;

fn main()
{
        let configed = config::main();



        let mjsoned = mjson::loadjson(mjson::initjson());
        let wploc = getwps::get_files(mjsoned.directories.clone()) ;


}

