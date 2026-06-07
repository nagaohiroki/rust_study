mod camera;
mod ecs;
mod input_manager;
mod mesh;
mod primitive;
mod scene;
mod shader;
mod shader_uniform;
mod state;
mod time_manager;
mod transform;
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
fn open_window() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Rust wgpu Window")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap(),
    );
    let mut wgpu_state = pollster::block_on(state::State::new(window.clone()));
    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::RedrawRequested => wgpu_state.update(),
                WindowEvent::Resized(size) => wgpu_state.resize(size),
                WindowEvent::ScaleFactorChanged { .. } => wgpu_state.resize(&window.inner_size()),
                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => wgpu_state.input_event(&key_event),
                _ => {}
            },
            Event::AboutToWait => window.request_redraw(),
            _ => {}
        })
        .unwrap();
}
async fn graphics_info() {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("not found gpu");
    let info = adapter.get_info();
    println!("usage GPU: {}", info.name);
    println!("backend: {:?}", info.backend);
}
fn main() {
    pollster::block_on(graphics_info());
    open_window();
}
