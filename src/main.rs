use wgpu;

fn main() {
    pollster::block_on(run());
}
async fn run() {
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
