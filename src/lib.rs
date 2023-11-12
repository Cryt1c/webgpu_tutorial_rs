#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::{Element, HtmlCanvasElement};
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;

        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas_opt: Option<HtmlCanvasElement> = window.canvas();
                if let Some(canvas) = canvas_opt {
                    let canvas_elem: Element = canvas.into();
                    dst.append_child(&canvas_elem).ok()?;
                }
                Some(())
                // let canvas = web_sys::Element::from(window.canvas());
                // dst.append_child(&canvas).ok()?;
                // Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut close_requested = false;

    let _ = event_loop.run(move |event, control_flow| {
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
