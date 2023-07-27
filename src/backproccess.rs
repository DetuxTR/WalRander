use std::thread;
use std::time::Duration;
use crate::mjson::{initjson, set_lastwp};
use crate::walbackend::Ops;
use crate::walbackend::main;
use crate::random::random;

pub fn pr(delaytime : i32,walvec : Vec<String>,lastwp : Vec<String>)
{
    let wppath = random(walvec,set_lastwp,lastwp,initjson());
    let sec = Duration::from_secs(delaytime as u64);
    thread::sleep(sec);
    println!("Delay is working :7");
    main(Ops::SetWp,&wppath as &str);
}