#[macro_use]
extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate image;
#[macro_use]
extern crate imgui;
extern crate interpolation;
extern crate piston;
extern crate vecmath;
extern crate rand;

mod toolkits;
mod states;

use std::io::Read;
use std::io::Cursor;

use piston::window::Size;
use glium::Blend;
use glium::Smooth;
use piston::window::Window;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::draw_parameters::DrawParameters;
use glium::VertexBuffer;
use glium::Surface;
use glium::texture::texture2d::Texture2d;
use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};
use graphics::Graphics;
use graphics::types::Matrix2d;
use graphics::math;
use piston::event_loop::EventLoop;
use piston::window::WindowSettings;
use glium::Program;

use states::StrokeAnchor;
use states::RenderMode;
use states::OneStroke;

const OPENGL: OpenGL = OpenGL::V3_2;

fn load_string(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    buf
}

fn load_texture<F: glium::backend::Facade>(window: &F, bytes: &[u8]) -> Texture2d {
    let image = image::load(Cursor::new(&bytes[..]), image::PNG)
        .unwrap()
        .to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);

    glium::texture::Texture2d::new(window, image).unwrap()
}

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
}
implement_vertex!(Vertex, pos);

#[derive(Copy, Clone)]
struct NormalVertex {
    pos: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(NormalVertex, pos, tex_coords);

#[derive(Copy, Clone)]
struct CircleData {
    pos: [f32; 2],
    center: [f32; 2],
    radius: f32,
}
implement_vertex!(CircleData, pos, center, radius);

struct App {
    window: GliumWindow,
    toolkits: toolkits::Toolkits,

    states: states::States,

    final_vertex_buffer: VertexBuffer<NormalVertex>,

    brush_preview_tex: Texture2d,
    stroke_outline_tex: Texture2d,
    stroke_outline_tmp_tex: Texture2d,
    stroke_ink_quantity_tex: Texture2d,
    stroke_ink_quantity_tmp_tex: Texture2d,

    // Watercolor textures.
    shallow_water_tex: Texture2d,
    pidment_deposition_tex: Texture2d,
    capillary_tex: Texture2d,
    passive_layer_tex: Texture2d,

    wipe_tmp_tex: Texture2d,
    diffusion_tmp_tex: Texture2d,

    final_program: Program,
    stroke_ink_quantity_program: Program,
    circle_program: Program,
    triangle_program: Program,
    black_n_white_brush_program: Program,
    watercolor_brush_program: Program,
    wipe_program: Program,
    diffusion_program: Program,

    rust_logo: Texture,
    doge_image: Texture2d,

