// the garden module contains three structs called `Asparagus`,
// `Pumpkin` and `Carrot`. `main.rs` imports them as crates using
// the "::" notation to represetn levels in the tree.
use crate::garden::vegetables::Asparagus;
use crate::garden::vegetables::Pumpkin;
use crate::garden::vegetables::Carrot;

// and garden needs to be set to a public module like so
pub mod garden;

fn main() {
    let plant1 = Asparagus {};
    println!("I'm growing {:?}!", plant1);

    let plant2 = Pumpkin {};
    println!("I'm growing {:?}!", plant2);

    let plant3 = Carrot {};
    println!("I'm growing {:?}!", plant3);
}
