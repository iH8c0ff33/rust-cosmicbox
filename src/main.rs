extern crate cosmicbox;
extern crate hidapi;

use cosmicbox::{CosmicBox, GenericCosmicBox};
use hidapi::HidApi;

fn main() {
    let hid = HidApi::new().unwrap();
    let cb = CosmicBox::connect(hid);
    cb.reset();

    println!("Cosmic Box reset done");
}
