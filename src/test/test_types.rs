use msfx::Types;
use strum::IntoEnumIterator;

fn main() {
    for t in Types::iter() {
        println!("Type: to_string {}, default {}", t.to_string(), t);
    }
}
