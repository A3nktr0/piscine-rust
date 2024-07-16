extern crate sdl2;

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod obj;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("road_intersection", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let mut road = obj::road::Road::new(800, 800);
    road.draw(&mut canvas);

    canvas.present();

    let mut last_add_times: HashMap<Keycode, Instant> = HashMap::new();
    last_add_times.insert(Keycode::Up, Instant::now());
    last_add_times.insert(Keycode::Down, Instant::now());
    last_add_times.insert(Keycode::Left, Instant::now());
    last_add_times.insert(Keycode::Right, Instant::now());
    last_add_times.insert(Keycode::R, Instant::now());
    let mut pressed_keys: HashSet<Keycode> = HashSet::new();
    let rate_limit = Duration::from_millis(500);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if !pressed_keys.contains(&keycode) => {
                    pressed_keys.insert(keycode);
                    let now = Instant::now();
                    let last_add_time = last_add_times.entry(keycode).or_insert_with(Instant::now);

                    if now.duration_since(*last_add_time) >= rate_limit {
                        match keycode {
                            Keycode::Space => {
                                road.change_traffic_lights_manual();
                            }
                            _ => {}
                        }
                        road.add_vehicle(keycode);
                        *last_add_time = now;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    pressed_keys.remove(&keycode);
                }
                _ => {}
            }
        }

        road.change_traffic_lights();

        road.update_state(&mut canvas);
        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
