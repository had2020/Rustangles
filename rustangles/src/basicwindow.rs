extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
   use glfw::fail_on_errors;
let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // create opengl window 
    let (mut window, events) = glfw.create_window(300, 300, "Rustangles", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // make's windows context to current opengl thing
    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        // buffering frames
        window.swap_buffers();

        // event handling input 
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { // input esc
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}