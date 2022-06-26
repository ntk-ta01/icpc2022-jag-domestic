use proconio::{fastout, input, marker::Chars, source};
use std::io::{self, BufReader};
#[fastout]
fn main() {
    let mut source = source::line::LineSource::new(BufReader::new(io::stdin()));
    input! {
        from &mut source,
        mut k: usize,
    }
    while k != 0 {
        input! {
            from &mut source,
            s: Chars,
        }
        // let n = s.len(); n mod k == 0;

        input! {
            from &mut source,
            new_k: usize,
        }
        k = new_k;
    }
}
