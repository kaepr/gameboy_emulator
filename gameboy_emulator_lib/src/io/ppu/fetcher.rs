use std::collections::VecDeque;

use super::registers::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pixel {
    color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel { color: Color::C0 }
    }
}

impl Pixel {
    pub fn new(color: Color) -> Self {
        Pixel { color }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn is_lightest_color(&self) -> bool {
        self.color == Color::C0
    }
}

pub enum FetcherState {
    GetTile,
    GetDataLow,
    GetDataHigh,
    Sleep,
    Push,
}

pub struct Fetcher {
    queue: VecDeque<Pixel>,
    pub state: FetcherState,
    pub pushed: usize,
    pub ticks: u64,
    x_coor: usize,
    y_coor: usize,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            queue: VecDeque::new(),
            state: FetcherState::GetTile,
            ticks: 0,
            pushed: 0,
            x_coor: 0,
            y_coor: 0,
        }
    }

    pub fn process(&mut self) {
        match self.state {
            FetcherState::GetTile => todo!(),
            FetcherState::GetDataLow => todo!(),
            FetcherState::GetDataHigh => todo!(),
            FetcherState::Sleep => todo!(),
            FetcherState::Push => todo!(),
        }
    }

    pub fn push_pixel(&mut self) {}

    pub fn reset(&mut self) {
        self.queue.clear();
        self.state = FetcherState::GetTile;
        self.ticks = 0;
        self.pushed = 0;
    }

    pub fn set_state(&mut self, state: FetcherState) {
        self.state = state;
    }

    pub fn push(&mut self, pixel: Pixel) {
        self.queue.push_back(pixel);
    }

    pub fn pop(&mut self) -> Pixel {
        match self.queue.pop_front() {
            Some(pixel) => {
                self.pushed += 1;
                pixel
            }
            None => panic!("Popping from empty fifo"),
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }
}
