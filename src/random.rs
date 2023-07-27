use rand;
use rand::Rng;
use crate::mjson::Json;


pub fn random (dirs: Vec<String>,chngfunc : fn(&str,Json), lastwp : Vec<String>,jsn:Json) -> String
{
    loop {
        let latestwp = lastwp.clone();
        let mut random = rand::thread_rng();
        let randomwp : usize = random.gen_range(0..dirs.len());
        if dirs[randomwp].clone() == latestwp[0]
        {
            continue
        }
        println!("Random Data {}", dirs[randomwp].clone());
        println!("{}",latestwp[0]);
        chngfunc(&*dirs[randomwp].clone(), jsn);
        return dirs[randomwp].clone();


    }

}