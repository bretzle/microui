use std::fmt::Write;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use microui::*;
use microui::atlas::*;

use crate::renderer::Renderer;

mod renderer;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas lacinia, sem eu lacinia molestie, mi risus faucibus ipsum, eu varius magna felis a nulla.";

#[derive(Copy, Clone)]
pub struct LabelColor<'a> {
    pub label: &'a str,
    pub idx: ControlColor,
}

struct State {
    label_colors: [LabelColor<'static>; 15],
    bg: [f32; 3],
    logbuf: String,
    logbuf_updated: bool,
    submit_buf: String,
    checks: [bool; 3],
}

impl State {
    pub fn new() -> Self {
        Self {
            label_colors: [
                ("text", ControlColor::Text),
                ("border:", ControlColor::Border),
                ("windowbg:", ControlColor::WindowBG),
                ("titlebg:", ControlColor::TitleBG),
                ("titletext:", ControlColor::TitleText),
                ("panelbg:", ControlColor::PanelBG),
                ("button:", ControlColor::Button),
                ("buttonhover:", ControlColor::ButtonHover),
                ("buttonfocus:", ControlColor::ButtonFocus),
                ("base:", ControlColor::Base),
                ("basehover:", ControlColor::BaseHover),
                ("basefocus:", ControlColor::BaseFocus),
                ("scrollbase:", ControlColor::ScrollBase),
                ("scrollthumb:", ControlColor::ScrollThumb),
                ("", ControlColor::Text),
            ]
            .map(|(label, idx)| LabelColor { label, idx }),
            bg: [90.0, 95.0, 100.0],
            logbuf: String::new(),
            logbuf_updated: false,
            submit_buf: String::new(),
            checks: [false, true, false],
        }
    }

    fn write_log(&mut self, text: &str) {
        if !self.logbuf.is_empty() {
            self.logbuf.push('\n');
        }
        self.logbuf.push_str(text);
        self.logbuf_updated = true;
    }

    fn test_window(&mut self, ui: &mut Context) {
        ui.window("Demo Window").position(40, 40).size(300, 450).show(ui, |ui| {
            let mut win = ui.get_current_container_rect();
            win.w = if win.w > 240 { win.w } else { 240 };
            win.h = if win.h > 300 { win.h } else { 300 };

            ui.set_current_container_rect(&win);

            let mut buff = String::new();

            ui.header("Window Info").show(ui, |ui| {
                let win_0 = ui.get_current_container_rect();
                ui.layout_row(&[54, -1], 0);
                ui.label("Position:");

                buff.clear();
                let _ = write!(buff, "{}, {}", win_0.x, win_0.y);

                ui.label(&buff);
                buff.clear();
                ui.label("Size:");

                let _ = write!(buff, "{}, {}", win_0.w, win_0.h);

                ui.label(&buff);
            });

            ui.header("Test Buttons").expanded().show(ui, |ui| {
                ui.layout_row(&[86, -110, -1], 0);
                ui.label("Test buttons 1:");

                ui.button("Button 1").show(ui, |_| self.write_log("Pressed button 1"));

                ui.button("Button 2").show(ui, |_| self.write_log("Pressed button 2"));
                ui.label("Test buttons 2:");

                ui.button("Button 3").show(ui, |_| self.write_log("Pressed button 3"));

                ui.button("Popup").show(ui, |ui| ui.open_popup("Test Popup"));

                ui.popup("Test Popup").show(ui, |ui| {
                    ui.button("Hello").show(ui, |_| self.write_log("Hello"));
                    ui.button("World").show(ui, |_| self.write_log("World"));
                });
            });

            ui.header("Tree and Text").expanded().show(ui, |ui| {
                ui.layout_row(&[140, -1], 0);
                ui.layout_begin_column();
                ui.treenode("Test 1").show(ui, |ui| {
                    ui.treenode("Test 1a").show(ui, |ui| {
                        ui.label("Hello");
                        ui.label("world");
                    });
                    ui.treenode("Test 1b").show(ui, |ui| {
                        ui.button("Button 1").show(ui, |_| {
                            self.write_log("Pressed button 1");
                        });

                        ui.button("Button 2").show(ui, |_| {
                            self.write_log("Pressed button 2");
                        });
                    });
                });
                ui.treenode("Test 2").show(ui, |ui| {
                    ui.layout_row(&[54, 54], 0);
                    ui.button("Button 3").show(ui, |_| self.write_log("Pressed button 3"));
                    ui.button("Button 4").show(ui, |_| self.write_log("Pressed button 4"));
                    ui.button("Button 5").show(ui, |_| self.write_log("Pressed button 5"));
                    ui.button("Button 6").show(ui, |_| self.write_log("Pressed button 6"));
                });
                ui.treenode("Test 3").show(ui, |ui| {
                    ui.checkbox("Checkbox 1", &mut self.checks[0]);
                    ui.checkbox("Checkbox 2", &mut self.checks[1]);
                    ui.checkbox("Checkbox 3", &mut self.checks[2]);
                });
                ui.layout_end_column();
                ui.layout_begin_column();
                ui.layout_row(&[-1], 0);
                ui.text(LOREM_IPSUM);
                ui.layout_end_column();
            });

            ui.header("Background Color").expanded().show(ui, |ui| {
                ui.layout_row(&[-78, -1], 74);
                ui.layout_begin_column();
                ui.layout_row(&[46, -1], 0);
                ui.label("Red:");
                ui.slider_ex(&mut self.bg[0], 0.0, 255.0, 0.0, 0, WidgetOption::ALIGN_CENTER);
                ui.label("Green:");
                ui.slider_ex(&mut self.bg[1], 0.0, 255.0, 0.0, 0, WidgetOption::ALIGN_CENTER);
                ui.label("Blue:");
                ui.slider_ex(&mut self.bg[2], 0.0, 255.0, 0.0, 0, WidgetOption::ALIGN_CENTER);
                ui.layout_end_column();
                let r: Rect = ui.layout_next();
                ui.draw_rect(r, Color::rgb(self.bg[0] as u8, self.bg[1] as u8, self.bg[2] as u8));
                let buff = format!("#{:02x}{:02x}{:02x}", self.bg[0] as u32, self.bg[1] as u32, self.bg[2] as u32);
                ui.draw_control_text(buff.as_str(), r, ControlColor::Text, WidgetOption::ALIGN_CENTER);
            });
        });
    }

