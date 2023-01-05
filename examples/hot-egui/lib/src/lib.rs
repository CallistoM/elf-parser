use eframe::egui;

pub struct State {
    name: String,
    age: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Robert".to_owned(),
            age: 36,
        }
    }
}

#[no_mangle]
pub fn render(state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // First add SidePanel widget to prevent overlap with CentalPanel widget.

    egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
        ui.label("Binary parser");
        ui.separator();
        ui.label("Overview");
        ui.label("ELF Header");
        ui.label("Symbols");
        ui.label("SHeaders");
        ui.label("PHeaders");
    });

    egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    println!("{:?}", path.display());
                }
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ctx.set_pixels_per_point(2.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.horizontal(|ui| {
                ui.label("Your name: ");
            });
        });

        ui.heading("My wgui Application");
        let response = ui.add(egui::Slider::new(&mut state.age, 0..=120).text("age"));
        response.on_hover_text("help");
        if ui.button("Click each year").clicked() {
            state.age += 1;
        }
        ui.label(format!("Hello '{}', age {}", state.name, state.age));
    });
}
