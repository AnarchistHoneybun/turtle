mod json_interface;
use json_interface::{get_random_word, get_debug_word, check_word};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io::stdout;
use std::thread::sleep;

const BOARD_WIDTH: u16 = 40;
const BOARD_HEIGHT: u16 = 25;

const POPUP_WIDTH: u16 = 20;
const POPUP_HEIGHT: u16 = 3;

struct App {
    current_word: String,
    game_board: GameBoard,
    remaining_attempts: u8,
    current_attempt: u8,
    win_flag: bool,
    win_frame_delay: u8,
    invalid_word_flag: bool,
    invalid_word_frame_delay: u8,
}

struct GameBoard {
    current_row: u8,
    rows: Vec<GameRow>,
}

struct GameRow {
    current_block: u8,
    blocks: Vec<GameBlock>,
}

struct GameBlock {
    current_letter: char,
    state: BlockState,
}

struct BlockState {
    bg_color: Color,
    fg_color: Color,
}

fn init() -> App {
    let mut game_board = GameBoard {
        current_row: 0,
        rows: Vec::new(),
    };

    for _ in 0..6 {
        let mut row = GameRow {
            current_block: 0,
            blocks: Vec::with_capacity(5),
        };

        for _ in 0..5 {
            row.blocks.push(GameBlock {
                current_letter: ' ',
                state: BlockState {
                    bg_color: Color::Red,
                    fg_color: Color::White,
                },
            });
        }

        game_board.rows.push(row);
    }

    App {
        // current_word: get_random_word(),
        current_word: get_debug_word(),
        game_board,
        remaining_attempts: 6,
        current_attempt: 0,
        win_flag: false,
        win_frame_delay: 0,
        invalid_word_flag: false,
        invalid_word_frame_delay: 0,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app: App = init();

    // Setup terminal
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let full_board = frame.size();


            if full_board.height < BOARD_HEIGHT || full_board.width < BOARD_WIDTH {
                frame.render_widget(
                    Paragraph::new("Terminal too small")
                        .style(Style::default().fg(Color::Red))
                        .block(Block::default().borders(Borders::ALL))
                        .alignment(Alignment::Center),
                    full_board,
                );
            } else {
                let [horizontal_center] = if full_board.width > BOARD_WIDTH {
                    Layout::horizontal([Constraint::Length(BOARD_WIDTH)])
                        .flex(Flex::Center)
                        .areas(full_board)
                } else {
                    Layout::horizontal([Constraint::Fill(1)]).areas(full_board)
                };

                let [vertical_center] = if full_board.height > BOARD_HEIGHT {
                    Layout::vertical([Constraint::Length(BOARD_HEIGHT)])
                        .flex(Flex::Center)
                        .areas(horizontal_center)
                } else {
                    Layout::vertical([Constraint::Fill(1)]).areas(horizontal_center)
                };

                let game_rows: [Rect; 6] = Layout::vertical([
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(1, 6),
                ])
                .areas(vertical_center);

                let mut game_columns: Vec<Vec<Rect>> = Vec::with_capacity(6);

                for row in &game_rows {
                    let row_columns: Vec<Rect> = Layout::horizontal([
                        Constraint::Ratio(1, 5),
                        Constraint::Ratio(1, 5),
                        Constraint::Ratio(1, 5),
                        Constraint::Ratio(1, 5),
                        Constraint::Ratio(1, 5),
                    ])
                    .areas::<5>(*row)
                    .iter()
                    .cloned()
                    .collect();
                    game_columns.push(row_columns);
                }

                for (i, row) in app.game_board.rows.iter().enumerate() {
                    for (j, block) in row.blocks.iter().enumerate() {
                        let content = &block.current_letter.to_string();
                        frame.render_widget(
                            Paragraph::new(format!("{}", content))
                                .style(Style {
                                    bg: Option::from(block.state.bg_color),
                                    fg: Option::from(block.state.fg_color),
                                    ..Default::default()
                                })
                                .block(Block::default().borders(Borders::ALL))
                                .alignment(Alignment::Center),
                            game_columns[i][j],
                        )
                    }
                }

                if app.win_flag {

                    if app.win_frame_delay <= 3 {
                        app.win_frame_delay += 1;
                    } else if app.win_frame_delay > 3 && app.win_frame_delay <= 6 {

                        app.win_frame_delay += 1;

                        let [horizontal_center] = Layout::horizontal([Constraint::Length(POPUP_WIDTH)])
                            .flex(Flex::Center)
                            .areas(full_board);
                        let [vertical_center] = Layout::vertical([Constraint::Length(POPUP_HEIGHT)])
                            .flex(Flex::Center)
                            .areas(horizontal_center);
                        let popup = Paragraph::new("You win!")
                            .style(Style::default().fg(Color::Black).bg(Color::Green))
                            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double))
                            .alignment(Alignment::Center);
                        frame.render_widget(Clear, vertical_center);
                        frame.render_widget(popup, vertical_center);
                    } else {
                        sleep(std::time::Duration::from_secs(2));
                        app = init();
                    }
                }

                if app.invalid_word_flag {
                    if app.invalid_word_frame_delay <= 3 {
                        app.invalid_word_frame_delay += 1;
                    } else if app.invalid_word_frame_delay > 3 && app.invalid_word_frame_delay <= 6 {
                        app.invalid_word_frame_delay += 1;

                        let [horizontal_center] = Layout::horizontal([Constraint::Length(POPUP_WIDTH)])
                            .flex(Flex::Center)
                            .areas(full_board);
                        let [vertical_center] = Layout::vertical([Constraint::Length(POPUP_HEIGHT)])
                            .flex(Flex::Center)
                            .areas(horizontal_center);
                        let popup = Paragraph::new("Invalid word")
                            .style(Style::default().fg(Color::Black).bg(Color::Red))
                            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double))
                            .alignment(Alignment::Center);
                        frame.render_widget(Clear, vertical_center);
                        frame.render_widget(popup, vertical_center);
                    } else {
                        sleep(std::time::Duration::from_secs(2));
                        app.invalid_word_flag = false; // Reset the invalid_word_flag
                        app.invalid_word_frame_delay = 0;
                    }
                }
            }
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
                if let KeyCode::Char(c) = key.code {
                    if c.is_ascii_alphabetic() {
                        let current_row = app.game_board.current_row as usize;
                        let current_block = app.game_board.rows[current_row].current_block as usize;

                        if current_block < 5 {
                            app.game_board.rows[current_row].blocks[current_block].current_letter =
                                c.to_ascii_uppercase();
                            app.game_board.rows[current_row].current_block += 1;
                        }
                    }
                } else if key.code == KeyCode::Backspace {
                    let current_row = app.game_board.current_row as usize;
                    let current_block = app.game_board.rows[current_row].current_block as usize;

                    if current_block > 0 {
                        app.game_board.rows[current_row].current_block -= 1;
                        app.game_board.rows[current_row].blocks[current_block - 1].current_letter = ' ';
                    }
                } else if key.code == KeyCode::Enter {
                    let current_row = app.game_board.current_row as usize;
                    let current_block = app.game_board.rows[current_row].current_block;

                    if current_block == 5 {
                        let entered_word: String = app
                            .game_board
                            .rows[current_row]
                            .blocks
                            .iter()
                            .map(|block| block.current_letter)
                            .collect();
                        let entered_word = entered_word.to_lowercase();
                        if check_word(&entered_word) {
                            let current_word_chars: Vec<char> = app.current_word.chars().collect();
                            let mut remaining_word_chars: Vec<char> = current_word_chars.clone();

                            for (i, block) in app.game_board.rows[current_row].blocks.iter_mut().enumerate() {
                                if block.current_letter == current_word_chars[i] {
                                    block.state.bg_color = Color::Green;
                                    block.state.fg_color = Color::Black;
                                    remaining_word_chars[i] = ' ';
                                    if remaining_word_chars.iter().all(|&c| c == ' ') {
                                        app.win_flag = true;
                                    }
                                } else {
                                    block.state.bg_color = Color::Red;
                                    block.state.fg_color = Color::White;
                                }
                            }

                            for (i, block) in app.game_board.rows[current_row].blocks.iter_mut().enumerate() {
                                if remaining_word_chars.contains(&block.current_letter) {
                                    block.state.bg_color = Color::Yellow;
                                    block.state.fg_color = Color::Black;
                                    let remaining_char_index = remaining_word_chars.iter().position(|&c| c == block.current_letter).unwrap();
                                    remaining_word_chars[remaining_char_index] = ' ';
                                }
                            }


                            if app.game_board.current_row < 5 {
                                app.game_board.current_row += 1;
                            }
                        } else {
                            // Set the invalid word flags
                            app.invalid_word_flag = true;
                            app.invalid_word_frame_delay = 4; // Skip the initial delay frames

                            // Clear the current row
                            let current_row = app.game_board.current_row as usize;
                            for block in &mut app.game_board.rows[current_row].blocks {
                                block.current_letter = ' ';
                                block.state.bg_color = Color::Red;
                                block.state.fg_color = Color::White;
                            }
                            app.game_board.rows[current_row].current_block = 0;
                        }
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn reset_game(app: &mut App) {
    app.current_word = "AUDIO".to_string(); // Change this to a new random word
    app.remaining_attempts = 6;
    app.current_attempt = 0;
    app.game_board = GameBoard {
        current_row: 0,
        rows: Vec::new(),
    };

    for _ in 0..6 {
        let mut row = GameRow {
            current_block: 0,
            blocks: Vec::with_capacity(5),
        };

        for _ in 0..5 {
            row.blocks.push(GameBlock {
                current_letter: ' ',
                state: BlockState {
                    bg_color: Color::Red,
                    fg_color: Color::White,
                },
            });
        }

        app.game_board.rows.push(row);
    }
}
