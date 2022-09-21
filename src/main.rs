mod window;

use pollster;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    pollster::block_on(window::draw());
}
