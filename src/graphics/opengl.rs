
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::event::Event;
use winit::event::WindowEvent;

pub fn render() {
    println!("rendering graphics...");

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Gamengine")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        // println!("{event:?}");

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });   

}