use super::road::{self, Lane};
use rand::Rng;

use Direction::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;
// const SCREEN_HEIGHT: i32 = 600;
const TURN_RIGHT_OFFSET: i32 = 15;
const TURN_LEFT_OFFSET_NORTH_SOUTH: i32 = 65;
const TURN_LEFT_OFFSET_EAST_SOUTH: i32 = 35;
const VELOCITY: i32 = 1; //1 for 120Hz change it to 5 for 60Hz or unppluged computer
const SAFE_DISTANCE: i32 = 20;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub lane: Lane,
    pub color: sdl2::pixels::Color,
    pub direction: Direction,
    pub hitbox: HitBox,
    pub state: State,
    pub collision: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HitBox {
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub shape: sdl2::rect::Rect,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    TurnRight,
    TurnLeft,
    GoStraight,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum State {
    Moving,
    Stopped,
}

impl Vehicle {
    pub fn new(lane: Lane) -> Vehicle {
        let (x, y, w, h) = match lane {
            Lane::North => (365, 0, 20, 20),
            Lane::South => (415, 780, 20, 20),
            Lane::East => (780, 365, 20, 20),
            Lane::West => (0, 415, 20, 20),
            // Lane::North => (380, 0, 20, 20),
            // Lane::South => (400, 580, 20, 20),
            // Lane::East => (780, 280, 20, 20),
            // Lane::West => (0, 300, 20, 20),
        };

        let (color, direction) = match rand::thread_rng().gen_range(0..=2) {
            0 => (sdl2::pixels::Color::RGB(255, 100, 0), Direction::TurnRight),
            1 => (sdl2::pixels::Color::RGB(200, 0, 250), Direction::TurnLeft),
            _ => (sdl2::pixels::Color::RGB(0, 200, 255), Direction::GoStraight),
        };

        let hitbox = match lane {
            Lane::North => HitBox {
                x_offset: 0,
                y_offset: -SAFE_DISTANCE,
                width: w,
                height: h + SAFE_DISTANCE,
                shape: sdl2::rect::Rect::new(
                    x,
                    y - SAFE_DISTANCE,
                    w as u32,
                    (h + SAFE_DISTANCE) as u32,
                ),
            },
            Lane::South => HitBox {
                x_offset: 0,
                y_offset: 0,
                width: w,
                height: h + SAFE_DISTANCE,
                shape: sdl2::rect::Rect::new(x, y, w as u32, (h + SAFE_DISTANCE) as u32),
            },
            Lane::East => HitBox {
                x_offset: 0,
                y_offset: 0,
                width: w + SAFE_DISTANCE,
                height: h,
                shape: sdl2::rect::Rect::new(x, y, (w + SAFE_DISTANCE) as u32, h as u32),
            },
            Lane::West => HitBox {
                x_offset: -SAFE_DISTANCE,
                y_offset: 0,
                width: w + SAFE_DISTANCE,
                height: h,
                shape: sdl2::rect::Rect::new(
                    x - SAFE_DISTANCE,
                    y,
                    (w + SAFE_DISTANCE) as u32,
                    h as u32,
                ),
            },
        };

        Vehicle {
            x,
            y,
            width: 20,
            height: 20,
            lane,
            color,
            direction,
            hitbox,
            state: State::Moving,
            collision: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
            ))
            .unwrap();

        // canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // canvas.draw_rect(self.hitbox.shape).unwrap();
    }

    pub fn move_vehicle(&mut self, intersection: &road::Intersection) {
        if self.state == State::Stopped {
            return;
        }

        match self.lane {
            Lane::North => {
                if self.is_in_turn_zone(&intersection) {
                    match self.direction {
                        Direction::TurnRight => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 40;
                            self.hitbox.height = 20;
                            if self.x >= 0 {
                                self.x -= VELOCITY;
                            }
                        }
                        Direction::TurnLeft => {
                            self.hitbox.x_offset = -20;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 40;
                            self.hitbox.height = 20;
                            if self.x <= SCREEN_WIDTH {
                                self.x += VELOCITY;
                            }
                        }
                        Direction::GoStraight => {
                            if self.y <= SCREEN_HEIGHT {
                                self.y += VELOCITY;
                            }
                        }
                    }
                } else if self.y <= SCREEN_HEIGHT {
                    self.y += VELOCITY;
                }
            }
            Lane::South => {
                if self.is_in_turn_zone(&intersection) {
                    match self.direction {
                        Direction::TurnRight => {
                            self.hitbox.x_offset = -20;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 40;
                            self.hitbox.height = 20;
                            if self.x <= SCREEN_WIDTH {
                                self.x += VELOCITY;
                            }
                        }
                        Direction::TurnLeft => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 40;
                            self.hitbox.height = 20;
                            if self.x >= 0 {
                                self.x -= VELOCITY;
                            }
                        }
                        Direction::GoStraight => {
                            if self.y >= 0 {
                                self.y -= VELOCITY;
                            }
                        }
                    }
                } else if self.y >= 0 {
                    self.y -= VELOCITY;
                }
            }
            Lane::East => {
                if self.is_in_turn_zone(&intersection) {
                    match self.direction {
                        Direction::TurnRight => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 20;
                            self.hitbox.height = 40;
                            if self.y >= 0 {
                                self.y -= VELOCITY;
                            }
                        }
                        Direction::TurnLeft => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = -20;
                            self.hitbox.width = 20;
                            self.hitbox.height = 40;
                            if self.y <= SCREEN_HEIGHT {
                                self.y += VELOCITY;
                            }
                        }
                        Direction::GoStraight => {
                            if self.x >= 0 {
                                self.x -= VELOCITY;
                            }
                        }
                    }
                } else if self.x >= 0 {
                    self.x -= VELOCITY;
                }
            }
            Lane::West => {
                if self.is_in_turn_zone(&intersection) {
                    match self.direction {
                        Direction::TurnRight => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = -20;
                            self.hitbox.width = 20;
                            self.hitbox.height = 40;
                            if self.y <= SCREEN_HEIGHT {
                                self.y += VELOCITY;
                            }
                        }
                        Direction::TurnLeft => {
                            self.hitbox.x_offset = 0;
                            self.hitbox.y_offset = 0;
                            self.hitbox.width = 20;
                            self.hitbox.height = 40;
                            if self.y >= 0 {
                                self.y -= VELOCITY;
                            }
                        }
                        Direction::GoStraight => {
                            if self.x <= SCREEN_WIDTH {
                                self.x += VELOCITY;
                            }
                        }
                    }
                } else if self.x <= SCREEN_WIDTH {
                    self.x += VELOCITY;
                }
            }
        }

        self.hitbox.shape = sdl2::rect::Rect::new(
            self.x + self.hitbox.x_offset,
            self.y + self.hitbox.y_offset,
            self.hitbox.width as u32,
            self.hitbox.height as u32,
        );
    }

    fn is_in_turn_zone(&self, intersection: &road::Intersection) -> bool {
        let i_x = intersection.x;
        let i_y = intersection.y;
        let i_w = intersection.width as i32;
        let i_h = intersection.height as i32;

        match self.lane {
            road::Lane::North => match &self.direction {
                TurnRight => self.y >= i_y + TURN_RIGHT_OFFSET,
                GoStraight => self.y >= i_y + i_h,
                TurnLeft => self.y >= i_y + TURN_LEFT_OFFSET_NORTH_SOUTH,
            },
            road::Lane::South => match &self.direction {
                TurnRight => self.y + self.height <= i_y + i_h - TURN_RIGHT_OFFSET,
                GoStraight => self.y + self.height <= i_y + i_h,
                TurnLeft => self.y + self.height <= i_y + TURN_LEFT_OFFSET_EAST_SOUTH,
            },
            road::Lane::West => match &self.direction {
                TurnRight => self.x >= i_x + TURN_RIGHT_OFFSET,
                GoStraight => self.x >= i_x + i_w,
                TurnLeft => self.x >= i_x + TURN_LEFT_OFFSET_NORTH_SOUTH,
            },
            road::Lane::East => match &self.direction {
                TurnRight => self.x + self.width <= i_x + i_w - TURN_RIGHT_OFFSET,
                GoStraight => self.x + self.width <= i_x + i_w,
                TurnLeft => self.x + self.width <= i_x + TURN_LEFT_OFFSET_EAST_SOUTH,
            },
        }
    }

    pub fn check_collision(&self, other: &Vehicle) -> bool {
        self.hitbox.shape.has_intersection(other.hitbox.shape)
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped;
    }

    pub fn go(&mut self) {
        self.state = State::Moving;
    }

    pub fn is_out_of_bounds(&self) -> bool {
        match self.lane {
            Lane::North => self.y >= SCREEN_HEIGHT || self.x <= 0 || self.x >= SCREEN_WIDTH,
            Lane::South => self.y <= 0 || self.x <= 0 || self.x >= SCREEN_WIDTH,
            Lane::East => self.x <= 0 || self.y <= 0 || self.y >= SCREEN_HEIGHT,
            Lane::West => self.x >= SCREEN_WIDTH || self.y <= 0 || self.y >= SCREEN_HEIGHT,
        }
    }
}
