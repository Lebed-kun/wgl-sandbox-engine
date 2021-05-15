use crate::types::{DrawProps, Matrix4x4, Vector4};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

mod constants {
    pub const u_viewport: &'static str = "u_viewport";
    pub const u_position: &'static str = "u_position";
    pub const u_rotation: &'static str = "u_rotation";
    pub const u_scale: &'static str = "u_scale";
    pub const u_color: &'static str = "u_color";
    pub const a_vertex: &'static str = "a_vertex";
}

fn is_valid_u_location(v: &WebGlUniformLocation) -> bool {
    let num = v.as_f64();
    num.is_some() && num.unwrap() >= 0.0
}

macro_rules! try_locate_uniform {
    ($gl:expr, $program:expr, $u_name:expr) => {
        {
            try_unwrap!(
                @dev;
                ($gl).get_uniform_location($program, $u_name),
                format!(
                    "Unable to get uniform location of parameter \"{}\"",
                    $u_name
                ),
                is_valid_u_location
            )
        }
    };
}

pub struct UniformLocations {
    pub u_viewport: WebGlUniformLocation,
    pub u_position: WebGlUniformLocation,
    pub u_rotation: WebGlUniformLocation,
    pub u_scale: WebGlUniformLocation,
    pub u_color: WebGlUniformLocation,
}

/// Struct which stores WebGl program and locations of associated shaders
pub struct Program {
    pub program: WebGlProgram,
    pub uniform_locations: UniformLocations,
    pub a_vertex_location: u32,
}

impl Program {
    fn compile_shader(
        gl: &WebGlRenderingContext,
        src: &str,
        shader_type: u32,
    ) -> Option<WebGlShader> {
        let shader = try_unwrap!(
            @dev;
            gl.create_shader(shader_type),
            "Unable to create shader"
        );

        gl.shader_source(&shader, src);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .is_falsy()
        {
            return None;
        }

        Some(shader)
    }

    fn init_program(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Option<WebGlProgram> {
        let vertex_shader = try_unwrap!(
            @dev;
            Self::compile_shader(
                gl,
                vertex_shader_src,
                WebGlRenderingContext::VERTEX_SHADER
            ),
            "Unable to compile vertex shader"
        );

        let fragment_shader = try_unwrap!(
            @dev;
            Self::compile_shader(
                gl,
                vertex_shader_src,
                WebGlRenderingContext::VERTEX_SHADER
            ),
            "Unable to compile fragment shader"
        );

        let program = try_unwrap!(
            @dev;
            gl.create_program(),
            "Unable to create program"
        );

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .is_falsy()
        {
            return None;
        }

        Some(program)
    }

    fn retrieve_uniforms(gl: &WebGlRenderingContext, program: &WebGlProgram) -> Option<UniformLocations> {
        let u_viewport = try_locate_uniform!(gl, program, constants::u_viewport);
        let u_position = try_locate_uniform!(gl, program, constants::u_position);
        let u_rotation = try_locate_uniform!(gl, program, constants::u_rotation);
        let u_scale = try_locate_uniform!(gl, program, constants::u_scale);
        let u_color = try_locate_uniform!(gl, program, constants::u_color);

        Some(UniformLocations {
            u_viewport,
            u_position,
            u_rotation,
            u_scale,
            u_color
        })
    }

    pub fn init(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Option<Self> {
        let program = try_unwrap!(
            @dev;
            Self::init_program(gl, vertex_shader_src, fragment_shader_src),
            "Unable to initialize program"
        );

        let vertex_attribute_location = try_unwrap!(
            @dev;
            Some(gl.get_attrib_location(&program, constants::a_vertex)),
            "Unable to find location of attribute \"a_vertex\"",
            |v| *v >= 0
        );

        let uniform_locations = try_unwrap!(
            @dev;
            Self::retrieve_uniforms(gl, &program),
            "Unable to retrieve uniforms"
        );

        Some(Self {
            program,
            a_vertex_location: vertex_attribute_location as u32,
            uniform_locations
        })
    }
}
