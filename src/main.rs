use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::{Constraint},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph, Padding},
    Frame, DefaultTerminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    board: [i32; 9],
    player: Player,
    position: i32,
    state:State,
    outcome:Outcome,
}



#[derive(PartialEq, Eq)]
enum Player {
    Cross,
    Circle,
}

#[derive(PartialEq, Eq)]
enum State {
    Start,
    Game,
    Finish,
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Circle,
    Cross,
    Draw,
}

impl App {
    const fn new() -> Self {
        Self {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            player: Player::Circle,
            position: 0,
            state:State::Start,
            outcome:Outcome::Draw,
        }
    }

    fn move_cursor_left(&mut self) {
        if self.position > 0{
            self.position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.position < 8{
            self.position += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.position > 2{
            self.position -= 3;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.position < 6{
            self.position += 3;
        }
    }

    fn reset(&mut self) {
        self.outcome = Outcome::Draw;
        self.board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    fn submit_message(&mut self) {
        if self.player == Player::Circle{
            if self.board[self.position as usize] == 0{
                self.board[self.position as usize] = 1;
                self.player = Player::Cross;
            }
        }
        else if self.player == Player::Cross{
            if self.board[self.position as usize] == 0{
                self.board[self.position as usize] = 4;
                self.player = Player::Circle;
            }
        }
        if self.check_win(){
            self.state = State::Finish;
        }
    }

    fn check_win(&mut self) -> bool{
        for i in 0..3{
            let row = self.board[i*3] + self.board[i*3 + 1] + self.board[i*3 + 2];
            let col = self.board[i] + self.board[i + 3] + self.board[i + 6];
            if col == 3 || row == 3{
                self.outcome = Outcome::Cross;
                return true;
            }
            if col == 12 || row == 12{
                self.outcome = Outcome::Circle;
                return true;
            }
        }
        let diag1 = self.board[0] + self.board[4] + self.board[8];
        let diag2 = self.board[2] + self.board[4] + self.board[6];
        if diag1 == 3 || diag2 == 3{
            self.outcome = Outcome::Cross;
            return true;
        }
        if diag1 == 12 || diag2 == 12{
            self.outcome = Outcome::Circle;
            return true;
        }
        let mut draw : bool = true;
        for i in 0..8{
            if self.board[i] == 0{
                draw = false;
                break;
            }
        }
        return draw;
    }


    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                if key.code == KeyCode::Esc{
                    return Ok(());
                }
                if self.state == State::Game{
                    if key.code == KeyCode::Enter{
                        self.submit_message();
                    }
                    if key.code == KeyCode::Left{
                        self.move_cursor_left();
                    }
                    if key.code == KeyCode::Right{
                        self.move_cursor_right();
                    }
                    if key.code == KeyCode::Up{
                        self.move_cursor_up();
                    }
                    if key.code == KeyCode::Down{
                        self.move_cursor_down();
                    }
                    if key.code == KeyCode::Backspace{
                        self.reset();
                    }
                }else if self.state == State::Start{
                    if key.code == KeyCode::Enter || key.code == KeyCode::Backspace{
                        self.state = State::Game;
                    }
                }else if self.state == State::Finish{
                    if key.code == KeyCode::Enter || key.code == KeyCode::Backspace{
                        self.state = State::Start;
                        self.reset();
                    }
                }
            }
        }
    }

    pub fn linegen(&self, state1: i32, state2: i32, state3: i32, line: i32, lines: &mut Vec<String>){
        let mut txtline: String = "  |".to_string();
        let empty: &str = "             ";
        let pipe: &str = "|";

        let cross: [&str; 4] = ["    ____     ", "   _|  |_    ", "  |_    _|   ", "    |__|     "];
        let circle: [&str; 4] = ["    ____     ", "  ,'    `.   ", "  |      |   ", "  `.____,'   "];

        for i in 0..3 {
            if state1 == 1 && i == 0 || state2 == 1 && i == 1 || state3 == 1 && i == 2{
                txtline.push_str(cross[line as usize]);
            }else if state1 == 4 && i == 0 || state2 == 4 && i == 1 || state3 == 4 && i == 2{
                txtline.push_str(circle[line as usize]);
            }else{
                txtline.push_str(empty);
            };
            txtline.push_str(pipe);
        }
        lines.push(txtline);
    }

    fn render(&self, frame: &mut Frame) {
        let main_block= Block::default().title("Tic Tac Tui").borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Thick).border_style(Style::default().fg(Color::White));
        frame.render_widget(main_block, frame.area());

        if self.state == State::Game{
            let text_area = frame.area()
                    .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2));


