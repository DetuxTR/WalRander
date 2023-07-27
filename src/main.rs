use serde_json::json;

mod config;
mod getwps;
mod json;
mod walbackend;
mod mjson;
mod backproccess;
mod random;

fn main()
{
        let configed = config::main();
        let mjsoned = mjson::loadjson(mjson::initjson());

        let wals = getwps::get_files(mjsoned.directories.clone());
        backproccess::pr(configed.delay, wals,mjsoned.lastwp);

}

