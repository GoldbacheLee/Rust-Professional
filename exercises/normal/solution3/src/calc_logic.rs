pub fn new_birthday_probability(n: u32) -> f64 {

        // 至少 2 个人才能计算概率
        if n < 2 {
            return 0.0;
        }
        
        let mut prob_unique = 1.0; // 生日都不同的概率
        let mut days = 365.0;
    
        for i in 0..n {
            if i > 0 {
                prob_unique *= (days - i as f64) / days;
            }
        }
    
        // 至少两人生日相同的概率
        let prob_same = 1.0 - prob_unique;
        
        // 保留 4 位小数
        (prob_same * 10000.0).round() / 10000.0
}
