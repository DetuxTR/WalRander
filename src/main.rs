mod config;
mod getwps;
mod json;

fn main()
{
        let configed = config::main();
        getwps::get_files("./");

}

