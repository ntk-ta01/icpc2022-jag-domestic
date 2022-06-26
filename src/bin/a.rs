use proconio::{fastout, input, source};
use std::io::{self, BufReader};
#[fastout]
fn main() {
    let mut source = source::line::LineSource::new(BufReader::new(io::stdin()));
    input! {
        from &mut source,
        mut n: usize,
    }
    while n != 0 {
        input! {
            from &mut source,
            mut a: [usize; n],
        }
        a.sort_unstable();
        let mut ans = 1;
        let mut now = 1;
        let mut pre = a[0];
        for &ai in a.iter().skip(1) {
            if pre + 1 == ai {
                now += 1;
            } else {
                now = 1;
            }
            if ans < now {
                ans = now;
            }
            pre = ai;
        }
        println!("{}", ans);
        input! {
            from &mut source,
            new_n: usize,
        }
        n = new_n;
    }
}
