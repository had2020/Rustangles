use std::convert::TryInto;

use glfw;
use glfw::Context;
use gl;

const WIDTH: u32 = 1000; // 480
const HEIGHT: u32 = 800; // 320
const TITLE: &str = "Rustangles";

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed).unwrap();
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        clear_color(Color(0.4, 0.4, 0.4, 1.0)); // background
    }
    // -------------------------------------------

    const VERT_SHADER: &str = "#version 330 core
    layout (location = 0) in vec3 position;
    
    void main()
    {
        gl_Position = vec4(position, 1.0);
        // gl_Position = vec4(position.xyz, 1.0);
        // gl_Position = vec4(position.x, position.y, position.z, 1.0);
    }";

    // Triangle color
    const FRAG_SHADER: &str = "#version 330 core
    out vec4 Color;
    void main()
    {
        Color = vec4(0.9, 0.5, 0.2, 1.0);
    }";

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &VERT_SHADER.as_bytes().as_ptr().cast(), &VERT_SHADER.len().try_into().unwrap());
        gl::CompileShader(vertex_shader);
        
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            // gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_len);
            // let mut v: Vec<u8> = Vec::with_capacity(log_len as usize);
            // gl::GetShaderInfoLog(vertex_shader, log_len, &mut log_len, v.as_mut_ptr().cast());
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &FRAG_SHADER.as_bytes().as_ptr().cast(), &FRAG_SHADER.len().try_into().unwrap());
        gl::CompileShader(fragment_shader);
        
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // (X)   (Y)    (Z)
    let mut vertices = [
        -0.2f32, -0.2, 0.0, 
        0.2, -0.2, 0.0, 
        0.0, 0.2, 0.0, 
    ];

    let mut vao;

    vao = vertex_into_frame(vertices);
    
    // -------------------------------------------
    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!("GLSL version: {}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));

    while !window.should_close() { // every frame
        glfw.poll_events(); // inputs are events
        for (_, event) in glfw::flush_messages(&events) {
            
            // INPUT handeling
            use glfw::WindowEvent as Event;
            use glfw::Key;
            use glfw::Action;

            let speed = 0.2f32;

            match event {
                Event::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                Event::Key(Key::Up | Key::W, _, Action::Repeat | Action::Press, _) => {
                    vertices[1] += speed;
                    vertices[4] += speed;
                    vertices[7] += speed;
                    vao = vertex_into_frame(vertices);
                },
                Event::Key(Key::Down | Key::S, _, Action::Repeat | Action::Press, _) => {
                    vertices[1] += -speed;
                    vertices[4] += -speed;
                    vertices[7] += -speed;
                    vao = vertex_into_frame(vertices);
                },
                Event::Key(Key::Left | Key::A, _, Action::Repeat | Action::Press, _) => {
                    vertices[0] += -speed;
                    vertices[3] += -speed;
                    vertices[6] += -speed;
                    vao = vertex_into_frame(vertices);
                },
                Event::Key(Key::Right | Key::D, _, Action::Repeat | Action::Press, _) => {
                    vertices[0] += speed;
                    vertices[3] += speed;
                    vertices[6] += speed;
                    vao = vertex_into_frame(vertices);
                },
                /* Debug input message 
                Event::Key(key, _, Action::Press, _) => {
                    println!("Other key pressed: {:?}", key);
                },
                */
                _ => {},
            }
            // END of input handeling
        }

        clear_color(Color(0.3, 0.4, 0.6, 1.0));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::UseProgram(shader_program); // sharder created earlyer
            gl::BindVertexArray(vao); 

            gl::DrawArrays(gl::TRIANGLES, 0, 3); // drawing of arrays

            gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }
}

pub struct Color(f32, f32, f32, f32);

pub fn clear_color(c: Color) {
    unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn vertex_into_frame(vertices: [f32; 9]) -> u32{
    // VERTEX Buffer 
    // Setup a buffer is storing each frame
    // stores variable, which stores id of vertices. Code named Vertex Array Object (VAO)
    let mut vao = 0;
    // creates new `vao` variable
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    // Create a variable to store the ID of the Vertex Buffer Object (VBO)
    let mut vbo = 0;
    // Generate a new VBO and store its ID in the `vbo` variable
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        // Bind the VAO, making it the active one for the current OpenGL context
        gl::BindVertexArray(vao);

        // Bind the VBO to the `GL_ARRAY_BUFFER` target, making it the active buffer for vertex data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Copy vertex data into the bound buffer (VBO), specifying its size and usage pattern
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize, // Size of the data in bytes
            vertices.as_ptr().cast(), // Pointer to the vertex data
            gl::STATIC_DRAW // Usage hint, indicating the data will be set once and used many times
        );

        // Define the layout of the vertex data:
        // - Index 0: Refers to the position attribute of the vertex
        // - Each vertex attribute contains 3 components (x, y, z)
        // - Data type is `GL_FLOAT`
        // - No normalization (`gl::FALSE`)
        // - Stride is 3 times the size of a `f32` (indicating the space between consecutive vertex attributes)
        // - Offset is `0`, indicating the start position in the buffer
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            3 * std::mem::size_of::<f32>() as i32, 
            0 as *const _
        );
        // Enable the vertex attribute at index 0 (the position attribute)
        gl::EnableVertexAttribArray(0);

        // Unbind the VBO from `GL_ARRAY_BUFFER` to avoid accidental modification
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // Unbind the VAO, which now holds the state of the vertex attribute setup
        gl::BindVertexArray(0);
        // END of vertex buffer array object intitliztion
    }
    vao
}