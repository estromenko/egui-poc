use eframe;
use eframe::egui;

#[derive(Default)]
struct App {
    checkbox_state: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("test");

            egui::CollapsingHeader::new("Dropdown").show(ui, |ui| {
                ui.label("Some text");
            });

            let button_text =
                egui::RichText::new("Some beautiful button").color(egui::Color32::WHITE);
            let button = egui::Button::new(button_text).fill(egui::Color32::from_rgb(0, 0, 255));

            ui.add(button);

            ui.checkbox(&mut self.checkbox_state, "Checkbox");
        });

        egui::Window::new("New window").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("test");
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_drag_and_drop(false),
        ..Default::default()
    };
    eframe::run_native("Test", options, Box::new(|_cc| Box::<App>::default())).unwrap();
}
