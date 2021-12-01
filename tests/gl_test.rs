//! Use "cargo test -- --test-threads=1" to test this crate.
//! GLFW windows aren't too happy about being created in parallel (as cargo runs it's tests).

#[cfg(test)]
mod tests {
    use glfw::Context;
    use open_rl::{ComputeShader, GpuSsbo, PipelineShader, SSBO, ShaderSource};

    pub struct Resolution {
        x: u32,
        y: u32,
    }

    impl GpuSsbo for Resolution {}

    #[test]
    fn ssbo_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, _) = glfw
            .create_window(100, 100, "Test", glfw::WindowMode::Windowed)
            .expect("");

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        //---------
        //Test setup
        //---------
        open_rl::debug::enable();

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
        let mut resolution_tuple = (100 as u32, 100 as u32); //TODO use

        let ssbo = SSBO::create_from(0, &resolution_struct, gl::STATIC_DRAW);

        //---------
        //Test setup verification
        //---------
        let mut resolution_on_gpu = Resolution {x:1,y:1};
        resolution_on_gpu.load_from(&ssbo, 0);

        assert_eq!(resolution_on_gpu.x, resolution_on_gpu.x);
        assert_eq!(resolution_on_gpu.y, resolution_on_gpu.y);

        resolution_tuple.save_to(&ssbo, 0);
        resolution_on_gpu.load_from(&ssbo, 0);

        assert_eq!(resolution_on_gpu.x, resolution_tuple.0);
        assert_eq!(resolution_on_gpu.y, resolution_tuple.1);

        while !window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //-----------
            //Test step
            //-----------
            shader.dispatch(1, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.swap_buffers();

            //-----------
            //Test step verification
            //-----------
            resolution_on_gpu.load_from(&ssbo, 0);

            assert_eq!(resolution_on_gpu.x, 400);
            assert_eq!(resolution_on_gpu.y, 400);

            resolution_tuple.load_from(&ssbo, 0);
            assert_eq!(resolution_tuple, (400,400));

            window.set_should_close(true);
        }

        Ok(())
    }

    #[test]
    fn compute_shader_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, _) = glfw
            .create_window(100, 100, "Test", glfw::WindowMode::Windowed)
            .expect("");

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        //-----------
        //Test setup
        //-----------
        open_rl::debug::enable();

        pub const COMPUTE_SHADER: &str = "        
            #version 430
            layout(local_size_x = 1, local_size_y = 1) in;

            layout(std430, binding = 1) buffer Content {
                uint[] content;
            } arr;

            void main() {
                arr.content[gl_GlobalInvocationID.x] = 123;
            }
        ";

        let shader = ComputeShader::create(ShaderSource::String(COMPUTE_SHADER))?;
        let vec = vec![0 as u32; 10];

        let ssbo = SSBO::create_from(1, &vec, gl::STATIC_DRAW);

        //-----------
        //Test setup verification
        //-----------
        let mut vec_on_gpu = vec![0 as u32; 10];
        vec_on_gpu.load_from(&ssbo, 0);

        assert_eq!(vec_on_gpu, vec);

        while !window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //-----------
            //Test step
            //-----------
            shader.dispatch(10, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

            window.swap_buffers();

            //-----------
            //Test step verification
            //-----------
            vec_on_gpu.load_from(&ssbo, 0);
            assert_eq!(vec_on_gpu, vec![123 as u32; 10]);

            window.set_should_close(true);
        }

        Ok(())
    }

    #[test]
    fn pipeline_shader_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, _) = glfw
            .create_window(100, 100, "Test", glfw::WindowMode::Windowed)
            .expect("");

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        //-----------
        //Test setup
        //-----------
        open_rl::debug::enable();

        const VERT_SHADER: &str = "
            #version 430
            layout (location = 0) in vec3 vPos;

            void main(){
                gl_Position = vec4(vPos, 1.0);
            }
        ";

        const FRAG_SHADER: &str = "
            #version 430
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ";

        let shader = PipelineShader::create(
            Some(ShaderSource::String(VERT_SHADER)), 
            Some(ShaderSource::String(FRAG_SHADER))
        )?;
        shader.enable();

        //-----------
        //Test setup verification
        //-----------


        while !window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //-----------
            //Test step
            //-----------
            shader.enable();

            window.swap_buffers();

            //-----------
            //Test step verification
            //-----------

            window.set_should_close(true);
        }

        Ok(())
    }
}
