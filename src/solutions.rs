use crate::file::FileReader;

static WIN_PATTERNS : [u32 ; 10] = [
    0b1111100000000000000000000,
    0b0000011111000000000000000,
    0b0000000000111110000000000,
    0b0000000000000001111100000,
    0b0000000000000000000011111,
    0b1000010000100001000010000,
    0b0100001000010000100001000,
    0b0010000100001000010000100,
    0b0001000010000100001000010,
    0b0000100001000010000100001
];

struct BingoBoard {
    numbers : [u32 ; 25],
    marks : u32
}

impl BingoBoard {
    pub fn parse(reader : &mut FileReader) -> Self {
        let marks = 0;
        let mut numbers : Vec<u32> = Vec::new();
        
        for _ in 0..5 {
            let row = reader.next().unwrap();

            for num in row.split_whitespace() {
                numbers.push(num.parse().unwrap())
            }
        }

        return BingoBoard {
            marks,
            numbers: numbers.try_into().unwrap()
        }
    }

    pub fn mark_board(&mut self, number : u32) {
        let mut index = 0;
        let mut found = false;

        for i in 0..self.numbers.len() {
            if self.numbers[i] == number {
                index = i;
                found = true;
                break;
            }
        }

        if !found {
            return;
        }

        let marker = 2u32.pow(index as u32);
        self.marks |= marker;
    }

    pub fn has_won(&self) -> bool{
        for win_pattern in WIN_PATTERNS {
            if self.marks & win_pattern == win_pattern {
                return true;
            }
        }

        return false;
    }

    pub fn get_score(&self) -> u32 {
        let mut score = 0;

        for index in 0..self.numbers.len() {
            let bit_flag = 2u32.pow(index as u32);
            if !self.marks & bit_flag != 0 {
                score += self.numbers[index];
            }
        }

        return score;
    }
}

pub fn part_one(mut lines : FileReader) -> u32 {
    let first_line = lines.next().unwrap();

    let numbers = first_line.split(',').map(|s| s.parse::<u32>().unwrap());
    let mut boards  = Vec::new();

    while let Some(_) = lines.next() {
        boards.push(BingoBoard::parse(&mut lines));
    }

    let mut winning_board = 0;
    let mut found_board = false;
    let mut last_num = 0;

    'outer: for number in numbers {
        last_num = number;
        for (index, board) in boards.iter_mut().enumerate() {
            board.mark_board(number);
            if board.has_won() {
                winning_board = index;
                found_board = true;
                break 'outer;
            }
        }
    }

    if !found_board {
        panic!("No board won!");
    }

    return last_num * boards[winning_board].get_score();
}

pub fn part_two(mut lines : FileReader) -> u32 {
    let first_line = lines.next().unwrap();

    let numbers = first_line.split(',').map(|s| s.parse::<u32>().unwrap());
    let mut boards  = Vec::new();

    while let Some(_) = lines.next() {
        boards.push(BingoBoard::parse(&mut lines));
    }

    let num_boards = boards.len();
    let mut last_num = 0;
    let mut winning_boards = Vec::new();
    let mut last_board = 0;

    'outer: for number in numbers {
        last_num = number;
        for (index, board) in boards.iter_mut().enumerate() {
            if winning_boards.contains(&index) {
                continue;
            }

            board.mark_board(number);
            if board.has_won() {
                winning_boards.push(index);
                if winning_boards.len() == num_boards {
                    last_board = index;
                    break 'outer;
                }
            }
        }
    }

    return last_num * boards[last_board].get_score();
}