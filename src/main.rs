mod renderer;
mod vectors;
mod vk_pipeline;

use vk_pipeline::create_pipeline;

pub const APP_NAME: &str = "Ash - Example";
pub const WINDOW_SIZE: [u32; 2] = [1024, 792];

fn main() {
    create_pipeline();
}
