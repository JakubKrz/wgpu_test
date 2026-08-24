use winit::event_loop::EventLoop;

pub mod app;
mod camera;
pub mod state;
mod texture;
mod vertex;

use crate::app::App;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