    level0_tex: Texture2d,
    level1_tex: Texture2d,
    level2_tex: Texture2d,
    level3_tex: Texture2d,
    level4_tex: Texture2d,
}

impl App {
    pub fn new() -> Self {
        let (w, h) = (900, 900);
        let mut window: GliumWindow = WindowSettings::new("npr homework1", [w, h])
            .exit_on_esc(true)
            .opengl(OPENGL)
            .build()
            .unwrap();
        window.set_ups(60);

        let toolkits = toolkits::Toolkits::new(&window).unwrap();

        let final_vertex_buffer = glium::VertexBuffer::new(&window,
                                                           &[NormalVertex {
                                                                 pos: [-1.0, 1.0],
                                                                 tex_coords: [0.0, 1.0],
                                                             },
                                                             NormalVertex {
                                                                 pos: [1.0, 1.0],
                                                                 tex_coords: [1.0, 1.0],
                                                             },
                                                             NormalVertex {
                                                                 pos: [-1.0, -1.0],
                                                                 tex_coords: [0.0, 0.0],
                                                             },
                                                             NormalVertex {
                                                                 pos: [1.0, -1.0],
                                                                 tex_coords: [1.0, 0.0],
                                                             }])
            .unwrap();

        let rust_logo = Texture::from_path(&mut window,
                                           "assets/rust.png",
                                           Flip::None,
                                           &TextureSettings::new())
            .unwrap();



        let final_program = Program::from_source(&window,
                                                 &load_string("shaders/final.vs"),
                                                 &load_string("shaders/final.fs"),
                                                 None)
            .expect("failed to initialize textured shader");
        let circle_program = Program::from_source(&window,
                                                  &load_string("shaders/final.vs"),
                                                  &load_string("shaders/circle.fs"),
                                                  None)
            .expect("failed to initialize textured shader");
        let triangle_program = Program::from_source(&window,
                                                    &load_string("shaders/triangle.vs"),
                                                    &load_string("shaders/triangle.fs"),
                                                    None)
            .expect("failed to initialize textured shader");
        let stroke_ink_quantity_program = Program::from_source(&window,
                                                               &load_string("shaders/final.vs"),
                                                               &load_string("shaders/ink.fs"),
                                                               None)
            .expect("failed to initialize textured shader");
        let black_n_white_brush_program =
            Program::from_source(&window,
                                 &load_string("shaders/final.vs"),
                                 &load_string("shaders/black_n_white_brush.fs"),
                                 None)
                .expect("failed to initialize textured shader");
        let watercolor_brush_program = Program::from_source(&window,
                                                            &load_string("shaders/final.vs"),
                                                            &load_string("shaders/watercolor_brush.\
                                                                          fs"),
                                                            None)
            .expect("failed to initialize textured shader");
        let wipe_program = Program::from_source(&window,
                                                &load_string("shaders/final.vs"),
                                                &load_string("shaders/wipe.fs"),
                                                None)
            .expect("failed to initialize textured shader");
        let diffusion_program = Program::from_source(&window,
                                                     &load_string("shaders/final.vs"),
                                                     &load_string("shaders/diffusion.fs"),
                                                     None)
            .expect("failed to initialize textured shader");

        let doge_image = load_texture(&window, include_bytes!("assets/doge.png"));

        let level0_tex = load_texture(&window, include_bytes!("assets/level0.png"));
        let level1_tex = load_texture(&window, include_bytes!("assets/level1.png"));
        let level2_tex = load_texture(&window, include_bytes!("assets/level2.png"));
        let level3_tex = load_texture(&window, include_bytes!("assets/level3.png"));
        let level4_tex = load_texture(&window, include_bytes!("assets/level4.png"));

        App {
            final_vertex_buffer: final_vertex_buffer,

            brush_preview_tex: Texture2d::empty(&window, w, h).unwrap(),
            stroke_outline_tex: Texture2d::empty(&window, w, h).unwrap(),
            stroke_outline_tmp_tex: Texture2d::empty(&window, w, h).unwrap(),
            stroke_ink_quantity_tex: Texture2d::empty(&window, w, h).unwrap(),
            stroke_ink_quantity_tmp_tex: Texture2d::empty(&window, w, h).unwrap(),
            black_n_white_brush_program: black_n_white_brush_program,
            watercolor_brush_program: watercolor_brush_program,

            shallow_water_tex: Texture2d::empty(&window, w, h).unwrap(),
            pidment_deposition_tex: Texture2d::empty(&window, w, h).unwrap(),
            capillary_tex: Texture2d::empty(&window, w, h).unwrap(),
            passive_layer_tex: Texture2d::empty(&window, w, h).unwrap(),

            wipe_tmp_tex: Texture2d::empty(&window, w, h).unwrap(),
            diffusion_tmp_tex: Texture2d::empty(&window, w, h).unwrap(),

            final_program: final_program,
            stroke_ink_quantity_program: stroke_ink_quantity_program,
            circle_program: circle_program,
            triangle_program: triangle_program,
            wipe_program: wipe_program,
            diffusion_program: diffusion_program,

            rust_logo: rust_logo,
            doge_image: doge_image,

            level0_tex: level0_tex,
            level1_tex: level1_tex,
            level2_tex: level2_tex,
            level3_tex: level3_tex,
            level4_tex: level4_tex,

            window: window,
            toolkits: toolkits,
            states: states::States::default(),
        }
    }

