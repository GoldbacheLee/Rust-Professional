use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    input_str
    .split(',') // 按逗号分割字符串
    .map(str::trim) // 去除前后空格
    .filter(|s| !s.is_empty()) // 过滤掉空字符串
    .collect::<HashSet<_>>() // 使用 HashSet 去重
    .len() // 计算唯一元素个数
}
