mod catalogue;
mod releases;

use std::process::ExitCode;
use std::sync::mpsc::{self, Receiver, Sender};

use catalogue::App;
use eframe::egui;

fn main() -> ExitCode {
    let apps = match catalogue::load() {
        Ok(apps) => apps,
        Err(error) => {
            eprintln!("Cannot load catalogue.json: {error:#}");
            return ExitCode::FAILURE;
        }
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Hica Launcher")
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    match eframe::run_native(
        "Hica Launcher",
        options,
        Box::new(move |cc| Ok(Box::new(Launcher::new(apps, cc.egui_ctx.clone())))),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Cannot open Hica Launcher: {error}");
            ExitCode::FAILURE
        }
    }
}

enum ReleaseState {
    Loading,
    Available(String),
    Failed(String),
}

struct ReleaseResult {
    generation: u64,
    index: usize,
    result: anyhow::Result<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum View {
    Grid,
    Table,
}

struct Launcher {
    apps: Vec<App>,
    releases: Vec<ReleaseState>,
    sender: Sender<ReleaseResult>,
    receiver: Receiver<ReleaseResult>,
    context: egui::Context,
    generation: u64,
    view: View,
    sort_by_version: bool,
    descending: bool,
}

impl Launcher {
    fn new(apps: Vec<App>, context: egui::Context) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut launcher = Self {
            apps,
            releases: Vec::new(),
            sender,
            receiver,
            context,
            generation: 0,
            view: View::Grid,
            sort_by_version: false,
            descending: false,
        };
        launcher.refresh();
        launcher
    }

    fn refresh(&mut self) {
        self.generation += 1;
        let generation = self.generation;
        self.releases = (0..self.apps.len())
            .map(|_| ReleaseState::Loading)
            .collect();
        for (index, app) in self.apps.iter().enumerate() {
            let repo = app.repo.clone();
            let sender = self.sender.clone();
            let context = self.context.clone();
            std::thread::spawn(move || {
                let result = releases::latest(&repo);
                if sender
                    .send(ReleaseResult {
                        generation,
                        index,
                        result,
                    })
                    .is_ok()
                {
                    context.request_repaint();
                }
            });
        }
    }

    fn receive(&mut self) {
        while let Ok(update) = self.receiver.try_recv() {
            if update.generation != self.generation {
                continue;
            }
            self.releases[update.index] = match update.result {
                Ok(version) => ReleaseState::Available(version),
                Err(error) => ReleaseState::Failed(format!("{error:#}")),
            };
        }
    }

    fn version(&self, index: usize) -> &str {
        match &self.releases[index] {
            ReleaseState::Loading => "Checking releases…",
            ReleaseState::Available(version) => version,
            ReleaseState::Failed(_) => "Release unavailable",
        }
    }

    fn app_details(&self, ui: &mut egui::Ui, index: usize) {
        let app = &self.apps[index];
        ui.label(&app.description);
        ui.label(format!("Latest: {}", self.version(index)));
        if let ReleaseState::Failed(error) = &self.releases[index] {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        ui.hyperlink_to(
            "GitHub releases ↗",
            format!("https://github.com/{}/releases", app.repo),
        );
    }
}

impl eframe::App for Launcher {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.receive();
        ui.horizontal(|ui| {
            ui.heading("Hica Launcher");
            ui.separator();
            ui.selectable_value(&mut self.view, View::Grid, "Grid");
            ui.selectable_value(&mut self.view, View::Table, "Table");
            if ui.button("Refresh").clicked() {
                self.refresh();
            }
        });
        ui.separator();
        if self.apps.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Your Hica apps will appear here");
                ui.label("Add GitHub repositories to catalogue.json and rebuild the launcher.");
            });
            return;
        }
        egui::ScrollArea::vertical().show(ui, |ui| match self.view {
            View::Grid => {
                ui.columns(2, |columns| {
                    for (index, app) in self.apps.iter().enumerate() {
                        columns[index % 2].group(|ui| {
                            ui.heading(&app.name);
                            self.app_details(ui, index);
                        });
                    }
                });
            }
            View::Table => {
                ui.horizontal(|ui| {
                    if ui.button("Name").clicked() {
                        if self.sort_by_version {
                            self.sort_by_version = false;
                            self.descending = false;
                        } else {
                            self.descending = !self.descending;
                        }
                    }
                    if ui.button("Latest version").clicked() {
                        if self.sort_by_version {
                            self.descending = !self.descending;
                        } else {
                            self.sort_by_version = true;
                            self.descending = false;
                        }
                    }
                });
                let mut order: Vec<_> = (0..self.apps.len()).collect();
                if self.sort_by_version {
                    order.sort_by(|&a, &b| {
                        self.version(a)
                            .cmp(self.version(b))
                            .then(self.apps[a].name.cmp(&self.apps[b].name))
                    });
                } else {
                    order.sort_by(|&a, &b| self.apps[a].name.cmp(&self.apps[b].name));
                }
                if self.descending {
                    order.reverse();
                }
                egui::Grid::new("apps_table")
                    .striped(true)
                    .min_col_width(140.0)
                    .show(ui, |ui| {
                        ui.strong("App");
                        ui.strong("Latest release");
                        ui.strong("Repository");
                        ui.end_row();
                        for index in order {
                            let app = &self.apps[index];
                            ui.label(&app.name);
                            ui.label(self.version(index));
                            ui.hyperlink_to(
                                &app.repo,
                                format!("https://github.com/{}/releases", app.repo),
                            );
                            ui.end_row();
                            if let ReleaseState::Failed(error) = &self.releases[index] {
                                ui.colored_label(egui::Color32::LIGHT_RED, error);
                                ui.end_row();
                            }
                        }
                    });
            }
        });
    }
}
