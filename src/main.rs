#[macro_use]
extern crate glium;
use glium::Surface;
mod teapot;

// rotation matrix on all axis depending on a:yaw, b:pitch and g:roll
fn rotation_matrix(a:f32, b:f32, g:f32) -> [[f32; 4];4]{
    let ca = a.cos();
    let cb = b.cos();
    let cg = g.cos();

    let sa = a.sin();
    let sb = b.sin();
    let sg = g.sin();

    [
        [(ca*cb),            (sa*cb),            (-sb),   0.0],
        [(ca*sb*sg - sa*cg), (sa*sb*sg + ca*cg), (cb*sg), 0.0],
        [(ca*sb*cg + sa*sg), (sa*sb*cg - ca*sg), (cb*cg), 0.0],
        [0.0,                0.0,                2.0,     1.0],
    ]
}
// projection matrix, to make the gpu know what divide x and y for
fn projection_matrix((width, height): (u32, u32)) -> [[f32; 4]; 4]{
    let aspect_ratio = height as f32 / width as f32;

    const PI:f32 = 3.141592;
    let fov:f32 = 90.0;
    let f:f32 = 1.0 / (fov * 0.5 / 180.0 * PI ).tan(); // converting to radians

    let zfar:f32 = 1024.0;
    let znear:f32 = 0.1;

    let q:f32 = zfar / (zfar - znear);

    [
        [aspect_ratio * f,  0.0,    0.0,    0.0],
        [0.0,               f,      0.0,    0.0],
        [0.0,               0.0,    q,      1.0],
        [0.0,               0.0,-znear * q, 0.0],
    ]
}

fn main(){
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Simple Window")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

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
                        matrix: rotation_matrix(angle.0,angle.1,angle.2),
                        projection: projection_matrix(frame.get_dimensions()),
                        light: [-1.0, 0.4, 0.9f32],
                    };

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };

                    frame.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();
                    frame.finish().unwrap();
                    angle.0 += 0.006;
                    angle.1 += 0.008;
                    angle.2 += 0.010;
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