    pub fn run(mut self) -> Result<(), ()> {
        let mut g2d = Glium2d::new(OPENGL, &mut self.window);

        while let Some(event) = self.window.next() {
            use piston::input::*;

            self.toolkits.handle_event(&event);

            match event {
                Input::Update(ref args) => self.update(&args.dt),
                Input::Render(ref args) => self.render(args, &mut g2d),

                _ => self.handle_inputs(&event),
            }
        }

        Ok(())
    }

    fn update(&mut self, dt: &f64) {
        if self.states.is_recording_trajectory {
            self.states.current_recording_cooldown += *dt as f32;
        }
    }

    fn render(&mut self, args: &piston::input::RenderArgs, g2d: &mut Glium2d) {
        use graphics::*;

        let mut target = self.window.draw();

        g2d.draw(&mut target,
                 args.viewport(),
                 |_c, g| { clear(color::WHITE, g); });

        if self.states.need_update_brush_preview {
            self.states.need_update_brush_preview = false;

            self.construct_brush_tex();
        }

        if self.states.show_brush_preview {
            self.draw_texture_on(&self.brush_preview_tex, &mut target);
        }

        if self.states.show_stroke_outline_preview {
            self.draw_texture_on(&self.stroke_outline_tex, &mut target);
        }

        if self.states.show_ink_quantity_preview {
            self.draw_texture_on(&self.stroke_ink_quantity_tex, &mut target);
        }

        g2d.draw(&mut target, args.viewport(), |c, g| {
            self.render_rust_logo_and_demo(c.transform, g);
            self.render_stroke_anchor_points(c.transform, g);
        });

        {
            let window = &self.window.window.borrow().window;
            self.toolkits.render(&mut target, window, &mut self.states);
        }

        target.finish().unwrap();
    }

    fn draw_texture_on<S: Surface>(&self, tex: &Texture2d, target: &mut S) {
        let draw_state =
            glium::DrawParameters { blend: Blend::alpha_blending(), ..Default::default() };
        target.draw(&self.final_vertex_buffer,
                  &NoIndices(PrimitiveType::TriangleStrip),
                  &self.final_program,
                  &uniform!{
                        tex: tex,
                    },
                  &draw_state)
            .expect("failed to draw triangle list");
    }

    fn handle_inputs(&mut self, event: &piston::input::Input) {
        use piston::input::*;

        match event {
            &Input::Press(Button::Mouse(button)) => {
                match button {
                    MouseButton::Right => {
                        self.states.is_recording_trajectory = true;
                        self.states.current_recording_cooldown = 0.;
                    }
                    _ => {}
                }
            }

            &Input::Release(Button::Mouse(button)) => {
                match button {
                    MouseButton::Right => {
                        self.states.is_recording_trajectory = false;
                        self.states.need_update_brush_preview = true;

                        self.states
                            .stroke_records
                            .push(self.states.recording_stroke_anchors.clone());
                        self.states.recording_stroke_anchors.clear();
                    }
                    _ => {}
                }
            }

            &Input::Move(Motion::MouseCursor(x, y)) => {
                let states = &mut self.states;

                if states.is_recording_trajectory {
                    while states.current_recording_cooldown >= states.max_recording_cooldown {
                        states.current_recording_cooldown -= states.max_recording_cooldown;

                        let new_stroke_anchor = StrokeAnchor::new(&[x as f32, y as f32], 1.);
                        states.recording_stroke_anchors.add_anchor(new_stroke_anchor);
                    }
                }
            }

            _ => {}
        }
    }

