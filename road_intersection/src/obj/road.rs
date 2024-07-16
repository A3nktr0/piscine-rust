use rand::Rng;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};
use std::collections::HashMap;

use super::{
    traffic_light::{self, TrafficLight},
    vehicle::{self, Vehicle},
};

pub struct Road {
    width: i32,
    height: i32,
    lanes: HashMap<Lane, Vec<Vehicle>>,
    crossing: Vec<Vehicle>,
    exiting: Vec<Vehicle>,
    traffic_lights: Vec<TrafficLight>,
    intersection: Intersection,
}

#[derive(Clone, Copy)]
pub struct Intersection {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub shape: Rect,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Lane {
    North,
    East,
    South,
    West,
}

impl Road {
    pub fn new(width: i32, height: i32) -> Road {
        Road {
            width,
            height,
            lanes: HashMap::new(),
            crossing: Vec::new(),
            exiting: Vec::new(),
            traffic_lights: vec![
                // TrafficLight::new(330, 230, Lane::North),
                // TrafficLight::new(450, 350, Lane::South),
                // TrafficLight::new(450, 230, Lane::East),
                // TrafficLight::new(330, 350, Lane::West),
                TrafficLight::new(330, 330, Lane::North),
                TrafficLight::new(450, 450, Lane::South),
                TrafficLight::new(450, 330, Lane::East),
                TrafficLight::new(330, 450, Lane::West),
            ],
            intersection: Intersection {
                x: width / 2 - 50,
                y: height / 2 - 50,
                width: 100,
                height: 100,
                // shape: Rect::new(width / 2 - 20, height / 2 - 20, 40, 40),
                shape: Rect::new(width / 2 - 50, height / 2 - 50, 100, 100),
            },
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(50, 180, 50));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(0, 350, 800, 100)).unwrap();
        canvas.fill_rect(Rect::new(350, 0, 100, 800)).unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas
            .draw_line(
                sdl2::rect::Point::new(0, self.height / 2),
                sdl2::rect::Point::new(self.width, self.height / 2),
            )
            .unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas
            .draw_line(
                sdl2::rect::Point::new(self.width / 2, 0),
                sdl2::rect::Point::new(self.width / 2, self.height),
            )
            .unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        // canvas.draw_rect(self.intersection.shape).unwrap();
        canvas.fill_rect(self.intersection.shape).unwrap();

        for traffic_light in self.traffic_lights.iter() {
            traffic_light.draw(canvas);
        }
    }

    pub fn add_vehicle(&mut self, keycode: Keycode) {
        let from = match keycode {
            Keycode::Up => Lane::South,
            Keycode::Down => Lane::North,
            Keycode::Left => Lane::East,
            Keycode::Right => Lane::West,
            Keycode::R => match rand::thread_rng().gen_range(0..=3) {
                0 => Lane::North,
                1 => Lane::South,
                2 => Lane::West,
                3 => Lane::East,
                _ => panic!("Invalid starting direction"),
            },
            _ => return,
        };

        if let Some(vehicles) = self.lanes.get_mut(&from) {
            if vehicles.len() < 8 {
                let vehicle = vehicle::Vehicle::new(from);
                vehicles.push(vehicle);
            }
        } else {
            let vehicle = vehicle::Vehicle::new(from);
            self.lanes.insert(from, vec![vehicle]);
        }
    }

    pub fn update_state(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.update_entering(canvas);
        self.update_crossing(canvas);
        self.update_exiting(canvas);

        // println!(
        //     "Entering: {}, Crossing: {}, Exiting: {}",
        //     self.lanes
        //         .values()
        //         .map(|vehicles| vehicles.len())
        //         .sum::<usize>(),
        //     self.crossing.len(),
        //     self.exiting.len()
        // );

        canvas.present();
    }

    pub fn update_entering(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.clear();
        self.draw(canvas);

        for (_, vehicles) in self.lanes.iter_mut() {
            collision_checker(vehicles);

            let mut to_cross = Vec::new();

            for (i, vehicle) in vehicles.iter_mut().enumerate() {
                if self.intersection.vehicle_is_at_intersection(vehicle) {
                    if self.traffic_lights.iter().any(|light| {
                        light.lane == vehicle.lane && light.color == traffic_light::State::Red
                    }) {
                        vehicle.stop();
                    } else {
                        to_cross.push(i);
                        vehicle.go();
                    }
                }
                if !vehicle.collision {
                    vehicle.move_vehicle(&self.intersection);
                }
                vehicle.draw(canvas);
            }

            collision_checker(vehicles);

            for &index in to_cross.iter().rev() {
                let vehicle = vehicles.remove(index);
                self.crossing.push(vehicle);
            }
        }
    }

    fn update_crossing(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let mut to_exit = Vec::new();

        for (i, vehicle) in self.crossing.iter_mut().enumerate() {
            if self.intersection.vehicle_exit_intersection(vehicle) {
                to_exit.push(i);
            }
            vehicle.move_vehicle(&self.intersection);
            vehicle.draw(canvas);
        }

        collision_checker(&mut self.crossing);

        for &index in to_exit.iter().rev() {
            let vehicle = self.crossing.remove(index);
            self.exiting.push(vehicle);
        }
    }

    fn update_exiting(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let mut to_remove = Vec::new();

        for (i, vehicle) in self.exiting.iter_mut().enumerate() {
            if vehicle.is_out_of_bounds() {
                to_remove.push(i);
            }
            vehicle.move_vehicle(&self.intersection);
            vehicle.draw(canvas);
        }

        collision_checker(&mut self.exiting);

        for &index in to_remove.iter().rev() {
            self.exiting.remove(index);
        }
    }

    pub fn change_traffic_lights_manual(&mut self) {
        for traffic_light in &mut self.traffic_lights {
            if rand::random() {
                traffic_light.change_state();
            }
        }
    }

    fn get_longest_lane(&self) -> Lane {
        self.lanes
            .iter()
            .max_by_key(|(_, vehicles)| vehicles.len())
            .map(|(lane, _)| *lane)
            .unwrap_or(Lane::North)
    }

    pub fn change_traffic_lights(&mut self) {
        let longest_lane = self.get_longest_lane();

        for traffic_light in &mut self.traffic_lights {
            if self.crossing.is_empty() {
                traffic_light.set_state(
                    if self.lanes.values().all(|vehicles| vehicles.is_empty()) {
                        traffic_light::State::Red
                    } else if traffic_light.lane == longest_lane {
                        traffic_light::State::Green
                    } else {
                        traffic_light::State::Red
                    },
                );
            }
        }
    }
}

impl Intersection {
    fn vehicle_is_at_intersection(&self, vehicle: &Vehicle) -> bool {
        match vehicle.lane {
            Lane::North => vehicle.y == self.y - vehicle.height,
            Lane::South => vehicle.y == self.y + self.height as i32,
            Lane::East => vehicle.x == self.x + self.width as i32,
            Lane::West => vehicle.x == self.x - vehicle.width,
        }
    }

    fn vehicle_exit_intersection(&self, vehicle: &Vehicle) -> bool {
        vehicle.x > self.x + self.width as i32
            || vehicle.x < self.x - vehicle.width
            || vehicle.y > self.y + self.height as i32
            || vehicle.y < self.y - vehicle.height
    }
}

fn collision_checker(vehicles: &mut Vec<Vehicle>) {
    for i in 0..vehicles.len() {
        for j in (i + 1)..vehicles.len() {
            if vehicles[i].check_collision(&vehicles[j]) {
                vehicles[j].collision = true;
            } else {
                vehicles[j].collision = false;
            }
        }
    }
}
