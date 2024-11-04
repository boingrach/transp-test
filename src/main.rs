use eframe::egui;
use eframe::egui::{Color32, ViewportCommand};
use tracing_subscriber::EnvFilter;

fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    eframe::run_native(
        "My egui App",
        Default::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    fullscreen: bool,
    transparent: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(ViewportCommand::Transparent(true));
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                ui.label(format!("fullscreen: {}", self.fullscreen));
                if ui.button("toggle fullscreen").clicked() {
                    self.fullscreen = !self.fullscreen;
                    ctx.send_viewport_cmd(ViewportCommand::Fullscreen(self.fullscreen));
                }

                ui.label(format!("transparent: {}", self.transparent));
                if ui.button("toggle transparency").clicked() {
                    self.transparent = !self.transparent;
                    ctx.send_viewport_cmd(ViewportCommand::Transparent(self.transparent));
                }
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        match self.transparent {
            true => egui::Rgba::TRANSPARENT.to_array(),
            false => egui::Rgba::BLUE.to_array(),
        }
    }
}
