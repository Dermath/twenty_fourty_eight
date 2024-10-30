use iced::keyboard as k;
use iced::widget::{button, column, container, row, text};
use iced::{time, Element, Length, Pixels, Theme};
use rand::thread_rng;
use rand::Rng;

const ICON: &[u8] = include_bytes!("2048.png");

const FONT_SIZE: f32 = 70.0;

const BUTTON_PADDING: f32 = 5.0;
const UI_PADDING: u16 = 20;

const WIN_SCALE: f32 = 11.0;

fn main() -> iced::Result {
    let icon = match iced::window::icon::from_file_data(ICON, None) {
        Ok(inner) => inner,
        Err(inner) => panic!(
            "the window icon appears to be missing with error code: {}",
            inner
        ),
    };
    let window_settings = iced::window::Settings {
        icon: Some(icon),
        ..iced::window::Settings::default()
    };
    iced::application("2048", Game::update, Game::view)
        .subscription(Game::subscription)
        .theme(|_| Theme::SolarizedDark)
        .antialiasing(true)
        .window(window_settings)
        .run()
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
    let mut r = piece.scale as f32 / WIN_SCALE;
    if r > 1.0 {
        r = 1.0;
    }
    let b = 1.0 - r;
    let press_action = match piece.scale {
        0 => Message::Upgrade(piece),
        _ => Message::None,
    };
    return button(
        text(piece.as_string())
            .center()
            .width(Length::Fill)
            .size(Pixels(FONT_SIZE))
            .color([r, 0.0, b]),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press(press_action);
}

fn ui_button(button_text: String, message: Message) -> button::Button<'static, Message> {
    button(
        text(button_text)
            .center()
            .width(iced::Length::Fill)
            .size(Pixels(FONT_SIZE / 2.0)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press(message)
}

impl Game {
    fn update(&mut self, message: Message) {
        match self.mode {
            Mode::Normal => match message {
                Message::Tick => self.time.tick(),
                Message::Up => self.up(),
                Message::Down => self.down(),
                Message::Left => self.left(),
                Message::Right => self.right(),
                Message::Restart => self.restart(),
                Message::ModeSwitch => self.switch_mode(),
                Message::Quit => std::process::exit(0),
                _ => (),
            },
            Mode::Reverse => match message {
                Message::Tick => self.time.tick(),
                Message::Upgrade(piece) => self.upgrade(&piece),
                Message::Restart => self.restart(),
                Message::ModeSwitch => self.switch_mode(),
                Message::Quit => std::process::exit(0),
                _ => (),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let board = row![
            column![
                piece_button(self.board[0][0]),
                piece_button(self.board[0][1]),
                piece_button(self.board[0][2]),
                piece_button(self.board[0][3]),
            ]
            .width(Length::FillPortion(1))
            .padding(BUTTON_PADDING)
            .spacing(2.0 * BUTTON_PADDING),
            column![
                piece_button(self.board[1][0]),
                piece_button(self.board[1][1]),
                piece_button(self.board[1][2]),
                piece_button(self.board[1][3]),
            ]
            .width(Length::FillPortion(1))
            .padding(BUTTON_PADDING)
            .spacing(2.0 * BUTTON_PADDING),
            column![
                piece_button(self.board[2][0]),
                piece_button(self.board[2][1]),
                piece_button(self.board[2][2]),
                piece_button(self.board[2][3]),
            ]
            .width(Length::FillPortion(1))
            .padding(BUTTON_PADDING)
            .spacing(2.0 * BUTTON_PADDING),
            column![
                piece_button(self.board[3][0]),
                piece_button(self.board[3][1]),
                piece_button(self.board[3][2]),
                piece_button(self.board[3][3]),
            ]
            .width(Length::FillPortion(1))
            .padding(BUTTON_PADDING)
            .spacing(2.0 * BUTTON_PADDING),
            column![
                text("Time: ".to_string() + self.time.current().as_ref()).size(0.7 * FONT_SIZE),
                text("Score: ".to_string() + self.score.to_string().as_ref()).size(0.7 * FONT_SIZE),
                ui_button("Restart".to_string(), Message::Restart),
                ui_button(
                    format!("Mode: {}", self.mode.to_string()),
                    Message::ModeSwitch,
                ),
                ui_button("Quit".to_string(), Message::Quit),
            ]
            .width(Length::FillPortion(3))
            .padding(UI_PADDING)
            .spacing(UI_PADDING)
        ];

        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::time::every(time::Duration::from_millis(1000)).map(|_| Message::Tick),
            k::on_key_press(input).into(),
        ])
    }
}

fn input(key: k::Key, _module: k::Modifiers) -> Option<Message> {
    return Some(match key.as_ref() {
        k::Key::Named(inner) => match inner {
            k::key::Named::ArrowLeft => Message::Left,
            k::key::Named::ArrowUp => Message::Up,
            k::key::Named::ArrowDown => Message::Down,
            k::key::Named::ArrowRight => Message::Right,
            _ => Message::None,
        },
        k::Key::Character(inner) => match inner {
            "h" => Message::Left,
            "a" => Message::Left,
            "k" => Message::Up,
            "w" => Message::Up,
            "j" => Message::Down,
            "s" => Message::Down,
            "l" => Message::Right,
            "d" => Message::Right,
            _ => Message::None,
        },
        _ => Message::None,
    });
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Upgrade(Piece),
    Up,
    Down,
    Left,
    Right,
    Restart,
    Quit,
    ModeSwitch,
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Piece {
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
enum Mode {
    Normal,
    Reverse,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    mode: Mode,
    board: [[Piece; 4]; 4], // [x][y]
    score: u32,
    moves: u32,
    time: Time,
}

impl Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Normal => "Normal".to_string(),
            Mode::Reverse => "Reverse".to_string(),
        }
    }
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
            self.seconds = 0;
        }
        if self.minutes >= 60 {
            self.hours += 1;
            self.minutes = 0;
            self.seconds = 0;
        }
    }
    fn current(&self) -> String {
        return format!("{}:{}:{}", self.hours, self.minutes, self.seconds);
    }
}
impl std::default::Default for Game {
    fn default() -> Self {
        let mut game = Self {
            mode: Mode::Normal,
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
        game.summon();
        return game;
    }
}
impl Game {
    fn reverse_default() -> Self {
        let mut game = Self {
            mode: Mode::Reverse,
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
        game.summon();
        return game;
    }
    fn move_random(&mut self) {
        let origenal_board = self.board.clone();
        let mut rng = thread_rng();
        let direction: u8 = rng.gen_range(1..=4);
        match direction {
            1 => self.auto_up(),
            2 => self.auto_right(),
            3 => self.auto_down(),
            4 => self.auto_left(),
            _ => panic!("The direction selected was not aplicable. "),
        }

        let mut compressible = false;
        for x in 0..self.board.len() {
            if self.compressible(x) {
                compressible = true;
            }
        }
        self.diag_flip();
        for y in 0..self.board.len() {
            if self.compressible(y) {
                compressible = true;
            }
        }
        self.diag_flip();
        if self.board == origenal_board && compressible {
            self.move_random();
        }
    }
    fn switch_mode(&mut self) {
        let new: Game;
        match self.mode {
            Mode::Normal => {
                new = Self::reverse_default();
            }
            Mode::Reverse => {
                new = Self::default();
            }
        }
        *self = new;
    }
    fn restart(&mut self) {
        let new = Game::default();
        self.board = new.board;
        self.score = new.score;
        self.moves = new.moves;
        self.time = new.time;
    }
    fn upgrade(&mut self, piece: &Piece) {
        self.board[piece.x as usize][piece.y as usize].scale += 1;
        self.move_random();
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
                self.score += 2 * self.board[x][y].as_number();

                self.board[x][y - 1].scale += 1;
                self.board[x][y] = Piece {
                    scale: 0,
                    x: x as u8,
                    y: y as u8,
                };
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
        // eprintln!("up")
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

        // eprintln!("down")
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
        // eprintln!("left")
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
        // eprintln!("right")
    }
    fn auto_up(&mut self) {
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
            }
        }
        // eprintln!("up")
    }
    fn auto_down(&mut self) {
        self.vert_flip();
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
            }
        }
        self.vert_flip();
        // eprintln!("down")
    }
    fn auto_left(&mut self) {
        self.diag_flip();
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
            }
        }
        self.diag_flip();
        // eprintln!("left")
    }
    fn auto_right(&mut self) {
        self.diag_flip();
        self.vert_flip();
        for x in 0..self.board.len() {
            if self.compressible(x) {
                self.compress(x);
            }
        }
        self.vert_flip();
        self.diag_flip();
        // eprintln!("right")
    }
}
