//! Use "cargo test -- --test-threads=1" to test this crate.
//! SDL windows aren't too happy about being created in parallel (as cargo runs it's tests).

#[cfg(test)]
mod tests {
    use rusty_gl::{
        vector::{Vector2, Vector3},
        shapes::{Rectangle, Drawable, Shape2D, CustomShape},
        ComputeShader, PipelineShader, ShaderSource, GPU, SSBO, Color, vertices::Vertex,
    };

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

        let window = video_subsystem
            .window("Test", 100, 100)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        //---------
        //Test setup
        //---------
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

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //-----------
            //Test step
            //-----------
            shader.dispatch(1, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.gl_swap_window();

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

        let window = video_subsystem
            .window("Test", 100, 100)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        //-----------
        //Test setup
        //-----------
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

        shader.set_uniform_int("value", 123);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //-----------
            //Test step
            //-----------
            shader.dispatch(10, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.gl_swap_window();

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
    fn rect_test() -> Result<(), Box<dyn std::error::Error>> {
        let sdl = sdl2::init().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let gl_attrib = video_subsystem.gl_attr();
        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(4, 5);

        let window = video_subsystem
            .window("Test", 100, 100)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        rusty_gl::debug::enable();

        const VERT_SHADER: &str = "
            #version 430
            layout (location = 0) in vec3 vPos;
            layout (location = 1) in vec3 vColor;
            layout (location = 2) in vec2 vTexCoord;

            out vec3 outColor;

            void main(){
                gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
                outColor = vColor;
            }
        ";

        const FRAG_SHADER: &str = "
            #version 430
            out vec4 FragColor;
            in vec3 outColor;

            void main()
            {
                FragColor = vec4(outColor, 1.0);
            }
        ";

        let shader = PipelineShader::create(
            Some(ShaderSource::String(VERT_SHADER)),
            Some(ShaderSource::String(FRAG_SHADER)),
        )?;
        shader.enable();

        let mut rect = Rectangle::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(1.5, 1.5),
            Some(vec![
                Color::new(1.0, 0.0, 0.0),
                Color::new(0.0, 1.0, 0.0),
                Color::new(0.0, 0.0, 1.0),
                Color::new(1.0, 1.0, 1.0),
            ]),
            None, None, None
        );

        rect
            .rotate(0.25 * 3.141592)
            .translate(Vector2::new(0.5, 0.0))
            .scale(0.75);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            rect.draw();
            window.gl_swap_window();
        }

        Ok(())
    }

    #[test]
    fn custom_shape_test() -> Result<(), Box<dyn std::error::Error>> {
        fn generate_circle(x: f32, y: f32) -> Vec<Vertex> {
            let mut vertices = Vec::new();

            for angle in (0..360).step_by(5).map(|x| x as f32 * 3.14159265 / 180.0) {
                vertices.push(Vertex::new(
                    Vector3::new(x + 0.2 * angle.cos(), y + 0.2 * angle.sin(), 0.0),
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

        //gl_attrib.set_multisample_buffers(1);
        //gl_attrib.set_multisample_samples(8);

        let window = video_subsystem
            .window("Test", 600, 600)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        unsafe { gl::Enable(gl::MULTISAMPLE); }

        rusty_gl::debug::enable();

        const VERT_SHADER: &str = "
            #version 430
            layout (location = 0) in vec3 vPos;
            layout (location = 1) in vec3 vColor;
            layout (location = 2) in vec2 vTexCoord;

            out vec3 outColor;

            void main(){
                gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
                outColor = vColor;
            }
        ";

        const FRAG_SHADER: &str = "
            #version 430
            out vec4 FragColor;
            in vec3 outColor;

            void main()
            {
                FragColor = vec4(outColor, 1.0);
            }
        ";

        let shader = PipelineShader::create(
            Some(ShaderSource::String(VERT_SHADER)),
            Some(ShaderSource::String(FRAG_SHADER)),
        )?;
        shader.enable();

        let mut vs = generate_circle(-0.8, 0.8);
        vs.insert(0, Vertex::new(Vector3::new(-0.8, 0.8, 0.0), Color::new(1.0, 1.0, 1.0), Vector2::new(0.0, 0.0)));
        let custom_shape_points = CustomShape::new(
            vs,
            gl::TRIANGLE_FAN,
            None, None, None,
        );

        let custom_shape_lines = CustomShape::new(
            generate_circle(-0.35, 0.8),
            gl::LINES,
            None, None, None,
        );

        let custom_shape_line_strip = CustomShape::new(
            generate_circle(0.1, 0.8),
            gl::LINE_STRIP,
            None, None, None,
        );

        let custom_shape_line_loop = CustomShape::new(
            generate_circle(0.55, 0.8),
            gl::LINE_LOOP,
            None, None, None,
        );

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            custom_shape_points.draw();
            custom_shape_lines.draw();
            custom_shape_line_strip.draw();
            custom_shape_line_loop.draw();
            
            window.gl_swap_window();
        }

        Ok(())
    }
}
