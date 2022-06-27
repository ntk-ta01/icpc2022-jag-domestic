use proconio::{fastout, input, marker::Chars, source};
use std::{
    collections::HashMap,
    io::{self, BufReader},
};
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
            s: [Chars; k],
        }
        // 各単語ごとにすべての変換をメモする L := max(|s_i|) としてO(L^3)
        let mut trans_per_word = HashMap::new();
        let mut word2len = HashMap::new();
        for si in s.iter() {
            let n = si.len();
            for i in 1..n - 1 {
                for j in i + 1..n {
                    // [i,j)を数字とする
                    // si[:i] + (j - i) + si[j:]
                    // let v_s = si[..i] + (('0' as usize + (j - i)) as u8 as char) + si[j..];
                    let mut v_s = vec![];
                    for ch in &si[..i] {
                        v_s.push(*ch);
                    }
                    v_s.push(('0' as usize + (j - i)) as u8 as char);
                    for ch in &si[j..] {
                        v_s.push(*ch);
                    }
                    let e = trans_per_word
                        .entry(si.iter().collect::<String>())
                        .or_insert(vec![]);
                    let s = v_s.into_iter().collect::<String>();
                    e.push(s.clone());
                    word2len.insert(s, j - i);
                }
            }
        }
        // すべての単語のすべての変換の個数をメモする
        let mut word_count = HashMap::new();
        for (_, vs) in trans_per_word.iter() {
            for s in vs.iter() {
                let e = word_count.entry(s.as_str()).or_insert(0);
                *e += 1;
            }
        }
        // 各単語ごとに変換の個数が1だって、値が最大の変換を用いる
        let mut ans = 0;
        for si in s.iter() {
            let mut now = 0;
            if let Some(vs) = trans_per_word.get(&si.iter().collect::<String>()) {
                for ts in vs {
                    if word_count[ts.as_str()] > 1 {
                        continue;
                    }
                    now = now.max(word2len[ts]);
                }
            }
            ans += now;
        }
        println!("{}", ans);
        input! {
            from &mut source,
            new_k: usize,
        }
        k = new_k;
    }
}
