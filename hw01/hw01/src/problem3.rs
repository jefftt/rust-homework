/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut i = 2;
    let mut check = vec![false; (n + 1) as usize];

    while i < n {
        let mut j = i;
        if !check[j as usize] {
            out.push(j as u32);
        }
        while j < n {
            check[j as usize] = true;
            j = j + i;
        }
        i = i + 1
    }
    out
}
