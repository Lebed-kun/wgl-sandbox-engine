use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer,
    WebGlShader, 
    WebGlUniformLocation, 
    WebGlRenderingContext, 
    WebGlProgram
};

struct Uniform<T> {
    pub location: WebGlUniformLocation,
    pub value: T,
}

type Matrix4x4 = [[f32; 4]; 4];

struct Uniforms {
    pub viewport_x_scale: Uniform<f32>,
    pub viewport_y_scale: Uniform<f32>,
    pub position: Uniform<Matrix4x4>,
    pub rotation: Uniform<Matrix4x4>,
    pub scale: Uniform<Matrix4x4>
}

pub struct Form {
    uniforms: Uniforms
}

impl Form {
    fn compile_shader(
        gl: &WebGlRenderingContext,
        src: &str,
        shader_type: u32
    ) -> Option<WebGlShader> {
        let shader = gl.create_shader(shader_type);
        if shader.is_none() {
            return None;
        }
        let shader = shader.unwrap();

        gl.shader_source(
            &shader,
            src
        );
        gl.compile_shader(&shader);
        
        if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).is_falsy() {
            return None;
        }

        return Some(shader);
    }

    fn init_program(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str
    ) -> Option<WebGlProgram> {
        let vertex_shader = Self::compile_shader(
            gl, 
            vertex_shader_src, 
            WebGlRenderingContext::VERTEX_SHADER
        );
        if vertex_shader.is_none() {
            return None;
        }
        let vertex_shader = vertex_shader.unwrap();

        let fragment_shader = Self::compile_shader(
            gl, 
            vertex_shader_src, 
            WebGlRenderingContext::VERTEX_SHADER
        );
        if fragment_shader.is_none() {
            return None;
        }
        let fragment_shader = fragment_shader.unwrap();

        let program = gl.create_program();
        if program.is_none() {
            return None;
        }
        let program = program.unwrap();

        gl.attach_shader(
            &program,
            &vertex_shader
        );
        gl.attach_shader(
            &program,
            &fragment_shader
        );
        gl.link_program(&program);

        if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).is_falsy() {
            return None;
        }

        return Some(program);
    }
}