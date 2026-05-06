use windowing::WindowState;

pub mod renderer;
pub mod windowing;

pub fn run() {
    env_logger::init();

    let window = WindowState::new("CopperBird", 1280, 720);
    window.run();
}
