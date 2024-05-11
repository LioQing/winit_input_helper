//! The simplest example, supporting only desktop applications.

use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
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
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn new_events(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, _cause: winit::event::StartCause) {
        self.input.new_events();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.input.window_event(&event);
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.input.device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
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
        // Alternatively, you can put your logic in the `window_event` method to run on events only.
        // render();

        self.window.as_mut().unwrap().request_redraw();
    }
}

fn main() {
    let mut state = State::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut state).unwrap();
}
