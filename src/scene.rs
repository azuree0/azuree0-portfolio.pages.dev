// Underwater / Subnautica-style WebGL2 particle scene
// Deep ocean blues, bioluminescent particles (bubbles, plankton)
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use yew::prelude::*;

// Deep ocean background: rgb(0.04, 0.07, 0.12) = #0a121f
// Bioluminescent cyan: rgb(0.06, 0.67, 0.78) = #0eacc7

const VERTEX_SHADER: &str = r#"
#version 300 es
in vec2 a_position;
in float a_size;
in float a_alpha;
out float v_alpha;
void main() {
    v_alpha = a_alpha;
    gl_Position = vec4(a_position, 0.0, 1.0);
    gl_PointSize = a_size;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 300 es
precision mediump float;
in float v_alpha;
out vec4 fragColor;
void main() {
    vec2 c = gl_PointCoord - 0.5;
    float d = length(c);
    float a = v_alpha * (1.0 - smoothstep(0.3, 0.5, d));
    fragColor = vec4(0.06, 0.67, 0.78, a);
}
"#;

pub struct Scene3d;

impl Component for Scene3d {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id="scene-canvas" />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let document = web_sys::window()
            .and_then(|w| w.document())
            .expect("no document");
        let canvas = document
            .get_element_by_id("scene-canvas")
            .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
            .expect("canvas not found");

        if let Err(e) = init_and_run(&canvas) {
            web_sys::console::error_1(&format!("Scene init error: {}", e).into());
            // WebGL2 fallback: show gradient via canvas 2D or body background
            apply_fallback_gradient();
        }
    }
}

fn init_and_run(canvas: &HtmlCanvasElement) -> Result<(), String> {
    let gl = canvas
        .get_context("webgl2")
        .map_err(|_| "WebGL2 not supported")?
        .and_then(|c| c.dyn_into::<WebGl2RenderingContext>().ok())
        .ok_or("WebGL2 context failed")?;

    let program = compile_shaders(&gl)?;
    gl.use_program(Some(&program));

    let vao = gl
        .create_vertex_array()
        .ok_or("create VAO failed")?;
    gl.bind_vertex_array(Some(&vao));

    let buffer = gl.create_buffer().ok_or("create buffer failed")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
    let size_loc = gl.get_attrib_location(&program, "a_size") as u32;
    let alpha_loc = gl.get_attrib_location(&program, "a_alpha") as u32;

    gl.enable_vertex_attrib_array(pos_loc);
    gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 16, 0);
    gl.enable_vertex_attrib_array(size_loc);
    gl.vertex_attrib_pointer_with_i32(size_loc, 1, WebGl2RenderingContext::FLOAT, false, 16, 8);
    gl.enable_vertex_attrib_array(alpha_loc);
    gl.vertex_attrib_pointer_with_i32(alpha_loc, 1, WebGl2RenderingContext::FLOAT, false, 16, 12);

    let particle_count = 400;
    let mut particles: Vec<f32> = Vec::with_capacity(particle_count * 4);
    for i in 0..particle_count {
        let x = (i as f32 * 0.618033989).fract() * 2.0 - 1.0;
        let y = (i as f32 * 0.381966011).fract() * 2.0 - 1.0;
        let size = 2.0 + (i as f32 * 0.01).fract() * 4.0;
        let alpha = 0.2 + (i as f32 * 0.07).fract() * 0.4;
        particles.extend_from_slice(&[x, y, size, alpha]);
    }

    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(particles.as_slice()),
        WebGl2RenderingContext::DYNAMIC_DRAW,
    );

    let start = js_sys::Date::now();
    let canvas = canvas.clone();
    let gl = Rc::new(gl);

    let animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();
    let gl_clone = gl.clone();
    let canvas_clone = canvas.clone();

    let closure = Closure::wrap(Box::new(move || {
        let t = (js_sys::Date::now() - start) / 1000.0;

        resize_canvas(&canvas_clone);
        gl_clone.viewport(0, 0, canvas_clone.width() as i32, canvas_clone.height() as i32);

        gl_clone.clear_color(0.04, 0.07, 0.12, 1.0);
        gl_clone.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let mut particles: Vec<f32> = Vec::with_capacity(particle_count * 4);
        let t32 = t as f32;
        for i in 0..particle_count {
            let phase = (i as f32 * 0.1) + t32 * 0.3;
            let x = (phase * 0.5).sin() * 0.8 + ((i as f32 * 0.03).fract() - 0.5) * 0.4;
            let y = (phase * 0.7).cos() * 0.8 + ((i as f32 * 0.05).fract() - 0.5) * 0.3;
            let size = 2.0 + (phase * 2.0).sin().abs() * 3.0;
            let alpha = 0.15 + (phase * 1.5).sin().abs() * 0.25;
            particles.extend_from_slice(&[x, y, size, alpha]);
        }

        gl_clone.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::from(particles.as_slice()),
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );
        gl_clone.draw_arrays(WebGl2RenderingContext::POINTS, 0, particle_count as i32);

        let closure_borrow = animate_clone.borrow();
        if let Some(ref c) = *closure_borrow {
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(c.as_ref().unchecked_ref());
        }
    }) as Box<dyn FnMut()>);

    let _ = web_sys::window()
        .unwrap()
        .request_animation_frame(closure.as_ref().unchecked_ref());

    *animate.borrow_mut() = Some(closure);

    Ok(())
}

fn apply_fallback_gradient() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            let _ = body.set_attribute(
                "style",
                "background: linear-gradient(180deg, #0a0a0f 0%, #0d1b2a 50%, #1d395e 100%);",
            );
        }
    }
}

fn resize_canvas(canvas: &HtmlCanvasElement) {
    let dpr = web_sys::window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0);
    let width = (canvas.client_width() as f64 * dpr) as u32;
    let height = (canvas.client_height() as f64 * dpr) as u32;
    if canvas.width() != width || canvas.height() != height {
        canvas.set_width(width);
        canvas.set_height(height);
    }
}

fn compile_shaders(gl: &WebGl2RenderingContext) -> Result<web_sys::WebGlProgram, String> {
    let vs = gl
        .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
        .ok_or("create vs failed")?;
    gl.shader_source(&vs, VERTEX_SHADER);
    gl.compile_shader(&vs);
    if !gl.get_shader_parameter(&vs, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let log = gl.get_shader_info_log(&vs).unwrap_or_default();
        return Err(format!("VS compile: {}", log));
    }

    let fs = gl
        .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
        .ok_or("create fs failed")?;
    gl.shader_source(&fs, FRAGMENT_SHADER);
    gl.compile_shader(&fs);
    if !gl.get_shader_parameter(&fs, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let log = gl.get_shader_info_log(&fs).unwrap_or_default();
        return Err(format!("FS compile: {}", log));
    }

    let program = gl.create_program().ok_or("create program failed")?;
    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);
    if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let log = gl.get_program_info_log(&program).unwrap_or_default();
        return Err(format!("Link: {}", log));
    }

    Ok(program)
}
