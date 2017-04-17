use std::default::Default;
use std::time::Instant;

use glium;
use piston::input::{Input, Button, Motion};
use piston::input::keyboard::Key;
use piston::input::mouse::MouseButton;
use glium::Surface;
use glium::backend::Facade;
use imgui::glium_renderer::{Renderer, RendererResult};
use imgui::*;

use states::States;
use states::RenderMode;

mod gui {
    use super::*;

    pub fn build_ui(ui: &Ui, states: &mut States) {
        if ui.collapsing_header(im_str!("Hello")).build() {
            build_hello_panel(ui, states);
        }
        if ui.collapsing_header(im_str!("Edit")).build() {
            build_edit_panel(ui, states);
        }
        if ui.collapsing_header(im_str!("Parameters")).build() {
            build_parameters_control_panel(ui, states);
        }
        if ui.collapsing_header(im_str!("View")).build() {
            build_view_panel(ui, states);
        }
        if ui.collapsing_header(im_str!("Brush")).build() {
            build_brush_panel(ui, states);
        }

        build_stroke_manipulation_panel(ui, states);
    }

    fn build_hello_panel(ui: &Ui, states: &States) {
        ui.text(im_str!("Hello world!"));
        ui.text(im_str!("This...is...imgui-rs!"));
        ui.separator();

        let mouse_pos = ui.imgui().mouse_pos();
        ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        ui.text(im_str!("is recording: {}", states.is_recording_trajectory));
        ui.text(im_str!("recording cooldown: {}", states.current_recording_cooldown));
        ui.text(im_str!("recording num: {}", states.stroke_records.len()));
    }

    fn build_edit_panel(ui: &Ui, states: &mut States) {
        if ui.button(im_str!("clear all"), ImVec2::new(0., 0.)) {
            states.stroke_records.clear();
        }

        if ui.button(im_str!("clear last"), ImVec2::new(0., 0.)) {
            states.stroke_records.pop();
        }

        if ui.button(im_str!("update preview"), ImVec2::new(0., 0.)) {
            states.need_update_brush_preview = true;
        }
    }

    fn build_view_panel(ui: &Ui, states: &mut States) {
        {
            let mut render_mode_i32 = states.render_mode as i32;

            if ui.combo(im_str!("render mode"),
                        &mut render_mode_i32,
                        &[im_str!("black & white"), im_str!("colored")],
                        10) {

                states.need_update_brush_preview = true;
            }

            match render_mode_i32 {
                0 => states.render_mode = RenderMode::BlackAndWhite,
                1 => states.render_mode = RenderMode::Colored,
                _ => panic!("should not happen"),
            }
        }

        ui.checkbox(im_str!("show anchors"), &mut states.show_anchors);
        ui.checkbox(im_str!("show lines"), &mut states.show_stroke_lines);

        let need_update = &mut states.need_update_brush_preview;
        *need_update |= ui.checkbox(im_str!("show brush preview"),
                                    &mut states.show_brush_preview);
        *need_update |= ui.checkbox(im_str!("show ink outline preview"),
                                    &mut states.show_stroke_outline_preview);
        *need_update |= ui.checkbox(im_str!("show ink quantity preview"),
                                    &mut states.show_ink_quantity_preview);
    }


    fn build_brush_panel(ui: &Ui, states: &mut States) {
        ui.color_edit4(im_str!("color"), &mut states.recording_stroke_anchors.color).build();
    }

    fn build_stroke_manipulation_panel(ui: &Ui, states: &mut States) {
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                // Build slider to manipulate stroke anchors
                for (stroke_index, stroke) in &mut states.stroke_records.iter_mut().enumerate() {
                    ui.text(im_str!("Stroke no.{}", stroke_index));
                    ui.separator();

                    let need_update = &mut states.need_update_brush_preview;
                    for (index, anchor) in stroke.anchors.iter_mut().enumerate() {
                        *need_update |= ui.slider_float(im_str!("{}-{}", stroke_index, index),
                                          &mut anchor.pressure,
                                          0.0,
                                          1.0)
                            .build();
                    }
                }
            });
    }

    fn build_parameters_control_panel(ui: &Ui, states: &mut States) {
        if ui.slider_float(im_str!("poke interval"),
                          &mut states.max_recording_cooldown,
                          0.033,
                          1.0)
            .build() {
            println!("jkflkfsdj");
        }

        let need_update = &mut states.need_update_brush_preview;
        *need_update |= ui.slider_float(im_str!("stroke line radius"),
                          &mut states.stroke_line_radius,
                          0.1,
                          1.0)
            .build();

        *need_update |= ui.slider_float(im_str!("max brush width"),
                          &mut states.max_brush_width,
                          1.0,
                          50.0)
            .build();

        *need_update |= ui.slider_float(im_str!("initial ink quantity"),
                          &mut states.initial_ink_quantity,
                          0.0,
                          100.0)
            .build();

        *need_update |= ui.slider_float(im_str!("ink quantity friction"),
                          &mut states.ink_quantity_friction,
                          0.001,
                          0.01)
            .build();

        *need_update |= ui.slider_float(im_str!("stroke speed factor"),
                          &mut states.stroke_speed_factor,
                          0.1,
                          5.0)
            .build();

        *need_update |= ui.slider_float(im_str!("stroke interpolation accuracy"),
                          &mut states.stroke_interpolation_accuracy,
                          1.,
                          50.)
            .build();
    }
}

