use std::thread;
use std::time::Duration;
use crate::mjson::{initjson, loadjson, set_lastwp};
use crate::walbackend::Ops;
use crate::walbackend::main;
use crate::random::random;

pub fn pr(delaytime : i32,walvec : Vec<String>,lastwp : Vec<String>)
{
    loop {


        let jsn = loadjson(initjson());
        let wppath = random(walvec.clone(),set_lastwp,jsn.lastwp,initjson());

        let sec = Duration::from_secs(delaytime as u64);
        thread::sleep(sec);

        main(Ops::SetWp,&wppath as &str);

    }



}