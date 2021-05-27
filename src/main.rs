mod color;
mod teapot;

use color::BACKGROUND_COLOR;
use egui::{vec2, Slider};
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


    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();


    let vertex_shader_src = r#"
        #version 330

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;  
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        #ifdef GL_ES
        precision mediump float;
        #endif
        
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        uniform vec4 rgba;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6 * rgba);
            vec3 regular_color = vec3(rgba);
            color = vec4(mix(dark_color, regular_color, brightness), rgba[3]);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 2.0;

    let mut color_picker_color = egui::Color32::from_rgba_premultiplied(255, 0, 0, 255);

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
                    .show_ui(ui, |ui| {
                        ui.label("Keine Ahnung");
                    });

                egui::CollapsingHeader::new("Dev")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("contains");
                        if ui.button("Hi").double_clicked() {
                            println!("Hi");
                        };
                    });
            });

            egui::Window::new("Teekanne")
                .scroll(false)
                .default_size(vec2(200.0, 256.0))
                .show(egui.ctx(), |ui| {
                    egui::CollapsingHeader::new("Location")
                        .default_open(true)
                        .show(ui, |ui| {
                            // Ui for the teacan location
                            ui.add(Slider::new(&mut x, -1.0..=1.0).text("X")).dragged();
                            ui.add(Slider::new(&mut y, -1.0..=1.0).text("Y")).dragged();
                            ui.add(Slider::new(&mut z, -2.0..=2.0).text("Z")).dragged();
                        });
                    ui.color_edit_button_srgba(&mut color_picker_color);
                });

            let (_needs_repaint, shapes) = egui.end_frame(&display);
            /*
            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };
            */

            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
             *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            
                display.gl_window().window().request_redraw();

            {
                use glium::Surface as _;
                let mut target = display.draw();
            
                target.clear_color_srgb_and_depth(
                    (
                        BACKGROUND_COLOR.get_glfloat_red(),
                        BACKGROUND_COLOR.get_glfloat_green(),
                        BACKGROUND_COLOR.get_glfloat_blue(),
                        1.0,
                    ),
                    1.0 // alpha
                );

                    
                let matrix = [
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [x, y, z, 1.0f32],
                ];

                let perspective = {
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;

                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;

                    let f = 1.0 / (fov / 2.0).tan();

                    [
                        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                        [         0.0         ,     f ,              0.0              ,   0.0],
                        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                    ]
                };   

                let light = [-1.0, 0.4, 0.9f32];
                let color = [
                    color_picker_color[0] as f32 / 250.0,
                    color_picker_color[1] as f32 / 250.0,
                    color_picker_color[2] as f32 / 250.0,
                    color_picker_color[3] as f32 / 250.0,
                ];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    blend: glium::draw_parameters::Blend::alpha_blending(),
                        .. Default::default()
                };

                target
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &uniform! { matrix: matrix, perspective: perspective, u_light: light, rgba: color },
                        &params,
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
               

                egui.on_event(event, control_flow);
                //display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
    });
}
