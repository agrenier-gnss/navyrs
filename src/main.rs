
use std::path::Path;
use rinex::{self, Rinex};

fn main() {
    println!("Hello, world!");

    let rinex_path = Path::new("../data/TAUN00FI__R_20221450000_01D_30S_MO.22o");

    // Load RINEX file
    if let Ok(rinex) = Rinex::from_path(rinex_path){
        println!("rinex.first_epoch()");
    }

}
