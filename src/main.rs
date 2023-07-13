mod config;
mod loadwp;
mod json;

fn main()
{
        let configed = config::main();
        loadwp::get_files("./");

}

