use glium::Surface;

fn main(){
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (_windwo, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Simple Window")
        .build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.4, 1.0, 1.0);
    frame.finish().unwrap();

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, ..} => match event {
            glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
            _ => (),
            },
            _ => (),
        };
    })
    .unwrap();
}