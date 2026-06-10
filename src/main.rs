#[macro_use]
extern crate glium;
use glium::Surface;
//mod teapot;
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
        //tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, normal); //tex_coords);

    let shape = glium::vertex::VertexBuffer::new(&display, &[
        Vertex{position:[-1.0,  1.0, 0.0], normal:[0.0, 0.0,-1.0]}, //tex_coords:[0.0, 0.0]},
        Vertex{position:[ 1.0,  1.0, 0.0], normal:[0.0, 0.0,-1.0]}, //tex_coords:[1.0, 0.0]},
        Vertex{position:[-1.0, -1.0, 0.0], normal:[0.0, 0.0,-1.0]}, //tex_coords:[1.0, 1.0]},
        Vertex{position:[ 1.0, -1.0, 0.0], normal:[0.0, 0.0,-1.0]}, //tex_coords:[0.0, 1.0]},
    ]).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    //let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    //let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    //let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

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
                        view: matrix::view_matrix(&[0.0, 0.0, 1.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]),
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

                    //frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
                    frame.draw(&shape, &indices, &program, &uniforms, &params).unwrap();
                    frame.finish().unwrap();
                    angle.0 += 0.00;
                    angle.1 += 0.02;
                    angle.2 += 0.01;
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
