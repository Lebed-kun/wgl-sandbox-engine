use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{form::Form, program::Program, types::DrawProps, types::XYTuple};
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};

const example_vertex_shader: &'static str = "
    //VERTEX SHADER

    uniform vec4 u_viewport;
    uniform mat4 u_position;
    uniform mat4 u_rotation;
    uniform mat4 u_scale;

    attribute vec4 a_vertex;

    void main() {
        vec4 norm_vertex = u_scale * a_vertex;
        norm_vertex = u_rotation * norm_vertex;
        norm_vertex = u_position * norm_vertex;

        gl_Position = (
            vec4(
                norm_vertex.x * 2.0 - u_viewport.x, 
                -norm_vertex.y * 2.0 + u_viewport.y, 
                1.0, 
                1.0
            )
        ) / u_viewport;
    }
";

const example_fragment_shader: &'static str = "
    //FRAGMENT SHADER
    precision mediump float;
    uniform vec4 u_color;

    void main() {
        gl_FragColor = u_color;
    }
";

pub fn example() -> Result<(), JsValue> {
    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("root")
        .unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

    // Don't know but this hack worked fine
    log!("{:?}", &canvas);

    let context = canvas.get_context("webgl").unwrap().unwrap();
    let context = context.dyn_into::<WebGlRenderingContext>()?;

    let program = Program::init(&context, example_vertex_shader, example_fragment_shader);
    let program = program.unwrap();

    let px_projection = 1.0 as f32;
    let vertex_data: [f32; 24] = [
        -px_projection, -px_projection, 1.0, 1.0, 
        px_projection, -px_projection, 1.0, 1.0, 
        -px_projection, px_projection, 1.0, 1.0, // Triangle 1
        px_projection, -px_projection, 1.0, 1.0, 
        -px_projection, px_projection, 1.0, 1.0, 
        px_projection, px_projection, 1.0, 1.0, // Triangle 2
    ];


    let mut form1 = Form::init(&context, &vertex_data, &program).unwrap();
    let draw_props1 = DrawProps {
        position: XYTuple { x: 100.0, y: 100.0 },
        rotation: 0.0,
        scale: XYTuple { x: 100.0, y: 100.0 },
        color: [0.7, 0.33, 0.1, 1.0],
    };

    let mut form2 = Form::init(&context, &vertex_data, &program).unwrap();
    let draw_props2 = DrawProps {
        position: XYTuple { x: 400.0, y: 400.0 },
        rotation: std::f32::consts::PI / 4.0,
        scale: XYTuple { x: 150.0, y: 100.0 },
        color: [0.0, 0.33, 0.1, 1.0],
    };

    form1.prepare([800.0, 800.0, 1.0, 1.0], &draw_props1);
    form2.prepare([800.0, 800.0, 1.0, 1.0], &draw_props2);

    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    form1.draw(&context);
    form2.draw(&context);

    Ok(())
}
