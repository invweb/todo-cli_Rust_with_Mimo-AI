use eframe::egui;
use crate::{Task, load_tasks, save_tasks, next_id};
use crate::i18n::Localizer;

pub fn run_gui() {
    let loc = Localizer::new();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title(loc.translate("app-title")),
        ..Default::default()
    };

    let _ = eframe::run_native(
        &loc.translate("app-title"),
        options,
        Box::new(|_cc| Ok(Box::new(TodoApp::new()))),
    );
}

struct TodoApp {
    tasks: Vec<Task>,
    input_text: String,
    loc: Localizer,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            tasks: load_tasks(),
            input_text: String::new(),
            loc: Localizer::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.loc.translate("app-title"));
            ui.separator();

            ui.horizontal(|ui| {
                let input = ui.text_edit_singleline(&mut self.input_text);
                if ui.button(self.loc.translate("add-button")).clicked()
                    || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    if !self.input_text.trim().is_empty() {
                        let task = Task {
                            id: next_id(&self.tasks),
                            title: self.input_text.trim().to_string(),
                            done: false,
                        };
                        self.tasks.push(task);
                        save_tasks(&self.tasks);
                        self.input_text.clear();
                    }
                }
            });

            ui.separator();

            let mut to_remove = None;
            let mut to_save = false;

            for (i, task) in self.tasks.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut task.done, "").changed() {
                        to_save = true;
                    }

                    let label = if task.done {
                        egui::RichText::new(&task.title).strikethrough().weak()
                    } else {
                        egui::RichText::new(&task.title)
                    };
                    ui.label(label);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("×").clicked() {
                            to_remove = Some(i);
                        }
                    });
                });
            }

            if to_save {
                save_tasks(&self.tasks);
            }

            if let Some(index) = to_remove {
                self.tasks.remove(index);
                save_tasks(&self.tasks);
            }
        });
    }
}