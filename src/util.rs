pub fn create_board(data: &str) -> Vec<Vec<char>> {
    data.trim()
        .split('\n')
        .map(|row| row.trim().chars().collect())
        .collect()
}

pub fn print_board(board: &[Vec<char>]) {
    for row in board{
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}
