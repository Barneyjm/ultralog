//! Field Normalization Editor UI.
//!
//! Provides a window for users to view and customize field name mappings.

use eframe::egui;

use crate::app::UltraLogApp;
use crate::normalize::get_builtin_mappings;

impl UltraLogApp {
    /// Render the field normalization editor window
    pub fn render_normalization_editor(&mut self, ctx: &egui::Context) {
        if !self.show_normalization_editor {
            return;
        }

        let mut open = true;

        egui::Window::new("Field Normalization Editor")
            .open(&mut open)
            .resizable(true)
            .default_width(500.0)
            .default_height(400.0)
            .order(egui::Order::Foreground) // Ensure window is on top of chart overlays
            .show(ctx, |ui| {
                ui.heading("Custom Mappings");
                ui.label(
                    egui::RichText::new(
                        "Add your own field name mappings. Custom mappings take priority over built-in ones.",
                    )
                    .color(egui::Color32::GRAY),
                );
                ui.add_space(8.0);

                // Input fields for adding new mapping
                ui.horizontal(|ui| {
                    ui.label("Source Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.norm_editor_source)
                            .hint_text("e.g., MyCustomRPM")
                            .desired_width(150.0),
                    );
                    ui.label("→");
                    ui.label("Display As:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.norm_editor_target)
                            .hint_text("e.g., RPM")
                            .desired_width(150.0),
                    );
                    if ui.button("Add").clicked() {
                        if !self.norm_editor_source.is_empty()
                            && !self.norm_editor_target.is_empty()
                        {
                            // Store with lowercase key for case-insensitive matching
                            self.custom_normalizations.insert(
                                self.norm_editor_source.to_lowercase(),
                                self.norm_editor_target.clone(),
                            );
                            self.norm_editor_source.clear();
                            self.norm_editor_target.clear();
                        }
                    }
                });

                ui.add_space(8.0);

                // Display existing custom mappings
                if !self.custom_normalizations.is_empty() {
                    ui.separator();
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Your Custom Mappings:").strong());
                    ui.add_space(4.0);

                    let mut to_remove: Option<String> = None;

                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            egui::Grid::new("custom_mappings_grid")
                                .striped(true)
                                .num_columns(3)
                                .min_col_width(100.0)
                                .spacing([16.0, 8.0]) // More horizontal and vertical spacing
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Source").strong());
                                    ui.label(egui::RichText::new("Display As").strong());
                                    ui.label("");
                                    ui.end_row();

                                    for (source, target) in &self.custom_normalizations {
                                        ui.label(source);
                                        ui.label(target);
                                        if ui.small_button("Remove").clicked() {
                                            to_remove = Some(source.clone());
                                        }
                                        ui.end_row();
                                    }
                                });
                        });

                    if let Some(key) = to_remove {
                        self.custom_normalizations.remove(&key);
                    }
                }

                ui.add_space(16.0);
                ui.separator();

                // Built-in mappings reference (collapsible)
                egui::CollapsingHeader::new("Built-in Mappings Reference")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(
                                "These mappings are built into UltraLog and cannot be modified.",
                            )
                            .color(egui::Color32::GRAY),
                        );
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                let mappings = get_builtin_mappings();
                                for (normalized, sources) in mappings {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(
                                            egui::RichText::new(normalized)
                                                .strong()
                                                .color(egui::Color32::LIGHT_BLUE),
                                        );
                                        ui.label("←");
                                        ui.label(
                                            egui::RichText::new(sources.join(", "))
                                                .color(egui::Color32::GRAY),
                                        );
                                    });
                                    ui.add_space(2.0);
                                }
                            });
                    });
            });

        if !open {
            self.show_normalization_editor = false;
        }
    }
}
