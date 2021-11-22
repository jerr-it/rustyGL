//! Use "cargo test -- --nocapture" for details on which test failed exactly.
//! Testing these OpenGL features is more difficult since they need an OpenGL context to work in.
//! GLFW windows aren't too happy about being created in parallel (as cargo runs it's tests).
//! At least that's what I suspect the issue to be. That's the reason behind the odd test structure.

mod ssbo_test {
    use open_rl::{ComputeShader, GpuSsbo, ShaderSource, SSBO};

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

    pub struct Resolution {
        x: u32,
        y: u32,
    }

    impl GpuSsbo for Resolution {}

    pub fn setup(
    ) -> Result<(ComputeShader, Resolution, (u32, u32), SSBO), Box<dyn std::error::Error>> {
        let shader = ComputeShader::create(ShaderSource::String(COMPUTE_SHADER))?;

        let resolution_struct = Resolution { x: 200, y: 200 };
        let resolution_tuple = (100 as u32, 100 as u32); //TODO use

        let ssbo = SSBO::create_from(0, &resolution_struct, gl::STATIC_DRAW);

        Ok((shader, resolution_struct, resolution_tuple, ssbo))
    }

    pub fn verify_setup(
        setup: &(ComputeShader, Resolution, (u32, u32), SSBO),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut resolution_on_gpu = Resolution { x: 1, y: 1 };
        resolution_on_gpu.load(&setup.3, 0);

        assert_eq!(resolution_on_gpu.x, setup.1.x);
        assert_eq!(resolution_on_gpu.y, setup.1.y);

        setup.2.store(&setup.3, 0);
        resolution_on_gpu.load(&setup.3, 0);

        assert_eq!(resolution_on_gpu.x, setup.2 .0);
        assert_eq!(resolution_on_gpu.y, setup.2 .1);

        Ok(())
    }

    pub fn step(
        setup: &(ComputeShader, Resolution, (u32, u32), SSBO),
    ) -> Result<(), Box<dyn std::error::Error>> {
        setup.0.dispatch(1, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);
        Ok(())
    }

    pub fn verify_step(
        setup: &(ComputeShader, Resolution, (u32, u32), SSBO),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut resolution_on_gpu = Resolution { x: 1, y: 1 };
        resolution_on_gpu.load(&setup.3, 0);

        assert_eq!(resolution_on_gpu.x, 400);
        assert_eq!(resolution_on_gpu.y, 400);

        let mut tup = (0 as u32, 0 as u32);
        tup.load(&setup.3, 0);

        assert_eq!(tup.0, 400);
        assert_eq!(tup.1, 400);

        Ok(())
    }
}

mod compute_shader_test {
    use open_rl::{ComputeShader, GpuSsbo, ShaderSource, SSBO};

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

    pub fn setup() -> Result<(ComputeShader, SSBO, Vec<u32>), Box<dyn std::error::Error>> {
        let shader = ComputeShader::create(ShaderSource::String(COMPUTE_SHADER))?;
        let vec = vec![0 as u32; 10];

        let ssbo = SSBO::create_from(1, &vec, gl::STATIC_DRAW);

        Ok((shader, ssbo, vec))
    }

    pub fn verify_setup(
        setup: &(ComputeShader, SSBO, Vec<u32>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut vec_on_gpu = vec![0 as u32; 10];
        vec_on_gpu.load(&setup.1, 0);

        assert_eq!(vec_on_gpu, setup.2);

        Ok(())
    }

    pub fn step(setup: &(ComputeShader, SSBO, Vec<u32>)) -> Result<(), Box<dyn std::error::Error>> {
        setup.0.dispatch(10, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);
        Ok(())
    }

    pub fn verify_step(
        setup: &(ComputeShader, SSBO, Vec<u32>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut vec_on_gpu = vec![0 as u32; 10];
        vec_on_gpu.load(&setup.1, 0);

        assert_eq!(vec_on_gpu, vec![123 as u32; 10]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use glfw::Context;

    use crate::{compute_shader_test, ssbo_test};

    #[test]
    fn gl_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, _) = glfw
            .create_window(100, 100, "Test", glfw::WindowMode::Windowed)
            .expect("");

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        //Test setup
        let ssbo_test_setup = ssbo_test::setup()?;
        let compute_shader_test_setup = compute_shader_test::setup()?;

        //Test setup verification
        ssbo_test::verify_setup(&ssbo_test_setup)?;
        compute_shader_test::verify_setup(&compute_shader_test_setup)?;

        while !window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //Test step
            ssbo_test::step(&ssbo_test_setup)?;
            compute_shader_test::step(&compute_shader_test_setup)?;

            window.swap_buffers();

            //Test step verification
            ssbo_test::verify_step(&ssbo_test_setup)?;
            compute_shader_test::verify_step(&compute_shader_test_setup)?;

            window.set_should_close(true);
        }

        Ok(())
    }
}
