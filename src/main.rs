mod config;
mod getwps;
mod json;
mod walbackend;

fn main()
{
        let configed = config::main();
        let wploc = getwps::get_files("./") ;
        walbackend::main(walbackend::Ops::SetWp,&wploc[1] as &str);
}

