pub fn goldbach_conjecture() -> String {
    // 高效素数判断（6k±1优化）
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        let mut w = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += w;
            w = 6 - w;
        }
        true
    }

    // 检查是否能表示为素数+2倍平方数
    fn can_be_written(n: u32) -> bool {
        let max_i = (n as f64 / 2.0).sqrt() as u32;
        for i in 1..=max_i {
            let remainder = n - 2 * i * i;
            if remainder >= 2 && is_prime(remainder) {
                return true;
            }
        }
        false
    }

    let mut results = Vec::new();
    let mut num = 9;

    while results.len() < 2 {
        if !is_prime(num) && !can_be_written(num) {
            results.push(num.to_string());
        }
        num += 2; // 只检查奇数
    }

    results.join(",")
}