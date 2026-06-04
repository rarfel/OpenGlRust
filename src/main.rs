#[macro_use]
extern crate glium;
use glium::Surface;

// rotation matrix on all axis depending on a:yaw, b:pitch and g:roll
fn rotation_matrix(a:f32, b:f32, g:f32) -> [[f32; 4];4]{
    let ca = a.cos();
    let cb = b.cos();
    let cg = g.cos();

    let sa = a.sin();
    let sb = b.sin();
    let sg = g.sin();

    [
        [(ca*cb), (sa*cb), (-sb), 0.0],
        [(ca*sb*sg - sa*cg), (sa*sb*sg + ca*cg), (cb*sg), 0.0],
        [(ca*sb*cg + sa*sg), (sa*sb*cg - ca*sg), (cb*cg), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

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

    // making a triangle manually
    let shape = vec![
        Vertex{position:[-0.5, -0.5]},
        Vertex{position:[0.0, 0.5]},
        Vertex{position:[0.5, -0.25]}
    ];

    // creating a buffer to store the triangle
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // shader to render the triangle

    let vertex_shader_src: &'static str = include_str!("../shaders/vertex.glsl");

    let fragment_shader_src: &'static str = include_str!("../shaders/fragment.glsl");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // variable to control the animation of the scene
    let mut t:f32 = 0.0;
    // event loop
    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, ..} => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x_off = t.cos() * 0.5;
                    let y_off = t.sin() * 0.5;

                    // draw function
                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.4, 1.0, 1.0);
                    let uniforms = uniform! {
                        matrix: rotation_matrix(0.0,0.0,0.0),
                        x: x_off,
                        y: y_off,
                        t: t,
                    };
                    frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
                glium::winit::event::WindowEvent::Resized(window_size)=>{
                    display.resize(window_size.into());
                }

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