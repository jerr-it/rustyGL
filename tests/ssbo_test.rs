#[cfg(test)]
mod tests {
    use glfw::{Action, Context, Key};
    use open_rl::{ComputeShader, GpuSsbo, ShaderSource, SSBO};

    const COMP_SHADER: &str = "
        #version 430
        layout(local_size_x = 1, local_size_y = 1) in;

        layout(std430, binding = 1) buffer Resolution {
            int x;
            int y;
        } resolution;

        void main() {
            resolution.x = 400;
            resolution.y = 400;
        }
    ";

    struct Resolution {
        resolution_x: i32,
        resolution_y: i32,
    }

    impl GpuSsbo for Resolution {}

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

        let test_resolution = Resolution {
            resolution_x: 100,
            resolution_y: 200,
        };

        let ssbo = SSBO::create_from(1, &test_resolution, gl::STATIC_DRAW);
        let mut unchanged_resolution = Resolution {
            resolution_x: 0,
            resolution_y: 0,
        };

        unchanged_resolution.load(&ssbo, 0);

        assert_eq!(
            test_resolution.resolution_x,
            unchanged_resolution.resolution_x
        );
        assert_eq!(
            test_resolution.resolution_y,
            unchanged_resolution.resolution_y
        );

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader.dispatch(1, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.swap_buffers();

            let mut changed_resolution = Resolution {
                resolution_x: 0,
                resolution_y: 0,
            };
            changed_resolution.load(&ssbo, 0);

            assert_eq!(400, changed_resolution.resolution_x);
            assert_eq!(400, changed_resolution.resolution_y);

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
