use std::io::stdout;

pub enum Ops
{
    SetWp,
    ReloadWal
}

pub fn main (opts:Ops,w_path:&str)
{
    match opts {
        Ops::SetWp => {set_wp(w_path)}
        Ops::ReloadWal => {reload_wal()}
        _ => {eprintln!("Wrong Usage")}
    }
}

fn set_wp(wp_path:&str)
{
    std::process::Command::new("wal")
        .arg("-i")
        .arg(wp_path)
        .output().unwrap();


}

fn reload_wal()
{
    std::process::Command::new("wal")
        .arg("-R")
        .output().unwrap();

}