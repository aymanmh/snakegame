use rusty_time::Timer;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_timer::Instant;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hi there {}", name);
    alert(name);
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(name: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
    size: usize,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    death_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
    //death_cell_lifetime: usize,
    death_cell_timer_instant: Instant,
    death_cell_timer: Timer,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, i: usize, snake_size: usize) -> World {
        let size = width * width;
        let snake = Snake::new(i, snake_size);
        let reward_cell = World::gen_reward_cell(&snake.body, None, size);
        let death_cell_timer_instant = Instant::now();
        let death_cell_timer = Timer::default();

        World {
            width,
            size,
            snake,
            next_cell: Option::None,
            reward_cell,
            status: None,
            points: 0,
            death_cell: None,
            death_cell_timer_instant,
            death_cell_timer,
        }
    }

    fn gen_reward_cell(
        snake_body: &Vec<SnakeCell>,
        death_cell: Option<usize>,
        max: usize,
    ) -> Option<usize> {
        let mut reward_cell: usize;
        loop {
            reward_cell = rnd(max);

            if !snake_body.contains(&SnakeCell(reward_cell)) && death_cell != Some(reward_cell) {
                return Some(reward_cell);
            }
        }
    }

    pub fn points(&self) -> usize {
        return self.points;
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn reward_cell(&self) -> Option<usize> {
        return self.reward_cell;
    }

    pub fn death_cell(&self) -> Option<usize> {
        return self.death_cell;
    }

    pub fn snake_head_index(&self) -> usize {
        return self.snake.body[0].0;
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have Won!"),
            Some(GameStatus::Lost) => String::from("You have Lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No Status!"),
        }
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temp_cells = self.snake.body.clone();
                if let Some(cell) = &self.next_cell {
                    self.snake.body[0] = cell.clone();
                    self.next_cell = None;
                } else {
                    self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                }

                for i in 1..self.snake_length() {
                    self.snake.body[i] = SnakeCell(temp_cells[i - 1].0)
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0])
                    || self.death_cell == Some(self.snake_head_index())
                {
                    self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell == Some(self.snake_head_index()) {
                    if self.snake_length() < self.size {
                        self.reward_cell =
                            World::gen_reward_cell(&self.snake.body, self.death_cell, self.size);
                        self.snake.body.push(SnakeCell(self.snake.body[1].0));
                        self.points += 1;
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }
                }

                if self.death_cell == None {
                    //one in a 40 chance
                    let temp = rnd(40);

                    if temp == 1 {
                        let mut death_cell: usize;
                        loop {
                            death_cell = rnd(self.size);

                            if !self.snake.body.contains(&SnakeCell(death_cell))
                                || self.reward_cell.unwrap() == death_cell
                            {
                                self.death_cell = Some(death_cell);
                                let random_seconds: u64 = rnd(2).try_into().unwrap_or(3) + 3;
                                self.death_cell_timer =
                                    Timer::new(Duration::from_secs(random_seconds));
                                self.death_cell_timer_instant = Instant::now();
                                break;
                            }
                        }
                    }
                } else {
                    {
                        self.death_cell_timer
                            .tick(self.death_cell_timer_instant.elapsed());
                        self.death_cell_timer_instant = Instant::now();
                        if self.death_cell_timer.just_finished() {
                            self.death_cell = None;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_index();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>, //where to draw the snake body
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}
