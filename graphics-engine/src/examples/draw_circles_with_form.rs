use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{form::Form, program::Program, types::DrawProps, types::XYTuple};
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};
use std::rc::Rc;

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

fn gen_vertex_data() -> Box<[f32]> {
    const max_viewport_size: f32 = 800.0;
    const min_viewport_size: f32 = 800.0;
    let origin = XYTuple {
        x: 0.0,
        y: 0.0,
    };
    const max_radius: f32 = 1000.0;
    const steps: i32 = (max_radius * 2.0 * std::f32::consts::PI / 8.0) as i32;
    
    let mut buffer_data: Box<[f32]> = Box::new(
        [0.0; ((steps as f32) * 4.0 * 3.0) as usize]
    );

    let mut i = 0_usize;
    let angle_step = 2.0 * std::f32::consts::PI / (steps as f32);
    let mut angle: f32 = 0.0;
    let radius = 1.0;
    for _ in 0..steps {
        // origin vertex
        buffer_data[i] = origin.x;
        buffer_data[i + 1] = origin.y;
        buffer_data[i + 2] = 1.0;
        buffer_data[i + 3] = 1.0;

        // current vertex
        buffer_data[i + 4] = origin.x + radius * angle.cos();
        buffer_data[i + 5] = origin.y - radius * angle.sin();
        buffer_data[i + 6] = 1.0;
        buffer_data[i + 7] = 1.0;

        // next vertex
        angle += angle_step;
        buffer_data[i + 8] = origin.x + radius * angle.cos();
        buffer_data[i + 9] = origin.y - radius * angle.sin();
        buffer_data[i + 10] = 1.0;
        buffer_data[i + 11] = 1.0;

        i += 12;
    }

    buffer_data
}

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

    let vertex_data = gen_vertex_data();

    let mut form1 = Form::init(&context, vertex_data.as_ref(), &program).unwrap();
    let draw_props1 = DrawProps {
        position: XYTuple { x: 200.0, y: 200.0 },
        rotation: 0.0,
        scale: XYTuple { x: 200.0, y: 200.0 },
        color: [0.7, 0.33, 0.1, 1.0],
    };

    let mut form2 = Form::init(&context, vertex_data.as_ref(), &program).unwrap();
    let draw_props2 = DrawProps {
        position: XYTuple { x: 400.0, y: 400.0 },
        rotation: std::f32::consts::PI / 4.0,
        scale: XYTuple { x: 200.0, y: 100.0 },
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
