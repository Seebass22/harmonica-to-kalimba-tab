#[path = "harp2kalimba.rs"]
mod harp2kalimba;
use egui::{Button, RichText, TextStyle};

pub struct App {
    input_tab: String,
    output_tab: String,
    semitone_offset: i32,
    playable_keys: Vec<(&'static str, i32)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // input_tab: "".to_owned(),
            input_tab: "4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 4 -4
4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 -4 4 

-5 -5 -6 -6 -6 6 5 4 -4 5 
4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 -4 4"
                .to_owned(),
            output_tab: "".to_owned(),
            semitone_offset: 0,
            playable_keys: Vec::new(),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("harmonica tab");
            if ui.text_edit_multiline(&mut self.input_tab).changed() {
                self.playable_keys = harp2kalimba::get_playable_keys(&self.input_tab, "richter");
                self.transpose();
            }

            if ui
                .add(egui::Slider::new(&mut self.semitone_offset, -20..=20).text("semitone change"))
                .changed()
            {
                self.transpose();
            };
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("kalimba tab");
            ui.text_edit_multiline(&mut self.output_tab);
            self.playable_keys_panel(ui);
        });
    }
}

impl App {
    fn transpose(&mut self) {
        (self.output_tab, _) =
            harp2kalimba::transpose_tabs(&self.input_tab, self.semitone_offset, "richter");
    }

    fn playable_keys_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled(
            false,
            Button::new(
                RichText::new("key, semitone change").text_style(TextStyle::Monospace),
            ),
        );

        for (key, semitones) in self.playable_keys.clone().iter() {
            let text = format!(
                "{:width$} {:+width$}",
                key,
                semitones,
                width = 7
            );

            if ui
                .add(Button::new(
                    RichText::new(text).text_style(TextStyle::Monospace),
                ))
                .clicked()
            {
                self.semitone_offset = *semitones;
                self.transpose();
            }
        }
    }
}
