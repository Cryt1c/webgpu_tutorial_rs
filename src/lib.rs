use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut close_requested = false;

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => close_requested = true,
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key.as_ref() {
                    Key::Named(NamedKey::Escape) => close_requested = true,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        if close_requested {
            control_flow.exit();
        }
    });
}
