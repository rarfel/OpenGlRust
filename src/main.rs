#[macro_use]
extern crate glium;
use glium::Surface;
mod matrix;
mod file_type;

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
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, normal, tex_coords);

    let shape = glium::vertex::VertexBuffer::new(&display, &[
        Vertex{position:[ 1.0, -1.0, 0.0], normal:[0.0, 0.0,-1.0], tex_coords:[1.0, 0.0]},
        Vertex{position:[ 1.0,  1.0, 0.0], normal:[0.0, 0.0,-1.0], tex_coords:[1.0, 1.0]},
        Vertex{position:[-1.0, -1.0, 0.0], normal:[0.0, 0.0,-1.0], tex_coords:[0.0, 0.0]},
        Vertex{position:[-1.0,  1.0, 0.0], normal:[0.0, 0.0,-1.0], tex_coords:[0.0, 1.0]},

    ]).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let image = file_type::load_image("../textures/wall.jpg").unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let image = file_type::load_image("../textures/wall-normal.png").unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_tex = glium::texture::Texture2d::new(&display, image).unwrap();

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
                        tex: &diffuse_texture,
                        normal_tex: &normal_tex,
                    };

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };

                    frame.draw(&shape, &indices, &program, &uniforms, &params).unwrap();
                    frame.finish().unwrap();
                    angle.0 += 0.000;
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
