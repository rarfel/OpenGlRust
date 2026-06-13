#[macro_use]
extern crate glium;
use glium::Surface;
mod matrix;

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
        position: [f32; 3],
        normal: [f32; 3],
    }
    implement_vertex!(Vertex, position, normal);

    let shape = glium::VertexBuffer::new(&display, &[
        Vertex{position:[ 1.0,  1.0, -1.0], normal:[ 1.0, 1.0,-1.0]},
        Vertex{position:[ 1.0, -1.0, -1.0], normal:[ 1.0,-1.0,-1.0]},
        Vertex{position:[ 1.0,  1.0,  1.0], normal:[ 1.0, 1.0, 1.0]},
        Vertex{position:[ 1.0, -1.0,  1.0], normal:[ 1.0,-1.0, 1.0]},
        Vertex{position:[-1.0,  1.0, -1.0], normal:[-1.0, 1.0,-1.0]},
        Vertex{position:[-1.0, -1.0, -1.0], normal:[-1.0,-1.0,-1.0]},
        Vertex{position:[-1.0,  1.0,  1.0], normal:[-1.0, 1.0, 1.0]},
        Vertex{position:[-1.0, -1.0,  1.0], normal:[-1.0,-1.0, 1.0]},

    ]).unwrap();

    let index:[u8; 36] = [
        0, 4, 6,
        0, 6, 2,

        3, 2, 6,
        3, 6, 7,

        7, 6, 4,
        7, 4, 5,

        5, 1, 3,
        5, 3, 7,

        1, 0, 2,
        1, 2, 3,

        5, 4, 0,
        5, 0, 1,
    ];
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &index).unwrap();

    let vertex_shader_src: &'static str = include_str!("../shaders/vertex.glsl");

    let fragment_shader_src: &'static str = include_str!("../shaders/fragment.glsl");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut angle:(f32,f32,f32) = (0.0,0.0,0.0);
    // event loop
    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, ..} => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // draw function
                    let mut frame = display.draw();
                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                    let uniforms = uniform! {
                        matrix: matrix::model_matrix(),
                        view: matrix::view_matrix(&[0.0, 0.0,-1.0], &[ 0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]),
                        rotation: matrix::rotation_matrix(angle),
                        projection: matrix::projection_matrix(frame.get_dimensions()),
                    };

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
                        .. Default::default()
                    };

                    frame.draw(&shape, &indices, &program, &uniforms, &params).unwrap();
                    frame.finish().unwrap();
                    angle.0 += 0.003;
                    angle.1 += 0.005;
                    angle.2 += 0.000;
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
