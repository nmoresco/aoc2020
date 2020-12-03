pub fn create_board_char(data: &str) -> Vec<Vec<char>> {
    data.trim()
        .split("\n")
        .map(|row| row.trim().chars().collect())
        .collect()
}
