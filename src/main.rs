use rayon::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[derive(Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Color{
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (color.b*255.0) as u32 | ((color.g*255.0) as u32) << 8 | ((color.r*255.0) as u32) << 16
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let x = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

fn circle(x: f32, y: f32, radius: f32, feather: f32) -> f32 {
    let x = x*x+y*y;
    smoothstep(x,x+feather, radius*radius)
}

fn donut(x: f32, y: f32, outer_radius: f32, inner_radius: f32, feather: f32) -> f32 {
    circle(x, y, outer_radius, feather) - circle(x, y, inner_radius, feather)
}

fn main() {
    let mut window_size = (800usize, 800usize);
    let background_color = Color::new(0.1, 0.1, 0.1);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Codotaku CPU Pixel Shader")
        .with_inner_size(PhysicalSize::new(window_size.0 as u32, window_size.1 as u32))
        .build(&event_loop).unwrap();

    let mut graphics_context = unsafe { softbuffer::GraphicsContext::new(window) }.unwrap();
    let mut buffer = vec![background_color.into(); window_size.0*window_size.1];
    let start = std::time::Instant::now();

    let threads: usize = std::thread::available_parallelism().unwrap().into();

    event_loop.run(move |event, _, control_flow| {
        use winit::event::Event;
        match event {
            Event::MainEventsCleared => {
                let tt = std::time::Instant::now();
                let len = buffer.len();
                let chunk_size = len/threads;
                let t = start.elapsed().as_secs_f32();
                let aspect_ratio = window_size.0 as f32/window_size.1 as f32;

                buffer.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_index, chunk)| {
                    chunk.iter_mut().enumerate().for_each(|(pixel_index, pixel)| {
                        let window_pixel_index = pixel_index + chunk_index*chunk_size;
                        let abs_x = window_pixel_index % window_size.0;
                        let abs_y = window_pixel_index / window_size.0;
                        let mut x = abs_x as f32/window_size.0 as f32;
                        let mut y = abs_y as f32/window_size.1 as f32;

                        x += -0.5;
                        x *= aspect_ratio;
                        y += -0.5;

                        let mut c = 0.0;
                        c += donut(x, y, 0.5, 0.4, 0.01);
                        c += donut(x-t.sin(), y, 0.5, 0.4, 0.01);

                        *pixel = Color::new(c, c, c).into();
                    });
                });

                graphics_context.set_buffer(&buffer, window_size.0 as u16, window_size.1 as u16);
                println!("{}", 1.0/tt.elapsed().as_secs_f64())
            }
            Event::WindowEvent {event, ..} => {
                use winit::event::WindowEvent;
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::Resized(physical_size) => {
                        window_size = (physical_size.width as usize, physical_size.height as usize);
                        buffer.resize(window_size.0*window_size.1, background_color.into());
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    })
}
