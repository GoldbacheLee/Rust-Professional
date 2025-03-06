pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut a = 1;  // 第一项
    let mut b = 1;  // 第二项
    let mut sum = 0;

    while a <= threshold {
        if a % 2 != 0 {  // 判断是否为奇数
            sum += a;
        }
        // 更新斐波那契数
        let temp = a;
        a = b;
        b = temp + b;
    }

    sum
}