#[macro_use]
extern crate glium;
use glium::Surface;

fn main(){
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Simple Window")
        .build(&event_loop);

    // Defining a struct to hold a vertex
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    // shader to render the triangle

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // variable to control the animation of the scene
    let mut t:f32 = 0.0;
    // event loop
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, ..} => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size)=>{
                    display.resize(window_size.into());
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x_off = t.cos() * 0.5;
                    let y_off = t.sin() * 0.5;

                    // making a triangle manually
                    let shape = vec![
                        Vertex{position:[-0.5 + x_off, -0.5 + y_off]},
                        Vertex{position:[0.0 + x_off, 0.5 + y_off]},
                        Vertex{position:[0.5 + x_off, -0.25 + y_off]}
                    ];

                    // creating a buffer to store the triangle
                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    // draw function
                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.4, 1.0, 1.0);
                    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
            _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                _window.request_redraw();
            },
            _ => (),
        };
    })
    .unwrap();
}