mod color;

use color::BACKGROUND_COLOR;
use egui::{vec2, Key, Slider};
use glium::glutin::event::{ElementState, KeyboardInput};
use glium::implement_vertex;
use glium::uniform;
use std::time::Instant;

extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Game");
    let context_builder = glutin::ContextBuilder::new()
        .with_hardware_acceleration(Some(true))
        .with_vsync(true)
        .with_srgb(true);

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let mut egui = egui_glium::EguiGlium::new(&display);

    let mut name = "Hi!";

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let mut triangle_color = [2 as u8; 3];

    event_loop.run(move |event, _, control_flow| {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut redraw = || {
            egui.begin_frame(&display);

            let mut quit = false;

            egui::TopPanel::top("my_top_panel").show(egui.ctx(), |ui| {
                let file_popup_id = ui.make_persistent_id("file_popup_0");
                ui.horizontal(|ui| {
                    let file_button = ui.button("File");

                    if file_button.clicked() {
                        ui.memory().toggle_popup(file_popup_id)
                    }
                    egui::popup::popup_below_widget(ui, file_popup_id, &file_button, |ui| {
                        ui.set_min_width(200.0); // if you want to control the size
                        ui.label("Some more info, or things you can select:");
                        ui.label("…");
                        if ui.button("Open").clicked() {
                            unimplemented!("Open Files ins't Implemented yet!");
                        }
                    });
                });
            });

            egui::SidePanel::left("my_side_panel", 300.0).show(egui.ctx(), |ui| {
                ui.heading("Hello World!");
                if ui.button("Quit").clicked() {
                    quit = true;
                }

                egui::ComboBox::from_label("Version")
                    .width(150.0)
                    .selected_text("Moin leude so ne box hier")
                    .show_ui(ui, |ui| {});

                egui::CollapsingHeader::new("Dev")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("contains");
                        ui.button("Hi");
                    });
            });

            egui::Window::new("Dreieck")
                .scroll(false)
                .resize(|r| r.resizable(true))
                .default_size(vec2(512.0, 256.0))
                .show(egui.ctx(), |ui| {
                    ui.label(name);
                    if ui.button("Set label").clicked() {
                        name = "Hello!";
                    }

                    ui.add(Slider::new(&mut x, -1.0..=1.0).text("X")).dragged();
                    ui.add(Slider::new(&mut y, -1.0..=1.0).text("Y")).dragged();

                    ui.color_edit_button_srgb(&mut triangle_color);
                });

            let (needs_repaint, shapes) = egui.end_frame(&display);
            
            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };
            

            {
                use glium::Surface as _;
                let mut target = display.draw();
            
                target.clear_color_srgb(
                    BACKGROUND_COLOR.get_glfloat_red(),
                    BACKGROUND_COLOR.get_glfloat_green(),
                    BACKGROUND_COLOR.get_glfloat_blue(),
                    1.0,
                );
                    

                let uniforms = uniform! {
                    matrix: [
                        [t.cos(), t.sin(), 0.0, 0.0],
                        [-t.sin(), t.cos(), 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [ x , y, 0.0, 1.0f32],
                    ]
                };

                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();
                // draw things behind egui here

                egui.paint(&display, &mut target, shapes);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                glutin::event::KeyboardInput {
                    scancode: 0,
                    state: ElementState::Pressed,
                    virtual_keycode: None,
                    modifiers: Default::default(),
                };

                egui.on_event(event, control_flow);
                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
    });
}
