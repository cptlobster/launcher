use std::io::Error;
use eframe::egui;

use std::process::{Command};
use std::os::unix::process::CommandExt;
use shlex::split;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "launcher",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    command: String,
    error: String,
}

fn exec_sh(command: &str) -> String {
    match split(command).as_deref() {
        Some([cmd, args @ ..]) => {
            let err: Error = Command::new(cmd).args(args).exec();
            format!("Error executing {}\n{}", command, err)
        },
        Some([]) => "No command specified".to_string(),
        None => format!("Error parsing command '{}'", command),
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("launcher");
            ui.horizontal(|ui| {
                let name_label = ui.label(">>>");
                ui.text_edit_singleline(&mut self.command)
                    .labelled_by(name_label.id);
                if ui.button("Run").clicked() {
                    exec_sh(&self.command);
                };
            });
            ui.colored_label(egui::Color32::RED, self.error.to_owned());
        });
    }
}