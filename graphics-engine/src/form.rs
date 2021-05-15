use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

struct Uniform<T> {
    pub location: WebGlUniformLocation,
    pub value: T,
}

struct Attribute<T> {
    pub location: i32,
    pub value: T,
}

type Vector4 = [f32; 4];
type Matrix4x4 = [Vector4; 4];

struct Uniforms {
    pub viewport_x_scale: Uniform<f32>,
    pub viewport_y_scale: Uniform<f32>,
    pub position: Uniform<Matrix4x4>,
    pub rotation: Uniform<Matrix4x4>,
    pub scale: Uniform<Matrix4x4>,
    pub color: Uniform<Vector4>,
}

mod constants {
    use super::{Matrix4x4, Vector4};

    pub const u_viewport_x_scale: &'static str = "u_viewport_x_scale";
    pub const u_viewport_y_scale: &'static str = "u_viewport_y_scale";
    pub const u_position: &'static str = "u_position";
    pub const u_rotation: &'static str = "u_rotation";
    pub const u_scale: &'static str = "u_scale";
    pub const u_color: &'static str = "u_color";

    pub const a_vertex: &'static str = "a_vertex";

    pub const default_matrix4x4: Matrix4x4 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    pub const default_vector4: Vector4 = [0.0, 0.0, 0.0, 0.0];
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

// TODO: profile memory size for call stack optimization (?)
pub struct Form {
    uniforms: Uniforms,
    vertex_attribute: Attribute<Vector4>,
    vertex_buffer: WebGlBuffer,
    vertex_data: Vec<f32>,
}

impl Form {
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

    fn retrieve_uniforms(gl: &WebGlRenderingContext, program: &WebGlProgram) -> Option<Uniforms> {
        let u_viewport_x_scale = try_locate_uniform!(gl, program, constants::u_viewport_x_scale);
        let u_viewport_y_scale = try_locate_uniform!(gl, program, constants::u_viewport_y_scale);
        let u_position = try_locate_uniform!(gl, program, constants::u_position);
        let u_rotation = try_locate_uniform!(gl, program, constants::u_rotation);
        let u_scale = try_locate_uniform!(gl, program, constants::u_scale);
        let u_color = try_locate_uniform!(gl, program, constants::u_color);

        Some(Uniforms {
            viewport_x_scale: Uniform {
                location: u_viewport_x_scale,
                value: 0.0,
            },
            viewport_y_scale: Uniform {
                location: u_viewport_y_scale,
                value: 0.0,
            },
            position: Uniform {
                location: u_position,
                value: constants::default_matrix4x4,
            },
            rotation: Uniform {
                location: u_rotation,
                value: constants::default_matrix4x4,
            },
            scale: Uniform {
                location: u_scale,
                value: constants::default_matrix4x4,
            },
            color: Uniform {
                location: u_color,
                value: constants::default_vector4,
            },
        })
    }

    pub fn init(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        vertex_data: Vec<f32>,
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
        let vertex_buffer = try_unwrap!(
            @dev;
            gl.create_buffer(),
            "Unable to find location of attribute \"a_vertex\""
        );

        let uniforms = try_unwrap!(
            @dev;
            Self::retrieve_uniforms(gl, &program),
            "Unable to retrieve uniforms"
        );

        Some(Self {
            uniforms,
            vertex_attribute: Attribute {
                location: vertex_attribute_location,
                value: constants::default_vector4
            },
            vertex_buffer,
            vertex_data
        })
    }
}
