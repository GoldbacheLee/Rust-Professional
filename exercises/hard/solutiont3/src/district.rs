use std::collections::{HashMap, HashSet};

pub fn count_provinces() -> String {
    let mut data = parse(std::fs::read_to_string("./district.json").expect("Failed to read file"))
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();

    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let data = data.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    fn bfs(start: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
        let mut nodes = vec![start.to_owned()];

        visited.insert(start.to_string());

        while let Some(node) = nodes.pop() {
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if visited.contains(neighbor) {
                        continue;
                    }
                    visited.insert(neighbor.clone());
                    nodes.push(neighbor.to_owned());
                }
            }
        }
    }

    let mut results = Vec::new();

    for v in data.into_iter() {
        // 构建图
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for (city, neighbors) in v {
            graph
                .entry(city.clone())
                .or_default()
                .extend(neighbors.clone());
            for ele in neighbors {
                graph.entry(ele).or_default().push(city.clone());
            }
        }


        // 计算连通分量数量
        let mut visited = HashSet::new();
        let mut province_count = 0;

        for city in graph.keys() {
            if visited.contains(city) {
                continue;
            }

            bfs(city, &graph, &mut visited);
            province_count += 1;
        }

        results.push(province_count.to_string());
    }

    results.join(",")
}

fn parse(file_content: String) -> HashMap<String, HashMap<String, HashSet<String>>> {
    let mut total_hashmap: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
    let mut tmp_hashmap = HashMap::new();
    let mut number = String::new();
    for i in file_content.lines() {
        let line = i.trim();
        if line.contains(":") && line.starts_with("\"") && line.ends_with("{") {
            tmp_hashmap.clear();
            number = (line.as_bytes()[1] - 48).to_string();
            continue;
        } else if line.starts_with("\"") && (line.ends_with("],") || line.ends_with("]")) {
            let tmp: Vec<_> = line.split(":").map(|s| s.to_string()).collect();
            let city = tmp[0].trim().trim_matches('"').to_string();
            let city_table: HashSet<String> = tmp[1]
                .trim()
                .trim_start_matches("[")
                .trim_end_matches("],")
                .trim_end_matches(']')
                .split(",")
                .map(|s| s.trim().trim_matches(|c| c == '"').to_string())
                .collect();

            let mut merged = false;
            for (existing_city, existing_set) in tmp_hashmap.iter_mut() {
                if !city_table.is_disjoint(existing_set) || existing_city == &city {
                    existing_set.insert(city.clone());
                    existing_set.extend(city_table.clone());
                    merged = true;
                    break;
                }
            }

            if !merged {
                tmp_hashmap.insert(city, city_table);
            }
        } else if line == "}," || line == "}" {
            total_hashmap.insert(number.clone(), tmp_hashmap.clone());
            continue;
        }
    }
    total_hashmap
}