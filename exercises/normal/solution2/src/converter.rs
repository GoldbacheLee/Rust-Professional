pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入，提取数字部分和原始进制
    let mut parts = num_str.trim().split('(');
    let num_part = parts.next().unwrap().trim();
    let base_part = parts.next().unwrap().trim_end_matches(')').trim();
    
    // 解析原进制
    let from_base = base_part.parse::<u32>().unwrap();
    
    // 将原进制的字符串转换为十进制整数
    let decimal_value = i64::from_str_radix(num_part, from_base).unwrap();

    // 处理特殊情况（十进制为 0 直接返回 "0"）
    if decimal_value == 0 {
        return "0".to_string();
    }

    // 将十进制转换为目标进制
    let mut result = String::new();
    let mut num = decimal_value;

    // 除基取余法
    while num > 0 {
        let remainder = (num % (to_base as i64)) as u32;
        let digit = if remainder < 10 {
            (b'0' + remainder as u8) as char // 数字 0-9
        } else {
            (b'a' + (remainder - 10) as u8) as char // 字母 a-f
        };
        result.push(digit);
        num /= to_base as i64;
    }

    // 结果是倒序的，需要反转
    result.chars().rev().collect()
}