            let mut lines: Vec<String> = vec![];
            for j in 0..3{
                if self.position == 0 + (j*3) as i32{
                    lines.push("  -------▼-----------------------------------".to_string());
                }else if self.position == 1 + (j*3) as i32{
                    lines.push("  --------------------▼----------------------".to_string());
                }else if self.position == 2 + (j*3) as i32{
                    lines.push("  ----------------------------------▼--------".to_string());
                }else{
                    lines.push("  -------------------------------------------".to_string());
                };
                for i in 0..4 {
                    self.linegen(self.board[j*3 as usize], self.board[j*3+1 as usize], self.board[j*3+2 as usize], i, &mut lines);
                }
            }
            lines.push("  -------------------------------------------".to_string());
            let list = List::new(lines);

            frame.render_widget(list, text_area);
        }else if self.state == State::Start{
            let mut lines: Vec<String> = vec![];
            lines.push("          _   _        _              _       _".to_string());
            lines.push("         | |_|_|___   | |_ ___ ___   | |_ _ _|_|".to_string());
            lines.push("         |  _| |  _|  |  _| .'|  _|  |  _| | | |".to_string());
            lines.push("         |_| |_|___|  |_| |__,|___|  |_| |___|_|".to_string());
            lines.push("".to_string());
            lines.push("".to_string());
            lines.push("                Press Enter to start the game                  ".to_string());
            for _i in 0..6{
                lines.push("".to_string());
            }
            lines.push("      arrow to move/enter to place/backspace to restart".to_string());
            lines.push("                  escape to stop the program            ".to_string());
            // lines.push("".to_string());
            // lines.push("".to_string());
            // lines.push("                       Made by FlowTy                ".to_string());
            let list = List::new(lines);
            let text_area = frame.area()
                    .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2));
            frame.render_widget(list, text_area);
        }else if self.state == State::Finish{
            let mut lines: Vec<String> = vec![];
            if self.outcome == Outcome::Cross{
                lines.push("                                       _    ".to_string());
                lines.push("          ___ ___ ___ ___ ___    _ _ _|_|___ ".to_string());
                lines.push("         |  _|  _| . |_ -|_ -|  | | | | |   |".to_string());
                lines.push("         |___|_| |___|___|___|  |_____|_|_|_|".to_string());
            }else if self.outcome == Outcome::Circle{
                lines.push("              _         _              _    ".to_string());
                lines.push("          ___|_|___ ___| |___    _ _ _|_|___ ".to_string());
                lines.push("         |  _| |  _|  _| | -_|  | | | | |   |".to_string());
                lines.push("         |___|_|_| |___|_|___|  |_____|_|_|_|".to_string());
            }else{
                lines.push("                     _               ".to_string());
                lines.push("                   _| |___ ___ _ _ _ ".to_string());
                lines.push("                  | . |  _| .'| | | |".to_string());
                lines.push("                  |___|_| |__,|_____|".to_string());
            }
            
            lines.push("".to_string());
            lines.push("".to_string());
            lines.push("                 Press Enter to continue                 ".to_string());
            let list = List::new(lines);
            let text_area = frame.area()
                    .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2));
            frame.render_widget(list, text_area);
        }
        
        // let p = Paragraph::new("Made By FlowTy")
        //     .block(Block::new().style(Style::new().bg(Color::Black)).padding(Padding::new(
        //     frame.area().width - 15,
        //     0,
        //     0,
        //     0,
        //     )))
        //     .style(Style::new().white());
        // frame.render_widget(p, frame.area());
    }

}