pub fn dp_rec_mc(amount: u32) -> u32 {
    let bills = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![amount + 1; (amount + 1) as usize];

    dp[0] = 0; // 初始状态，找零0元需要0张纸币

    for i in 1..=amount as usize {
        for &bill in &bills {
            if i as u32 >= bill {
                dp[i] = dp[i].min(dp[i - bill as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == amount + 1 {
        0 // 找不到解
    } else {
        dp[amount as usize]
    }
}
