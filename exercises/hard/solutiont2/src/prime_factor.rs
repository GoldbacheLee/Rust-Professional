use rand::Rng;

/// 返回 number 的最大质因子，如果 number 为 0 则 panic
pub fn find_max_prime_factor(number: u128) -> u128 {
    if number == 0 {
        panic!("0 没有定义的质因子");
    }
    if number == 1 {
        return 1;
    }
    let mut max_factor = 1;
    let mut n = number;

    // 处理偶数
    if n % 2 == 0 {
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    // 处理奇数因子
    if n > 1 {
        let prime_factors = factorize(n);
        if let Some(&max) = prime_factors.iter().max() {
            max_factor = max_factor.max(max);
        }
    }

    max_factor
}

/// 分解质因数并返回所有质因数
fn factorize(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    let mut stack = vec![n];

    while let Some(x) = stack.pop() {
        if x == 1 {
            continue;
        }
        if is_prime(x) {
            factors.push(x);
        } else {
            let divisor = pollards_rho(x);
            stack.push(divisor);
            stack.push(x / divisor);
        }
    }

    factors
}

/// Pollard's Rho 算法（只保留此一个版本）
fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n);
    let mut y = x;
    let mut d = 1;
    let f = |x: u128| (x.wrapping_mul(x).wrapping_add(1)) % n;

    while d == 1 {
        x = f(x);
        y = f(f(y));
        d = gcd(if x > y { x - y } else { y - x }, n);
    }

    if d == n {
        // 如果因子等于 n，则递归尝试 n+1（或者其他扰动策略）
        return pollards_rho(n + 1);
    }
    d
}

/// 最大公约数
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Miller-Rabin 质数测试
fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let (d, s) = {
        let mut d = n - 1;
        let mut s = 0;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        (d, s)
    };

    // 针对 u128 优化的确定性基数组
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in bases.iter() {
        if a >= n {
            continue;
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut passed = false;
        for _ in 0..s - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                passed = true;
                break;
            }
        }
        if !passed {
            return false;
        }
    }
    true
}

/// 快速幂取模
fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modulus;
        }
        exp >>= 1;
        base = base.wrapping_mul(base) % modulus;
    }
    result
}