fn setup_imgui_key(imgui: &mut ImGui) {
    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

#[derive(Default)]
struct MouseButtonStates {
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub struct Toolkits {
    imgui: ImGui,
    imgui_renderer: Renderer,

    last_frame: Instant,

    mouse_pos: (i32, i32),
    mouse_button_states: MouseButtonStates,
    mouse_wheel: f32,
}

impl Toolkits {
    pub fn new<F: Facade>(window: &F) -> RendererResult<Toolkits> {
        let mut imgui = ImGui::init();
        let imgui_renderer = Renderer::init(&mut imgui, window)?;

        setup_imgui_key(&mut imgui);

        let result = Toolkits {
            imgui: imgui,
            imgui_renderer: imgui_renderer,

            last_frame: Instant::now(),

            mouse_pos: (0, 0),
            mouse_button_states: MouseButtonStates::default(),
            mouse_wheel: 0.,
        };

        Ok(result)
    }

    pub fn handle_event(&mut self, event: &Input) {
        fn set_key(imgui: &mut ImGui, key: &Key, pressed: bool) {
            match key {
                &Key::Tab => imgui.set_key(0, pressed),
                &Key::Left => imgui.set_key(1, pressed),
                &Key::Right => imgui.set_key(2, pressed),
                &Key::Up => imgui.set_key(3, pressed),
                &Key::Down => imgui.set_key(4, pressed),
                &Key::PageUp => imgui.set_key(5, pressed),
                &Key::PageDown => imgui.set_key(6, pressed),
                &Key::Home => imgui.set_key(7, pressed),
                &Key::End => imgui.set_key(8, pressed),
                &Key::Delete => imgui.set_key(9, pressed),
                &Key::Backspace => imgui.set_key(10, pressed),
                &Key::Return => imgui.set_key(11, pressed),
                &Key::Escape => imgui.set_key(12, pressed),
                &Key::A => imgui.set_key(13, pressed),
                &Key::C => imgui.set_key(14, pressed),
                &Key::V => imgui.set_key(15, pressed),
                &Key::X => imgui.set_key(16, pressed),
                &Key::Y => imgui.set_key(17, pressed),
                &Key::Z => imgui.set_key(18, pressed),
                &Key::LCtrl | &Key::RCtrl => imgui.set_key_ctrl(pressed),
                &Key::LShift | &Key::RShift => imgui.set_key_shift(pressed),
                &Key::LAlt | &Key::RAlt => imgui.set_key_alt(pressed),
                _ => {}
            }
        };

        fn set_mouse_button(mouse_button_states: &mut MouseButtonStates,
                            mouse_button: &MouseButton,
                            pressed: bool) {
            match mouse_button {
                &MouseButton::Left => mouse_button_states.left = pressed,
                &MouseButton::Right => mouse_button_states.right = pressed,
                &MouseButton::Middle => mouse_button_states.middle = pressed,
                _ => {}
            }
        };

        match event {
            &Input::Press(Button::Keyboard(ref key)) => {
                set_key(&mut self.imgui, &key, true);
            }

            &Input::Release(Button::Keyboard(ref key)) => {
                set_key(&mut self.imgui, &key, false);
            }

            &Input::Move(Motion::MouseCursor(x, y)) => self.mouse_pos = (x as i32, y as i32),
            &Input::Press(Button::Mouse(ref button)) => {
                set_mouse_button(&mut self.mouse_button_states, button, true)
            }
            &Input::Release(Button::Mouse(ref button)) => {
                set_mouse_button(&mut self.mouse_button_states, button, false)
            }
            &Input::Move(Motion::MouseScroll(_x, y)) => {
                self.mouse_wheel = y as f32;
            }

            &Input::Text(ref string) => {
                if let Some(c) = string.chars().last() {
                    self.imgui.add_input_character(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<S: Surface>(&mut self,
                              surface: &mut S,
                              window: &glium::glutin::Window,
                              states: &mut States) {
        let delta = self.get_time_elapse();

        self.update_mouse();

        let (size_points, size_pixels) = {
            let size_points = window.get_inner_size_points().unwrap();
            let size_pixels = window.get_inner_size_pixels().unwrap();

            ((size_points), size_pixels)
        };
        let ui = self.imgui.frame(size_points, size_pixels, delta);

        gui::build_ui(&ui, states);

        self.imgui_renderer.render(surface, ui).unwrap();
    }
}

impl Toolkits {
    fn get_time_elapse(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now - self.last_frame;

        self.last_frame = now;

        delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0
    }

    fn update_mouse(&mut self) {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui.set_mouse_pos(self.mouse_pos.0 as f32 / scale.0,
                                 self.mouse_pos.1 as f32 / scale.1);
        self.imgui.set_mouse_down(&[self.mouse_button_states.left,
                                    self.mouse_button_states.right,
                                    self.mouse_button_states.middle,
                                    false,
                                    false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel / scale.1);
        self.mouse_wheel = 0.0;
    }
}
