use evolution::graphics::{self, Camera};
use evolution::world::EntityData;
use evolution::{
    world::{Location, Vector},
    State,
};
use macroquad::prelude::{self as mq};
use macroquad::{
    camera,
    color::{colors, Color},
    window::clear_background,
};
use nalgebra::Vector4;

const SPEEDUP: f64 = 20.;

#[macroquad::main("Evolution")]
async fn main() {
    let config = evolution::Config::default();
    let mut state = State::init(config.clone(), 1);

    let camera = Camera::view_whole_world(&config, graphics::screen_size());

    let seconds_per_tick = config.tick_length() as f64 / SPEEDUP;
    let mut next_tick_time = mq::get_time();

    camera::set_camera(&camera.mq_camera(graphics::screen_size()));

    loop {
        while mq::get_time() > next_tick_time {
            state.tick();
            next_tick_time += seconds_per_tick;
        }

        while mq::get_time() < next_tick_time {
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

            camera::set_camera(&camera.mq_camera(graphics::screen_size()));

            mq::next_frame().await
        }
    }
}
