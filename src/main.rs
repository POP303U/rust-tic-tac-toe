use std::io::{self, Write};
use rand::Rng;

const BOARD_SIZE: usize = 3;
const PLAYER: char = 'X';
const COMPUTER: char = 'O';

fn reset_board(board: &mut [[char; BOARD_SIZE]; BOARD_SIZE]) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = ' ';
        }
    }
}

fn print_board(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    print!("\x1B[2J\x1B[1;1H"); 
    println!("#---------------------------------------------------#");
    println!("|          rust-tic-tac-toe [version 1.0]           |");
    println!("#---------------------------------------------------#\n");
    println!("                #############                     ");
    println!("                # {} | {} | {} #",board[0][0],board[0][1],board[0][2]);
    println!("                # --------- #");
    println!("                # {} | {} | {} #",board[1][0],board[1][1],board[1][2]);
    println!("                # --------- #");
    println!("                # {} | {} | {} #",board[2][0],board[2][1],board[2][2]);
    println!("                #############                     ");
    println!("#---------------------------------------------------#");
}

fn check_free_spaces(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> i32 {
    let mut free_spaces = 9;
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j] != ' ' {
                free_spaces -= 1;
            }
        }
    }
    free_spaces
}

fn player_move(board: &mut [[char; BOARD_SIZE]; BOARD_SIZE]) {
    loop {
        print_flush(String::from("Enter row (1-3): "));
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let x: usize = input.trim().parse().expect("Invalid input");

        print_flush(String::from("Enter column (1-3): "));
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let y: usize = input.trim().parse().expect("Invalid input");

        let x = x - 1;
        let y = y - 1;

        if board[x][y] != ' ' {
            println!("Invalid move");
        } else {
            board[x][y] = PLAYER;
            break;
        }
    }
}

fn computer_move(board: &mut [[char; BOARD_SIZE]; BOARD_SIZE]) {
    let mut x: usize;
    let mut y: usize;
    if check_free_spaces(board) > 0 {
        loop {
            x = rand::thread_rng().gen_range(0..BOARD_SIZE);
            y = rand::thread_rng().gen_range(0..BOARD_SIZE);

            if board[x][y] == ' ' {
                board[x][y] = COMPUTER;
                break;
            }
        }
    } else {
        print_winner(' ');
    }
}

fn check_winner(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> char {
    // rows
    for i in 0..BOARD_SIZE {
        if board[i][0] == board[i][1] && board[i][0] == board[i][2] {
            return board[i][0];
        }
    }
    // columns
    for i in 0..BOARD_SIZE {
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            return board[0][i];
        }
    }
    // diagonals
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return board[0][0];
    }
    if board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return board[0][2];
    }

    return ' ';
}

fn print_winner(winner: char) {
    match winner {
        PLAYER => println!("You win"),
        COMPUTER => println!("Computer wins"),
        _ => println!("It's a tie"),
    }
}

fn main() {
    let mut board: [[char; BOARD_SIZE]; BOARD_SIZE] = [[' '; BOARD_SIZE]; BOARD_SIZE];
    let mut winner = ' ';
    reset_board(&mut board);
    while winner == ' ' && check_free_spaces(&board) != 0 {
        print_board(&board);

        player_move(&mut board);
        winner = check_winner(&board);
        if winner != ' ' && check_free_spaces(&board) == 0 {
            break;
        }

        computer_move(&mut board);
        winner = check_winner(&board);
        if winner != ' ' && check_free_spaces(&board) == 0 {
            break;
        }
    }
    print_board(&board);
    print_winner(winner);

    print_flush(String::from("Do you want to play again? (y/n) "));
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if input.trim() == "y" {
        main();
    }
}

fn print_flush(input: String) {
    print!("{input}");
    io::stdout().flush().expect("Failed to flush");
}
