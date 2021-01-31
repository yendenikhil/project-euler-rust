// time to implement better algorithm than brute force approach
// using lexicographical order to generate this.
fn perm(list: &[u8]) -> Vec<Vec<u8>> {
    let mut curr = list.to_vec();
    let mut ans = Vec::new();
    ans.push(curr.to_vec());
    loop {
        let mut i = curr.len() - 1;
        let mut j = curr.len() - 1;
        let mut k = curr.len() - 1;
        while i > 0 && curr[i - 1] >= curr[i] {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        while curr[j] <= curr[i - 1] {
            j -= 1;
        }
        curr.swap(i - 1, j);
        while i < k {
            curr.swap(i, k);
            i += 1;
            k -= 1;
        }
        ans.push(curr.to_vec());
    }
    ans
}

pub fn run() {
    let nums = perm(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let sum: usize = nums
        .iter()
        .filter(|&v| v[3] % 2 == 0)
        .filter(|&v| (v[2] + v[3] + v[4]) % 3 == 0)
        .filter(|&v| v[5] % 5 == 0)
        .filter(|&v| (v[4] as usize * 100 + v[5] as usize * 10 + v[6] as usize) % 7 == 0)
        .filter(|&v| (v[5] as usize * 100 + v[6] as usize * 10 + v[7] as usize) % 11 == 0)
        .filter(|&v| (v[6] as usize * 100 + v[7] as usize * 10 + v[8] as usize) % 13 == 0)
        .filter(|&v| (v[7] as usize * 100 + v[8] as usize * 10 + v[9] as usize) % 17 == 0)
        .map(|v| {
            v.iter()
                .map(|e| e.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("Answer: {}", sum);
}
