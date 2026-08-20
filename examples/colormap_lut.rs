//! Build and print a fixed-size Viridis lookup table.

#![expect(clippy::print_stdout, reason = "example stdout is the deliverable")]

extern crate iris;

use iris::color::{LookupTable, NamedColorMap};

fn main() {
    let table = LookupTable::<NamedColorMap, 8>::from_map(NamedColorMap::Viridis);
    for color in table.entries() {
        println!("{:?}", color.to_rgba8());
    }
}