    fn log_window(&mut self, ui: &mut Context) {
        ui.window("Log Window").position(350, 40).size(300, 200).show(ui, |ui| {
            ui.layout_row(&[-1], -25);
            ui.panel("Log Output").show(ui, |ui| {
                let mut scroll = ui.get_current_container_scroll();
                let content_size = ui.get_current_container_content_size();
                ui.layout_row(&[-1], -1);
                ui.text(&self.logbuf);
                if self.logbuf_updated {
                    scroll.y = content_size.y;
                    ui.set_current_container_scroll(&scroll);
                    self.logbuf_updated = false;
                }
            });

            let mut submitted = false;
            ui.layout_row(&[-70, -1], 0);
            if ui.textbox_ex(&mut self.submit_buf, WidgetOption::empty()).is_submitted() {
                ui.set_focus(ui.last_id);
                submitted = true;
            }

            ui.button("Submit").show(ui, |_| submitted = true);

            if submitted {
                let mut buf = String::new();
                buf.push_str(self.submit_buf.as_str());
                self.write_log(&buf);
                self.submit_buf.clear();
            }
        });
    }

    fn style_window(&mut self, ui: &mut Context) {
        ui.window("Style Editor").position(350, 250).size(300, 240).show(ui, |ui| {
            let sw = (ui.get_current_container_body().w as f64 * 0.14) as i32;
            ui.layout_row(&[80, sw, sw, sw, sw, -1], 0);
            let mut i = 0;
            while !self.label_colors[i].label.is_empty() {
                ui.label(self.label_colors[i].label);
                unsafe {
                    let color = ui.style.colors.as_mut_ptr().add(i);
                    uint8_slider(ui, &mut (*color).r, 0, 255);
                    uint8_slider(ui, &mut (*color).g, 0, 255);
                    uint8_slider(ui, &mut (*color).b, 0, 255);
                    uint8_slider(ui, &mut (*color).a, 0, 255);
                }
                let next_layout = ui.layout_next();
                ui.draw_rect(next_layout, ui.style.colors[i]);
                i += 1;
            }
        })
    }