    fn render_rust_logo_and_demo(&self,
                                 transform: Matrix2d,
                                 g: &mut glium_graphics::GliumGraphics<glium::Frame>) {
        use graphics::*;

        rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], transform, g);
        rectangle([0.0, 1.0, 0.0, 0.3],
                  [50.0, 50.0, 100.0, 100.0],
                  transform,
                  g);
        image(&self.rust_logo, transform.trans(100.0, 100.0), g);
    }

    fn render_stroke_anchor_points<G: Graphics>(&self, transform: Matrix2d, g: &mut G) {
        use graphics::*;
        use graphics::ellipse::circle;

        let mut draw_one_stroke = |one_stroke: &states::OneStroke| {
            let mut stroke_records_iter = one_stroke.anchors.iter();

            if let Some(mut prev_stroke_anchor) = stroke_records_iter.next() {
                for stroke_anchor in stroke_records_iter {
                    let prev_anchor_pos = &prev_stroke_anchor.pos;
                    let anchor_pos = &stroke_anchor.pos;

                    // Draw line between dots
                    if self.states.show_stroke_lines {
                        line([0., 0.2, 1., 1.0],
                             self.states.stroke_line_radius as f64,
                             [prev_anchor_pos[0] as f64,
                              prev_anchor_pos[1] as f64,
                              anchor_pos[0] as f64,
                              anchor_pos[1] as f64],
                             transform,
                             g);
                    }

                    // Draw dot
                    if self.states.show_anchors {
                        ellipse([1.0, 0.0, 0.0, 0.8],
                                circle(anchor_pos[0] as f64, anchor_pos[1] as f64, 3.0),
                                transform,
                                g);
                    }

                    prev_stroke_anchor = stroke_anchor;
                }
            }
        };

        for one_stroke in &self.states.stroke_records {
            draw_one_stroke(one_stroke);
        }

        draw_one_stroke(&self.states.recording_stroke_anchors);
    }

    fn render_circle(&self,
                     target_tex: &Texture2d,
                     center: [f32; 2],
                     radius: f32,
                     brush_color: &[f32; 4]) {
        target_tex.as_surface()
            .draw(&self.final_vertex_buffer,
                  &NoIndices(PrimitiveType::TriangleStrip),
                  &self.circle_program,
                  &uniform!{
                      center: center,
                      radius: radius,
                      brush_color: *brush_color,
                  },
                  &DrawParameters::default())
            .expect("failed to draw triangle list");
    }

    fn render_triangle_lists_on(&self,
                                triangles: &[Vertex],
                                target_tex: &Texture2d,
                                brush_color: &[f32; 4]) {
        let Size { width, height } = self.window.draw_size();

        let triangles: Vec<_> = triangles.iter()
            .map(|v| {
                Vertex {
                    pos: [2.0 * v.pos[0] / width as f32 - 1.0,
                          2.0 * (900.0 - v.pos[1]) / height as f32 - 1.0],
                }
            })
            .collect();

        let vertex_buffer = glium::VertexBuffer::new(&self.window, &triangles).unwrap();
        target_tex.as_surface()
            .draw(&vertex_buffer,
                  &NoIndices(PrimitiveType::TriangleStrip),
                  &self.triangle_program,
                  &uniform!{
                      brush_color: *brush_color,
                  },
                  &DrawParameters::default())
            .expect("failed to draw triangle list");
    }

    fn caculate_anchor_polygon(&self,
                               prev_stroke_anchor: &StrokeAnchor,
                               stroke_anchor: &StrokeAnchor)
                               -> [Vertex; 4] {
        let prev_anchor_pos = &prev_stroke_anchor.pos;
        let anchor_pos = &stroke_anchor.pos;

        let start_pos = math::cast([prev_anchor_pos[0], prev_anchor_pos[1]]);
        let end_pos = math::cast([anchor_pos[0], anchor_pos[1]]);

        let norm_v = vecmath::vec2_normalized([(anchor_pos[0] - prev_anchor_pos[0]) as f32,
                                               (anchor_pos[1] - prev_anchor_pos[1]) as f32]);

        let start_brush_width = self.caculate_brush_radius(prev_stroke_anchor.pressure);
        let end_brush_width = self.caculate_brush_radius(stroke_anchor.pressure);

        let start_v = math::mul_scalar(norm_v, start_brush_width);
        let end_v = math::mul_scalar(norm_v, end_brush_width);

        let rotate_right = math::rotate_radians(std::f32::consts::PI / 2.);
        let rotate_left = math::rotate_radians(std::f32::consts::PI / -2.);

        let start_a_mat = math::translate(math::transform_vec(rotate_left, start_v));
        let start_b_mat = math::translate(math::transform_vec(rotate_right, start_v));
        let end_a_mat = math::translate(math::transform_vec(rotate_left, end_v));
        let end_b_mat = math::translate(math::transform_vec(rotate_right, end_v));

        [Vertex { pos: math::transform_pos(start_a_mat, start_pos) },
         Vertex { pos: math::transform_pos(end_a_mat, end_pos) },
         Vertex { pos: math::transform_pos(start_b_mat, start_pos) },
         Vertex { pos: math::transform_pos(end_b_mat, end_pos) }]
    }

    fn render_stroke_ink_outline_tex(&self) {
        self.stroke_outline_tex.as_surface().clear_color(1.0, 1.0, 1.0, 1.0);

        let render_circle_part = |stroke_anchor: &StrokeAnchor, stroke_color| {
            let radius = self.caculate_brush_radius(stroke_anchor.pressure);
            self.render_circle(&self.stroke_outline_tmp_tex,
                               [stroke_anchor.pos[0], stroke_anchor.pos[1]],
                               radius,
                               &stroke_color);
        };

        // Render polygon part between each anchor.
        for stroke in &self.states.stroke_records {
            if stroke.anchors.is_empty() {
                continue;
            }

            // Texture used for storing new stroke.
            self.stroke_outline_tmp_tex.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

            let mut stroke_anchors_iter = stroke.anchors.iter();
            let mut prev_stroke_anchor = stroke_anchors_iter.next().unwrap();

            let stroke_color = match self.states.render_mode {
                RenderMode::Colored => stroke.color,
                _ => [1.0, 0.0, 0.0, 1.0],
            };

            // Draw circle of start anchor.
            render_circle_part(prev_stroke_anchor, stroke_color);

            // Draw outline form by all anchor.
            for stroke_anchor in stroke_anchors_iter {
                let polygon_points =
                    self.caculate_anchor_polygon(prev_stroke_anchor, stroke_anchor);

                render_circle_part(stroke_anchor, stroke_color);
                self.render_triangle_lists_on(&polygon_points,
                                              &self.stroke_outline_tmp_tex,
                                              &stroke_color);

                prev_stroke_anchor = &stroke_anchor;
            }

            // Wipe previous pigment on canvas according to current new stroke.
            self.wipe_pigment_by_stroke(&self.stroke_outline_tex, stroke);

            // Fake diffusion on canvas according to current new stroke.
            self.render_fake_stroke_diffusion(&self.stroke_outline_tex, stroke);

            // Blit new stroke onto previous canvas.
            self.draw_texture_on(&self.stroke_outline_tmp_tex,
                                 &mut self.stroke_outline_tex.as_surface());
        }
    }

    fn render_fake_stroke_diffusion(&self, canvas: &Texture2d, stroke: &OneStroke) {
        if stroke.anchors.is_empty() {
            return;
        }

        let mut canvas_surface = canvas.as_surface();
        let mut diffusion_tmp_surface = self.diffusion_tmp_tex.as_surface();

        let mut haha = |brush_color, radius_offset| {
            let mut stroke_anchors_iter = stroke.anchors.iter();
            let mut prev_stroke_anchor = stroke_anchors_iter.next().unwrap();

            diffusion_tmp_surface.clear_color(0.0, 0.0, 0.0, 0.0);

            for stroke_anchor in stroke_anchors_iter {
                let stroke_start_pos = &prev_stroke_anchor.pos;
                let stroke_end_pos = &stroke_anchor.pos;

                let start_radius = self.caculate_brush_radius(prev_stroke_anchor.pressure);
                let end_radius = self.caculate_brush_radius(stroke_anchor.pressure);

                canvas_surface.fill(&diffusion_tmp_surface,
                                    glium::uniforms::MagnifySamplerFilter::Nearest);

                canvas_surface.draw(&self.final_vertex_buffer,
                          &NoIndices(PrimitiveType::TriangleStrip),
                          &self.diffusion_program,
                          &uniform!{
                              current_tex: &self.diffusion_tmp_tex,

                              brush_color: brush_color,

                              stroke_start_pos: *stroke_start_pos,
                              stroke_end_pos: *stroke_end_pos,

                              start_radius: start_radius + radius_offset,
                              end_radius: end_radius + radius_offset,
                          },
                          &DrawParameters::default())
                    .expect("failed to draw triangle list");

                prev_stroke_anchor = stroke_anchor;
            }
        };

        let inner_brush_color = {
            let mut c = stroke.color;

            c[0] *= 0.7;
            c[1] *= 0.7;
            c[2] *= 0.7;

            c
        };
        let outter_brush_color = {
            let mut c = stroke.color;

            c[0] *= 1.3;
            c[1] *= 1.3;
            c[2] *= 1.3;

            c
        };

        haha(outter_brush_color, 10.0);
        haha(inner_brush_color, 2.5);
    }

    fn wipe_pigment_by_stroke(&self, canvas: &Texture2d, stroke: &OneStroke) {
        if stroke.anchors.is_empty() {
            return;
        }

        let mut wipe_tmp_surface = self.wipe_tmp_tex.as_surface();
        let mut canvas_surface = canvas.as_surface();

        wipe_tmp_surface.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut stroke_anchors_iter = stroke.anchors.iter();
        let mut prev_stroke_anchor = stroke_anchors_iter.next().unwrap();

        for stroke_anchor in stroke_anchors_iter {
            let stroke_start_pos = &prev_stroke_anchor.pos;
            let stroke_end_pos = &stroke_anchor.pos;

            let start_radius = self.caculate_brush_radius(prev_stroke_anchor.pressure);
            let end_radius = self.caculate_brush_radius(stroke_anchor.pressure);

            // Copy canvas to wipe_tmp_tex.
            canvas_surface.fill(&wipe_tmp_surface,
                                glium::uniforms::MagnifySamplerFilter::Nearest);

            // Do actual wipe opeartion.
            canvas_surface.draw(&self.final_vertex_buffer,
                      &NoIndices(PrimitiveType::TriangleStrip),
                      &self.wipe_program,
                      &uniform!{
                          current_tex: &self.wipe_tmp_tex,

                          stroke_start_pos: *stroke_start_pos,
                          stroke_end_pos: *stroke_end_pos,

                          start_radius: start_radius,
                          end_radius: end_radius,
                      },
                      &DrawParameters::default())
                .expect("failed to draw triangle list");

            prev_stroke_anchor = stroke_anchor;
        }
    }

    fn render_stroke_ink_quantity_tex(&self) {
        let draw = |start_pos: [f32; 2],
                    start_radius,
                    start_ink_quantity,
                    end_pos: [f32; 2],
                    end_radius,
                    end_ink_quantity| {

            self.stroke_ink_quantity_tex
                .as_surface()
                .draw(&self.final_vertex_buffer,
                      &NoIndices(PrimitiveType::TriangleStrip),
                      &self.stroke_ink_quantity_program,
                      &uniform!{
                          stroke_outline_tex: &self.stroke_outline_tex,
                          stroke_ink_quantity_tmp_tex: &self.stroke_ink_quantity_tmp_tex,

                          start_pos: start_pos,
                          end_pos: end_pos,

                          start_radius: start_radius,
                          end_radius: end_radius,

                          start_ink_quantity: start_ink_quantity,
                          end_ink_quantity: end_ink_quantity,
                      },
                      &DrawParameters::default())
                .expect("failed to draw triangle list");

            // Copy to tmp texture for future reference
            self.stroke_ink_quantity_tex
                .as_surface()
                .fill(&self.stroke_ink_quantity_tmp_tex.as_surface(),
                      glium::uniforms::MagnifySamplerFilter::Nearest);
        };

        self.stroke_ink_quantity_tex.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
        self.stroke_ink_quantity_tmp_tex.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

        for stroke in &self.states.stroke_records {
            let mut stroke_iter = stroke.anchors.iter();
            let mut current_ink_quantity = self.states.initial_ink_quantity;

            if let Some(mut prev_stroke_anchor) = stroke_iter.next() {
                for stroke_anchor in stroke_iter {
                    let start_pos = &prev_stroke_anchor.pos;
                    let end_pos = &stroke_anchor.pos;

                    let start_radius = self.caculate_brush_radius(prev_stroke_anchor.pressure);
                    let end_radius = self.caculate_brush_radius(stroke_anchor.pressure);

                    let ink_cost =
                        self.caculate_ink_cost(start_pos, start_radius, end_pos, end_radius);

                    let start_ink_quantity = current_ink_quantity /
                                             self.states.initial_ink_quantity;
                    let end_ink_quantity = (current_ink_quantity - ink_cost) /
                                           self.states.initial_ink_quantity;

                    current_ink_quantity -= ink_cost;

                    draw(*start_pos,
                         start_radius,
                         start_ink_quantity,
                         *end_pos,
                         end_radius,
                         end_ink_quantity);

                    prev_stroke_anchor = stroke_anchor;
                }
            }
        }
    }

    fn render_brush_tex(&self) {
        use glium::uniforms::Sampler;

        self.brush_preview_tex.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

        let apply_sampler = |tex| {
            Sampler::new(tex)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        self.brush_preview_tex
            .as_surface()
            .draw(&self.final_vertex_buffer,
                  &NoIndices(PrimitiveType::TriangleStrip),
                  &self.black_n_white_brush_program,
                  &uniform!{
                      stroke_ink_quantity_tex: &self.stroke_ink_quantity_tex,

                      level_0_brush_tex: apply_sampler(&self.level0_tex),
                      level_1_brush_tex: apply_sampler(&self.level1_tex),
                      level_2_brush_tex: apply_sampler(&self.level2_tex),
                      level_3_brush_tex: apply_sampler(&self.level3_tex),
                      level_4_brush_tex: apply_sampler(&self.level4_tex),
                  },
                  &glium::DrawParameters { smooth: Some(Smooth::Fastest), ..Default::default() })
            .expect("failed to draw triangle list");
    }

    fn render_watercolor_brush_tex(&self) {
        self.brush_preview_tex.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

        self.brush_preview_tex
            .as_surface()
            .draw(&self.final_vertex_buffer,
                  &NoIndices(PrimitiveType::TriangleStrip),
                  &self.watercolor_brush_program,
                  &uniform!{
                      stroke_outline_tex: &self.stroke_outline_tex,
                      stroke_ink_quantity_tex: &self.stroke_ink_quantity_tex,
                  },
                  &DrawParameters::default())
            .expect("failed to draw triangle list");
    }

    fn construct_brush_tex(&self) {
        // TODO Use struct to wrap these up.
        match self.states.render_mode {
            RenderMode::BlackAndWhite => {
                self.render_stroke_ink_outline_tex();
                self.render_stroke_ink_quantity_tex();
                self.render_brush_tex();
            }
            RenderMode::Colored => {
                self.render_stroke_ink_outline_tex();
                self.render_stroke_ink_quantity_tex();
                self.render_watercolor_brush_tex();
            }
        }
    }

    fn caculate_brush_radius(&self, pressure: f32) -> f32 {
        pressure * self.states.max_brush_width
    }

    fn caculate_ink_cost(&self,
                         start_pos: &[f32; 2],
                         start_radius: f32,
                         end_pos: &[f32; 2],
                         end_radius: f32)
                         -> f32 {
        let offset = vecmath::vec2_sub(*end_pos, *start_pos);
        let stroke_len = vecmath::vec2_len(offset);
        let area = (start_radius + end_radius) * stroke_len / 2.0;

        self.states.ink_quantity_friction * area
    }
}

fn main() {
    let app = App::new();

    match app.run() {
        Ok(_) => {}
        Err(_) => {}
    }
}
