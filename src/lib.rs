mod chunk;
mod mesh;
mod renderer;

use std::sync::{Arc, Mutex};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::renderer::Renderer;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Arc<Mutex<Renderer<'static>>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_title = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let window_attributes = Window::default_attributes().with_title(window_title);
        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        self.renderer = Some(Arc::new(Mutex::new(pollster::block_on(Renderer::new(
            Arc::clone(self.window.as_ref().unwrap()),
        )))));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .resize(physical_size);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut renderer = self.renderer.as_mut().unwrap().lock().unwrap();

                match renderer.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Warning: a surface timeout occured");
                    }
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                        let size = renderer.size();
                        renderer.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("A surface error occured: OutOfMemory");
                        event_loop.exit();
                    }
                    Err(wgpu::SurfaceError::Other) => {
                        log::error!("An unspecified surface error occured");
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