    fn process_frame(&mut self, ui: &mut Context) {
        ui.frame(|ui| {
            self.style_window(ui);
            self.log_window(ui);
            self.test_window(ui);
        })
    }
}

fn uint8_slider(ui: &mut Context, value: &mut u8, low: i32, high: i32) -> ResourceState {
    let mut tmp = *value as f32;
    ui.push_id_from_ptr(value);
    let res = ui.slider_ex(&mut tmp, low as _, high as _, 0.0, 0, WidgetOption::ALIGN_CENTER);
    *value = tmp as u8;
    ui.pop_id();
    res
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video_subsystem.window("Window", 800, 600).opengl().build().unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.

    // TODO: the rust compiler optimizes this out
    let _x_ = window.gl_create_context().unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _) };

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let (width, height) = window.size();
    let mut rd = Renderer::new(&gl, &ATLAS_TEXTURE, width, height);

    let mut state = State::new();

    pub fn r_get_char_width(_font: FontId, c: char) -> usize { ATLAS[ATLAS_FONT as usize + c as usize].w as usize }

    pub fn r_get_font_height(_font: FontId) -> usize { 18 }
    let mut ctx = microui::Context::new(r_get_char_width, r_get_font_height);

    'running: loop {
        let (width, height) = window.size();

        rd.clear(
            &gl,
            width as i32,
            height as i32,
            Color::rgba(state.bg[0] as u8, state.bg[1] as u8, state.bg[2] as u8, 255),
        );

        fn map_mouse_button(sdl_mb: sdl2::mouse::MouseButton) -> microui::MouseButton {
            match sdl_mb {
                sdl2::mouse::MouseButton::Left => microui::MouseButton::LEFT,
                sdl2::mouse::MouseButton::Right => microui::MouseButton::RIGHT,
                sdl2::mouse::MouseButton::Middle => microui::MouseButton::MIDDLE,
                _ => microui::MouseButton::empty(),
            }
        }

        fn map_keymode(sdl_km: sdl2::keyboard::Mod, sdl_kc: Option<sdl2::keyboard::Keycode>) -> microui::KeyMode {
            match (sdl_km, sdl_kc) {
                (sdl2::keyboard::Mod::LALTMOD, _) | (sdl2::keyboard::Mod::RALTMOD, _) => microui::KeyMode::ALT,
                (sdl2::keyboard::Mod::LCTRLMOD, _) | (sdl2::keyboard::Mod::RCTRLMOD, _) => microui::KeyMode::CTRL,
                (sdl2::keyboard::Mod::LSHIFTMOD, _) | (sdl2::keyboard::Mod::RSHIFTMOD, _) => microui::KeyMode::SHIFT,
                (_, Some(sdl2::keyboard::Keycode::Backspace)) => microui::KeyMode::BACKSPACE,
                (_, Some(sdl2::keyboard::Keycode::Return)) => microui::KeyMode::RETURN,
                _ => microui::KeyMode::empty(),
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::Window { win_event: WindowEvent::Close, .. } => break 'running,
                Event::MouseMotion { x, y, .. } => ctx.input_mousemove(x, y),
                Event::MouseWheel { y, .. } => ctx.input_scroll(0, y * -30),
                Event::MouseButtonDown { mouse_btn, .. } => {
                    let mb = map_mouse_button(mouse_btn);
                    ctx.input_mousedown(mb);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    let mb = map_mouse_button(mouse_btn);
                    ctx.input_mouseup(mb);
                }
                Event::KeyDown { keymod, keycode, .. } => {
                    let km = map_keymode(keymod, keycode);
                    ctx.input_keydown(km);
                }
                Event::KeyUp { keymod, keycode, .. } => {
                    let km = map_keymode(keymod, keycode);
                    ctx.input_keyup(km);
                }
                Event::TextInput { text, .. } => {
                    ctx.input_text(text.as_str());
                }

                _ => {}
            }
        }

        state.process_frame(&mut ctx);

        for &cmd in ctx.commands() {
            match cmd {
                Command::Text { str_start, str_len, pos, color, .. } => {
                    let str = &ctx.text_stack[str_start..str_start + str_len];
                    rd.draw_text(&gl, str, pos, color)
                }
                Command::Rect { rect, color } => rd.draw_rect(&gl, rect, color),
                Command::Icon { id, rect, color } => rd.draw_icon(&gl, id, rect, color),
                Command::Clip { rect } => rd.set_clip_rect(&gl, 800, 600, rect),
            }
        }

        rd.flush(&gl);
        window.gl_swap_window();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
