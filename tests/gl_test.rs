//! Use "cargo test -- --test-threads=1" to test this crate.
//! SDL windows aren't too happy about being created in parallel (as cargo runs it's tests).

#[cfg(test)]
mod tests {
    use rusty_gl::{
        color,
        shapes::{CustomShape2D, Shape2D},
        vertices::Vertex,
        Color, ComputeShader, ShaderSource, GPU, SSBO,
    };

    use vector::{Vector2, Vector3};

    pub struct Resolution {
        x: u32,
        y: u32,
    }

    impl GPU for Resolution {}

    #[test]
    fn ssbo_test() -> Result<(), Box<dyn std::error::Error>> {
        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();
        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let window = rusty_gl::Window::new()
            .dimensions(400, 400)
            .title("Testing Window")
            .build(&video_subsystem)?;

        rusty_gl::debug::enable();

        pub const COMPUTE_SHADER: &str = "
            #version 430
            layout(local_size_x = 1, local_size_y = 1) in;

            layout(std430, binding = 0) buffer Resolution {
                uint x;
                uint y;
            } resolution;

            void main() {
                resolution.x = 400;
                resolution.y = 400;
            }
        ";

        let shader = ComputeShader::create(ShaderSource::String(COMPUTE_SHADER))?;

        let resolution_struct = Resolution { x: 200, y: 200 };

        let mut ssbo = SSBO::create_from(0, resolution_struct, gl::STATIC_DRAW);

        //---------
        //Test setup verification
        //---------
        let target_resolution = Resolution { x: 200, y: 200 };
        ssbo.load();

        //Verify setup
        assert_eq!((*ssbo).x, target_resolution.x);
        assert_eq!((*ssbo).y, target_resolution.y);

        (*ssbo).x = 350;
        (*ssbo).y = 350;

        ssbo.update();
        ssbo.load();

        let target = Resolution { x: 350, y: 350 };

        assert_eq!((*ssbo).x, target.x);
        assert_eq!((*ssbo).y, target.y);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            window.clear(color::BLACK);

            //-----------
            //Test step
            //-----------
            shader.dispatch(1, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.gl_swap();

            //-----------
            //Test step verification
            //-----------
            let verify_resolution = Resolution { x: 400, y: 400 };
            ssbo.load();

            assert_eq!((*ssbo).x, verify_resolution.x);
            assert_eq!((*ssbo).y, verify_resolution.y);

            break;
        }

        Ok(())
    }

    #[test]
    fn compute_shader_test() -> Result<(), Box<dyn std::error::Error>> {
        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();
        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let window = rusty_gl::Window::new()
            .dimensions(400, 400)
            .title("Testing Window")
            .build(&video_subsystem)?;

        rusty_gl::debug::enable();

        pub const COMPUTE_SHADER: &str = "        
            #version 430
            layout(local_size_x = 1, local_size_y = 1) in;

            uniform int value;

            layout(std430, binding = 1) buffer Content {
                uint[] content;
            } arr;

            void main() {
                arr.content[gl_GlobalInvocationID.x] = value;
            }
        ";

        let shader = ComputeShader::create(ShaderSource::String(COMPUTE_SHADER))?;
        let vec = vec![0 as u32; 10];

        let mut ssbo = SSBO::create_from(1, vec, gl::STATIC_DRAW);

        //-----------
        //Test setup verification
        //-----------
        let vec_on_gpu = vec![0 as u32; 10];
        ssbo.load();

        assert_eq!(*ssbo, vec_on_gpu);

        shader.set_uniform("value", 123 as i32);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            window.clear(color::BLACK);

            //-----------
            //Test step
            //-----------
            shader.dispatch(10, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.gl_swap();

            //-----------
            //Test step verification
            //-----------
            ssbo.load();

            assert_eq!(*ssbo, vec![123 as u32; 10]);

            break;
        }

        Ok(())
    }

    #[test]
    fn custom_shape_test() -> Result<(), Box<dyn std::error::Error>> {
        fn generate_circle(x: f32, y: f32, radius: f32) -> Vec<Vertex> {
            let mut vertices = Vec::new();

            for angle in (0..360).step_by(5).map(|x| x as f32 * 3.14159265 / 180.0) {
                vertices.push(Vertex::new(
                    Vector3::new(x + radius * angle.cos(), y + radius * angle.sin(), 0.0),
                    Color::new(1.0, 1.0, 1.0),
                    Vector2::new(0.0, 0.0),
                ));
            }

            vertices
        }

        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();

        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let window = rusty_gl::Window::new()
            .dimensions(400, 400)
            .title("Testing Window")
            .build(&video_subsystem)?;

        rusty_gl::debug::enable();

        // gl_attrib.set_multisample_buffers(1);
        // gl_attrib.set_multisample_samples(8);
        // unsafe {
        //     gl::Enable(gl::MULTISAMPLE);
        // }

        let mut vs = generate_circle(100.0, 100.0, 50.0);
        vs.insert(
            0,
            Vertex::new(
                Vector3::new(100.0, 100.0, 0.0),
                Color::new(1.0, 1.0, 1.0),
                Vector2::new(0.0, 0.0),
            ),
        );

        let mut custom_shape_points = CustomShape2D::new(vs, gl::TRIANGLE_FAN);

        let mut custom_shape_lines =
            CustomShape2D::new(generate_circle(300.0, 100.0, 50.0), gl::LINES);

        let custom_shape_line_strip =
            CustomShape2D::new(generate_circle(100.0, 300.0, 50.0), gl::LINE_STRIP);

        let custom_shape_line_loop =
            CustomShape2D::new(generate_circle(300.0, 300.0, 50.0), gl::LINE_LOOP);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            custom_shape_points
                .translate(Vector2::new(0.03, 0.03))
                .rotate(0.05);

            custom_shape_lines.scale(0.001).rotate(0.005);

            window.clear(color::BLACK);

            window.draw(&custom_shape_points);
            window.draw(&custom_shape_lines);
            window.draw(&custom_shape_line_strip);
            window.draw(&custom_shape_line_loop);

            window.gl_swap();
        }

        Ok(())
    }
}
