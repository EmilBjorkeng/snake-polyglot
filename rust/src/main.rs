use std::io::{self, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use crossterm::style::{SetForegroundColor, Color, ResetColor};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::ops::Neg;
use rand::RngExt;

const WIDTH: u16 = 20 + 2;
const HEIGHT: u16 = 20 + 2;
const MAX_INPUTS: u16 = 4;
const STARTING_LENGTH: u16 = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Snake {
    body: Vec<Position>,
}

impl Snake {
    fn new() -> Self {
        let head = Position { x: ((WIDTH / 2) - 3) as i32, y: (HEIGHT / 2) as i32 };
        let body = (0..STARTING_LENGTH)
            .map(|i| Position { x: head.x - i as i32, y: head.y })
            .collect();
        Self { body }
    }

    fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }
}

#[derive(Debug, Clone, Copy)]
struct Apple {
    position: Position,
}

impl Apple {
    fn new() -> Self {
        Self { position: Position { x: ((WIDTH / 2) + 3) as i32, y: (HEIGHT / 2) as i32 } }
    }

    fn relocate(&mut self, snake: &Snake) {
        let mut rng = rand::rng();
        let position;
        loop {
            let candidate = Position {
                x: rng.random_range(1..WIDTH as i32 - 1),
                y: rng.random_range(1..HEIGHT as i32 - 1),
            };
            if !snake.body.contains(&candidate) {
                position = candidate;
                break;
            }
        }
        self.position = position
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        Direction { x: -self.x, y: -self.y }
    }
}

fn border_char(x: u16, y: u16) -> Option<&'static str> {
    let (w, h) = (WIDTH - 1, HEIGHT - 1);
    match (x, y) {
        (0, 0)                  => Some(" ╔"),
        (x, 0)   if x == w     => Some("╗ "),
        (0, y)   if y == h     => Some(" ╚"),
        (x, y)   if x == w
                 && y == h     => Some("╝ "),
        (_, 0)                  => Some("══"),
        (_, y)   if y == h     => Some("══"),
        (0, _)                  => Some(" ║"),
        (x, _)   if x == w     => Some("║ "),
        _                       => None,
    }
}

fn draw(stdout: &mut io::Stdout, snake: &Snake, apple: &Apple) -> io::Result<()> {
    execute!(stdout, terminal::Clear(ClearType::All))?;
    // Draw Border
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if let Some(ch) = border_char(x, y) {
                execute!(stdout, cursor::MoveTo(x * 2, y), Print(ch))?;
            }
        }
    }

    // Draw Snake
    for (i, segment) in snake.body.iter().enumerate().rev() {
        let color =
            if i == 0 { Color::Yellow }
            else if i % 2 == 0 { Color::DarkGreen }
            else { Color::Green };
        execute!(
            stdout,
            SetForegroundColor(color),
            cursor::MoveTo((segment.x * 2) as u16, segment.y as u16),
            Print("██")
        )?;
    }

    // Draw Apple
    execute!(stdout, SetForegroundColor(Color::Red),
        cursor::MoveTo((apple.position.x * 2) as u16, apple.position.y as u16), Print("██"))?;

    execute!(stdout, ResetColor)?;
    execute!(
        stdout,
        cursor::MoveTo(0, HEIGHT + 1),
        Print(" WASD to move - Q to quit")
    )?;
    stdout.flush()
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut snake = Snake::new();
    let mut apple = Apple::new();

    let mut input_stack: VecDeque<Direction> = VecDeque::with_capacity(MAX_INPUTS as usize);
    let mut current_direction = Direction { x: 1, y: 0 };

    let valid_x = 1..WIDTH as i32 - 1;
    let valid_y = 1..HEIGHT as i32 - 1;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let direction = match key.code {
                        KeyCode::Char('w') => Some(Direction { x:  0, y: -1 }),
                        KeyCode::Char('s') => Some(Direction { x:  0, y:  1 }),
                        KeyCode::Char('a') => Some(Direction { x: -1, y:  0 }),
                        KeyCode::Char('d') => Some(Direction { x:  1, y:  0 }),
                        KeyCode::Char('q') => break,
                        KeyCode::Esc       => break,
                        _                  => None,
                    };

                    if let Some(dir) = direction {
                        if input_stack.len() < MAX_INPUTS as usize {
                            input_stack.push_back(dir);
                        }
                    }
                }
            }
        }

        // Game update
        if last_tick.elapsed() >= tick_rate {
            // Game logic
            while let Some(dir) = input_stack.pop_front() {
                if dir != -current_direction && dir != current_direction {
                    current_direction = dir;
                    break;
                }
            }

            // Calculate Snake Body
            for i in (1..snake.body.len()).rev() {
                snake.body[i] = snake.body[i - 1];
            }

            // Move snake head
            snake.body[0].x += current_direction.x;
            snake.body[0].y += current_direction.y;

            // Collision
            if !valid_x.contains(&snake.body[0].x) || !valid_y.contains(&snake.body[0].y) {
                break;
            }

            let mut hit_self = false;
            for segment in &snake.body[1..] {
                if snake.body[0] == *segment {
                    hit_self = true;
                    break;
                }
            }
            if hit_self {
                break;
            }

            // Eat apple
            if snake.body[0] == apple.position {
                snake.grow();
                apple.relocate(&snake);
            }

            // Draw
            draw(&mut stdout, &snake, &apple)?;

            last_tick = Instant::now();
        }
    }

    execute!(stdout, cursor::Show)?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    Ok(())
}
