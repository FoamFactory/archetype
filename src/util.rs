pub fn get_version_code_from_string(version_string: &str) -> u32 {
    let mut multiplier: u32 = 1;
    let tokens: Vec<u32> = version_string
        .split(".")
        .map(|s| String::from(s))
        .map(|s| {
            multiplier = multiplier + 2;
            multiplier * (s.parse::<u32>().unwrap())
        })
        .collect();
    tokens.into_iter().sum()
}