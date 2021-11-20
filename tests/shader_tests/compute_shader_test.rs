#[cfg(test)]
mod tests {
    use glfw::{Action, Context, Key};
    use open_rl::{ComputeShader, GpuSsbo, ShaderSource, SSBO};

    const COMP_SHADER: &str = "
        #version 430
        layout(local_size_x = 1, local_size_y = 1) in;

        layout(std430, binding = 0) buffer List {
            uint[] content;
        } list;

        void main() {
            ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);
            list.content[iCoords.x] = 10;
        }
    ";

    #[test]
    fn ssbo() -> Result<(), Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut window, events) = glfw
            .create_window(100, 100, "SSBO Test", glfw::WindowMode::Windowed)
            .expect("Can't create window");

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        glfw.make_context_current(Some(&window));

        let shader = ComputeShader::create(ShaderSource::String(COMP_SHADER))
            .expect("Failed to create shader");

        let vec = vec![0 as u32; 10];
        let ssbo = SSBO::create_from(0, &vec, gl::DYNAMIC_DRAW);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader.dispatch(10, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.swap_buffers();

            let mut cmp_vec = vec![0; 10];
            cmp_vec.load(&ssbo, 0);

            assert_eq!(cmp_vec, vec![10 as u32; 10]);

            window.set_should_close(true);
        }

        Ok(())
    }

    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}
