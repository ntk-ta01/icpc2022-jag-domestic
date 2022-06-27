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
        const TEN: [i64; 15] = [
            1,
            10,
            100,
            1_000,
            10_000,
            100_000,
            1_000_000,
            10_000_000,
            100_000_000,
            1_000_000_000,
            10_000_000_000,
            100_000_000_000,
            1_000_000_000_000,
            10_000_000_000_000,
            100_000_000_000_000,
        ];
        let n = s.len();
        let mut dp = vec![vec![0; k]; n + 1];
        for i in 0..n {
            for j in 0..k {
                let di = s[i] as i64 - '0' as i64;
                let nv = dp[i][j] + di * TEN[k - j - 1];
                // s[i]を使わない or 使う
                dp[i + 1][(j + 1) % k] = dp[i][(j + 1) % k];
                // k桁目として、もしくはk - j桁目として選べるか確認 && i+1枚目まで見てj + 1残すときの寄与の最大値なら更新
                if (j == 0 || dp[i][j] > 0) && dp[i + 1][(j + 1) % k] < nv {
                    dp[i + 1][(j + 1) % k] = nv;
                }
            }
        }
        println!("{}", dp[n][0]);

        input! {
            from &mut source,
            new_k: usize,
        }
        k = new_k;
    }
}
