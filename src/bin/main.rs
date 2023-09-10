use std::collections::VecDeque;
use std::time::Duration;

use evolution::graphics::{self, Camera};
use evolution::world::EntityData;
use evolution::{
    world::{Location, Vector},
    State,
};
use macroquad::prelude::{self as mq};
use macroquad::text::{self, TextParams};
use macroquad::{
    camera,
    color::{colors, Color},
    window::clear_background,
};
use nalgebra::Vector4;

const SPEEDUP: Option<f64> = None;
const FRAME_RATE: f64 = 60.;

fn draw_info(state: &State, tps: usize) {
    let num_creatures = state
        .entities()
        .filter(|entity| entity.is_creature())
        .count();

    text::draw_text_ex(
        &format!("Creatures: {num_creatures}"),
        state.config().world_width() + 1.,
        3.,
        TextParams {
            font_size: 16,
            font_scale: 1. / 4.,
            ..Default::default()
        },
    );
    let avg_max_acceleration = state
        .entities()
        .filter_map(|entity| match entity.entity_data() {
            EntityData::Creature(creature) => Some(creature.max_acceleration()),
            _ => None,
        })
        .sum::<f32>()
        / num_creatures as f32;
    text::draw_text_ex(
        &format!("{avg_max_acceleration:.2}"),
        state.config().world_width() + 1.,
        7.,
        TextParams {
            font_size: 16,
            font_scale: 1. / 4.,
            ..Default::default()
        },
    );
    text::draw_text_ex(
        "Max acc. | Energy",
        state.config().world_width() + 1.,
        11.,
        TextParams {
            font_size: 16,
            font_scale: 1. / 4.,
            ..Default::default()
        },
    );
    for (index, creature) in state
        .entities()
        .filter_map(|entity| match entity.entity_data() {
            EntityData::Creature(creature) => Some(creature),
            _ => None,
        })
        .enumerate()
    {
        let energy = creature.energy();
        let max_acceleration = creature.max_acceleration();
        text::draw_text_ex(
            &format!("{max_acceleration: >8.2} | {energy: >6.2}"),
            state.config().world_width() + 1.,
            15. + (index * 4) as f32,
            TextParams {
                font_size: 16,
                font_scale: 1. / 4.,
                ..Default::default()
            },
        );
    }

    text::draw_text_ex(
        &format!("TPS: {tps: >6}"),
        state.config().world_width() + 1.,
        state.config().world_height() - 1.,
        TextParams {
            font_size: 16,
            font_scale: 1. / 4.,
            ..Default::default()
        },
    );
}

#[macroquad::main("Evolution")]
async fn main() {
    let config = evolution::Config::default();
    let mut state = State::init(config.clone(), 1);

    let camera = Camera::view_whole_world(&config, graphics::screen_size());

    let seconds_per_tick = SPEEDUP.map(|speedup| config.tick_length() as f64 / speedup);
    let mut next_tick_time = mq::get_time();

    let seconds_per_frame = 1. / FRAME_RATE;
    let mut next_frame_time = mq::get_time();

    let mut ticks_last_second = VecDeque::new();

    camera::set_camera(&camera.mq_camera(graphics::screen_size()));

    loop {
        let cur_time = mq::get_time();

        if mq::is_key_pressed(mq::KeyCode::Escape) {
            break;
        }
        while next_tick_time < next_frame_time {
            ticks_last_second.push_front(cur_time);
            while ticks_last_second
                .back()
                .map(|&t| cur_time - t > 1.)
                .unwrap_or(false)
            {
                ticks_last_second.pop_back();
            }
            state.tick();
            if let Some(seconds_per_tick) = seconds_per_tick {
                next_tick_time += seconds_per_tick;
            } else {
                next_tick_time = mq::get_time();
            }
        }

        if next_frame_time > mq::get_time() {
            std::thread::sleep(Duration::from_secs_f64(next_frame_time - mq::get_time()));
        }

        clear_background(Color::new(0.3921, 0.5842, 0.9294, 1.0));

        for entity in state.entities() {
            let color = match entity.entity_data() {
                EntityData::Creature(creature) => graphics::vec_to_color(
                    config.graphics.creature_color()
                        + Vector4::new(
                            0.,
                            0.,
                            0.,
                            creature.energy() / config.creature_max_energy() - 1.,
                        ),
                ),
                EntityData::Food => colors::GREEN,
            };

            let offsets = [Vector::new(0., 0.)];
            for offset in offsets {
                let location = entity.location()
                    + offset.component_mul(&(config.lower_right() - Location::ORIGIN));
                mq::draw_circle(
                    location.x(),
                    location.y(),
                    config.entity_size() * 0.5,
                    color,
                );
            }
        }

        let screen_edge = camera.camera_to_world(Location::ORIGIN + graphics::screen_size());
        mq::draw_rectangle(
            config.world_width(),
            0.,
            screen_edge.x() - config.world_width(),
            screen_edge.y(),
            mq::BLACK,
        );

        mq::draw_rectangle(
            0.,
            config.world_height(),
            screen_edge.x(),
            screen_edge.y() - config.world_height(),
            mq::BLACK,
        );

        draw_info(&state, ticks_last_second.len());

        camera::set_camera(&camera.mq_camera(graphics::screen_size()));

        next_frame_time += seconds_per_frame;
        mq::next_frame().await
    }
}
