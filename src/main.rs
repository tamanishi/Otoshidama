use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let values = line1
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let n = values[0];
    let y = values[1];
    // println!("{} {}", n, y);

    for i in 0..=n {
        for j in 0..=n - i {
            // println!("{} {} {} {}", i, j, k, i * 10000 + j * 5000 + k * 1000);
            let k = n - i - j;
            if i + j + k == n && i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
