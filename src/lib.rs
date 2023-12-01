pub fn generic_parser(cmd: &str, seps: Vec<char>) -> Vec<&str> {
    let mut parsed: Vec<&str> = Vec::new();
    let mut last = 0;
    for (index, matched) in cmd.match_indices(|c: char| (seps.contains(&c))) {
        if last != index {
            parsed.push(&cmd[last..index]);
        }
        parsed.push(matched);
        last = index + matched.len();
    }
    if last < cmd.len() {
        parsed.push(&cmd[last..])
    }
    parsed
}

