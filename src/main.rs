use std::default;

use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::executor;
use iced::keyboard as k;
use iced::theme::{Custom, Palette};
use iced::widget::{button, column, container, row, text::Text};
use iced::Color;
use iced::Pixels;
use iced::{Application, Command, Element, Length, Settings, Theme};
use rand::thread_rng;
use rand::Rng;

const FONT_SIZE: f32 = 70.0;

const BUTTONWIDTH: u16 = 200;
const BUTTONHEIGHT: u16 = 200;
const BUTTONPADDING: u16 = 5;

fn main() -> iced::Result {
    Game::run(Settings::default())
}

fn _interesting(game: &Game) {
    for i in game.board {
        for j in i {
            if j.scale > 0 {
                eprintln!("interesting: {:?}", j);
            }
        }
    }
}

fn piece_button(piece: Piece) -> button::Button<'static, Message> {
    button(
        Text::new(piece.as_string())
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(Pixels(FONT_SIZE)),
    )
    .width(BUTTONHEIGHT)
    .height(BUTTONWIDTH)
    .padding(BUTTONPADDING)
    .on_press(Message::None)
}

impl Application for Game {
    type Executor = executor::Default;
    type Theme = Theme;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut game = Self::default();
        game.summon();
        (game, Command::none())
    }

    fn theme(&self) -> Self::Theme {
        //        Theme::custom(Custom::new(
        //          "my_theme".to_string(),
        //          Palette {
        //              background: BLACK,
        //              text: WHITE,
        //              primary: RED,
        //              success: RED,
        //              danger: RED,
        //          },
        //      ))
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("2048")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Up => self.up(),
            Message::Down => self.down(),
            Message::Left => self.left(),
            Message::Right => self.right(),
            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let board = row![
            column![
                piece_button(self.board[0][0]),
                piece_button(self.board[0][1]),
                piece_button(self.board[0][2]),
                piece_button(self.board[0][3]),
            ],
            column![
                piece_button(self.board[1][0]),
                piece_button(self.board[1][1]),
                piece_button(self.board[1][2]),
                piece_button(self.board[1][3]),
            ],
            column![
                piece_button(self.board[2][0]),
                piece_button(self.board[2][1]),
                piece_button(self.board[2][2]),
                piece_button(self.board[2][3]),
            ],
            column![
                piece_button(self.board[3][0]),
                piece_button(self.board[3][1]),
                piece_button(self.board[3][2]),
                piece_button(self.board[3][3]),
            ]
        ];

        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        return k::on_key_press(input).into();
    }
}

fn input(key: k::Key, _module: k::Modifiers) -> Option<Message> {
    let char = match key {
        // k::Key::Character(inner) => inner,
        k::Key::Named(inner) => inner,
        _ => return None,
    };

    return Some(match char {
        k::key::Named::ArrowUp => Message::Up,
        k::key::Named::ArrowDown => Message::Down,
        k::key::Named::ArrowLeft => Message::Left,
        k::key::Named::ArrowRight => Message::Right,
        _ => Message::None,
    });
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    scale: u8,
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, Copy)]
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    board: [[Piece; 4]; 4], // [x][y]
    score: u32,
    moves: u32,
    time: Time,
}

impl Piece {
    fn as_number(&self) -> u32 {
        return (2 as u32).pow(self.scale as u32);
    }
    fn as_string(&self) -> String {
        if self.scale == 0 {
            return "".to_string();
        } else {
            return self.as_number().to_string();
        };
    }
}

impl Time {
    fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds >= 60 {
            self.minutes += 1;
        }
        if self.minutes >= 60 {
            self.hours += 1;
        }
    }
}

impl Game {
    fn default() -> Self {
        let mut game = Self {
            board: [[Piece {
                scale: 0,
                x: 0,
                y: 0,
            }; 4]; 4],
            score: 0,
            moves: 0,
            time: Time {
                seconds: 0,
                minutes: 0,
                hours: 0,
            },
        };
        for c in 0..game.board.len() {
            for p in 0..game.board[c].len() {
                game.board[c][p].x = c as u8;
                game.board[c][p].y = p as u8;
            }
        }
        return game;
    }
    fn vert_flip(&mut self) {
        let mut new_board: [[Piece; 4]; 4] = Self::default().board;
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                new_board[x][self.board.len() - 1 - y] = self.board[x][y];
                new_board[x][self.board.len() - 1 - y].y = (self.board[x].len() - 1 - y) as u8;
            }
        }
        self.board = new_board;
    }
    fn diag_flip(&mut self) {
        let mut new_board: [[Piece; 4]; 4] = Self::default().board;
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                new_board[x][y] = self.board[y][x];
                new_board[x][y].x = x as u8;
                new_board[x][y].y = y as u8;
            }
        }
        self.board = new_board;
    }

    fn summon(&mut self) {
        let mut rng = thread_rng();

        let x = rng.gen_range(0..4);
        let y = rng.gen_range(0..4);

        if self.board[x][y].scale == 0 {
            self.board[x][y].scale = rng.gen_range(1..=2);
            self.board[x][y].x = x as u8;
            self.board[x][y].y = y as u8;
        } else {
            self.summon();
        };
    }

    fn compressible(&self, x: usize) -> bool {
        for y in self.board[x] {
            if (y.y > 0 && y.scale > 0)
                && (self.board[x][y.y as usize - 1].scale == 0
                    || self.board[x][y.y as usize - 1].scale == y.scale)
            {
                return true;
            }
        }
        return false;
    }

    fn compress(&mut self, x: usize) {
        for y in 0..self.board[x].len() {
            if y > 0 && self.board[x][y - 1].scale == 0 {
                self.board[x][y - 1] = self.board[x][y];
                self.board[x][y - 1].x = x as u8;
                self.board[x][y - 1].y = y as u8 - 1;
                self.board[x][y] = Piece {
                    scale: 0,
                    x: x as u8,
                    y: y as u8,
                };
            } else if y > 0 && self.board[x][y - 1].scale == self.board[x][y].scale {
                self.board[x][y - 1].scale += 1;
                self.board[x][y] = Piece {
                    scale: 0,
                    x: x as u8,
                    y: y as u8,
                };
                self.score += self.board[x][y].as_number();
            }
        }
        if self.compressible(x) {
            self.compress(x);
        }
    }
    fn up(&mut self) {
        let mut has_shifted = false;
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
                has_shifted = true;
            }
        }

        if has_shifted {
            self.summon();
        }
        eprintln!("up")
    }
    fn down(&mut self) {
        self.vert_flip();
        let mut has_shifted = false;
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
                has_shifted = true;
            }
        }
        self.vert_flip();
        if has_shifted {
            self.summon();
        }

        eprintln!("down")
    }
    fn left(&mut self) {
        self.diag_flip();
        let mut has_shifted = false;
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
                has_shifted = true;
            }
        }
        self.diag_flip();

        if has_shifted {
            self.summon();
        }
        eprintln!("left")
    }
    fn right(&mut self) {
        self.diag_flip();
        self.vert_flip();
        let mut has_shifted = false;
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
                has_shifted = true;
            }
        }
        self.vert_flip();
        self.diag_flip();

        if has_shifted {
            self.summon();
        }
        eprintln!("right")
    }
}
