//! The simplest example, supporting only desktop applications.

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
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
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());

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
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.input.window_event(&event);
        if let WindowEvent::RedrawRequested = event {
            if self.input.key_released(KeyCode::KeyQ) || self.input.close_requested() || self.input.destroyed()
            {
                println!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
                event_loop.exit();
                return;
            }

            if self.input.key_pressed(KeyCode::KeyW) {
                println!("The 'W' key (US layout) was pressed on the keyboard");
            }


            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // render();

            std::thread::sleep(std::time::Duration::from_millis(16));
            self.window.as_mut().unwrap().request_redraw();

            self.input.end_step();
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
    let mut state = State::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut state).unwrap();
}
