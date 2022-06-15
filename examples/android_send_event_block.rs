#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    use simple_logger::SimpleLogger;
    use winit::{
        event::{ElementState, Event, Touch, TouchPhase, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    enum UserEvent {
        MyEvent,
    }

    let event_loop = EventLoop::with_user_event();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let proxy = event_loop.create_proxy();
    let mut request_redraw = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Touch(Touch {
                    phase: TouchPhase::Started,
                    ..
                })
                | WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    ..
                } => {
                    request_redraw = !request_redraw;
                    println!("request_redraw = {}", request_redraw);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                println!("MainEventsCleared!");
                std::thread::sleep(std::time::Duration::from_millis(200));
                let _ = proxy.send_event(UserEvent::MyEvent);
                if request_redraw {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                println!("Redrawing!");
            }
            Event::UserEvent(UserEvent::MyEvent) => {
                println!("I received MyEvent");
            }
            _ => (),
        }
    });
}
