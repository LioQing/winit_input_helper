//! A complex example demonstrating use of every API feature, runs on both desktop and web.
//! To run on desktop: `cargo run --example example`
//! To run on web: `cargo run-wasm --example example`
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

struct State {
    input: WinitInputHelper,
    window: Option<Window>,
}

impl State {
    fn new() -> Self {
        Self {
            input: WinitInputHelper::new(),
            window: None,
        }
    }
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(platform::create_window(event_loop));

        // Set control flow to poll so that it does not wait.
        // If you want the program to only run when events are received,
        // you should not use the return value of `window_event`,
        // instead directly run logics after `window_event` method.
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        self.input.new_events();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.input.window_event(&event) {
            if self.input.key_released(KeyCode::KeyQ) || self.input.close_requested() || self.input.destroyed()
            {
                log::info!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
                event_loop.exit();
                return;
            }

            // If you are taking input for a game or similar you should use physical keys.

            if self.input.key_pressed(KeyCode::KeyW) {
                log::info!("The 'W' key (US layout) was pressed on the keyboard");
            }

            if self.input.key_pressed_os(KeyCode::KeyE) {
                log::info!(
                    "The 'E' key (US layout) was pressed on the keyboard (Os Repeating)"
                );
            }

            if self.input.key_held(KeyCode::KeyR) {
                log::info!("The 'R' key (US layout) is held");
            }

            // Logical keys are usually used for text input and rarely make sense in the way they are presented in this API.

            if self.input.key_pressed_logical(Key::Character("a")) {
                log::info!("'a' was input by the keyboard");
            }

            if self.input.key_pressed_logical(Key::Character("A")) {
                log::info!("'A' was input by the keyboard (detected seperately to 'a')");
            }

            if self.input.key_pressed_os_logical(Key::Character("s")) {
                log::info!("'s' was input by the keyboard (OS repeating)");
            }

            if self.input.key_held_logical(Key::Character("d")) {
                log::info!("`d` input is held on the keyboard");
            }

            // query the change in cursor this update
            let cursor_diff = self.input.cursor_diff();
            if cursor_diff != (0.0, 0.0) {
                log::info!("The cursor diff is: {:?}", cursor_diff);
                log::info!("The cursor position is: {:?}", self.input.cursor());
            }

            // query the change in mouse this update (useful for first person camera controls)
            let mouse_diff = self.input.mouse_diff();
            if mouse_diff != (0.0, 0.0) {
                log::info!("The mouse diff is: {:?}", mouse_diff);
            }

            let scroll_diff = self.input.scroll_diff();
            if scroll_diff != (0.0, 0.0) {
                log::info!("The scroll diff is: {:?}", scroll_diff);
            }


            for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
                if self.input.mouse_pressed(button) {
                    log::info!("The {:?} mouse button was pressed", button);
                }

                if self.input.mouse_held(button) {
                    log::info!("The {:?} mouse button is being held", button);
                }

                if self.input.mouse_released(button) {
                    log::info!("The {:?} mouse button was released", button);
                }
            }


            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // Alternatively, you can put your logic in the `window_event` method to run on events only.
            // render();

            std::thread::sleep(std::time::Duration::from_millis(16));
            self.window.as_mut().unwrap().request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input.device_event(&event);
    }
}

fn main() {
    platform::init();

    let mut state = State::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut state).unwrap();
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use super::*;
    use winit::platform::web::{WindowAttributesExtWebSys, WindowExtWebSys};

    pub fn create_window(event_loop: &ActiveEventLoop) -> Window {
        let window = event_loop
            .create_window(Window::default_attributes().with_append(true))
            .unwrap();

        // Set a background color for the canvas to make it easier to tell the where the canvas is for debugging purposes.
        let canvas = window.canvas().unwrap();
        canvas.style().set_css_text(
            "display: block; background-color: crimson; margin: auto; width: 50%; aspect-ratio: 4 / 2;",
        );
        window
    }

    pub fn init() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("could not initialize logger");
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use super::*;

    pub fn create_window(event_loop: &ActiveEventLoop) -> Window {
        event_loop.create_window(Window::default_attributes()).unwrap()
    }

    pub fn init() {
        env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    }
}
