
extern crate gl;
extern crate glfw;

pub mod shader;

use shader::{Shader, Program};
use glfw::{Action, Context, Key};
use std::thread;

//http://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut glfw_window, events) = glfw.create_window(900, 700, 
        "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    glfw_window.make_current();
    glfw_window.set_key_polling(true);

    let _gl = gl::load_with(|s| glfw_window.get_proc_address(s) as *const std::os::raw::c_void);
    
    let vertex_shader = Shader::from_source_file("shaders/vertex_shader.vert", gl::VERTEX_SHADER).unwrap();
    let fragment_shader =  Shader::from_source_file("shaders/fragment_shader.frag", gl::FRAGMENT_SHADER).unwrap();

    let shader_program = Program::from_shaders(
        &[vertex_shader, fragment_shader]
    ).unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,      0.0, 0.0, 1.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3, 
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3, 
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as * const gl::types::GLvoid,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut render_ctx = glfw_window.render_context();
    glfw.make_context_current(None);

    thread::spawn(move || {
        render_ctx.make_current();

        while !render_ctx.should_close() { 
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
    
            shader_program.set_used();
    
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            render_ctx.swap_buffers();
        }
    });

    while !glfw_window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    glfw_window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}   