//! Channel selection and display UI components.

use eframe::egui;

use crate::app::UltraLogApp;
use crate::normalize::{normalize_channel_name_with_custom, sort_channels_by_priority};
use crate::state::MAX_CHANNELS;

impl UltraLogApp {
    /// Render channel selection panel - fills available space
    pub fn render_channel_selection(&mut self, ui: &mut egui::Ui) {
        ui.heading("Channels");
        ui.separator();

        // Get active tab info
        let tab_info = self.active_tab.and_then(|tab_idx| {
            let tab = &self.tabs[tab_idx];
            if tab.file_index < self.files.len() {
                Some((
                    tab.file_index,
                    tab.channel_search.clone(),
                    tab.selected_channels.len(),
                ))
            } else {
                None
            }
        });

        if let Some((file_index, current_search, selected_count)) = tab_info {
            let channel_count = self.files[file_index].log.channels.len();

            // Search box - use a temporary string that we'll update
            let mut search_text = current_search;
            let mut search_changed = false;
            ui.horizontal(|ui| {
                ui.label("Search:");
                let response = ui
                    .add(egui::TextEdit::singleline(&mut search_text).desired_width(f32::INFINITY));
                search_changed = response.changed();
            });

            // Defer the set_channel_search call to avoid borrow issues
            if search_changed {
                self.set_channel_search(search_text.clone());
            }

            ui.add_space(5.0);

            // Channel count
            ui.label(format!(
                "Selected: {} / {} | Total: {}",
                selected_count, MAX_CHANNELS, channel_count
            ));

            ui.separator();

            // Channel list - use all remaining vertical space
            let search_lower = search_text.to_lowercase();
            let mut channel_to_add: Option<(usize, usize)> = None;
            let mut channel_to_remove: Option<usize> = None;

            // Sort channels: normalized fields first, then alphabetically
            // Collect channel names upfront to avoid borrow issues
            let file = &self.files[file_index];
            let sorted_channels = sort_channels_by_priority(
                file.log.channels.len(),
                |idx| file.log.channels[idx].name(),
                self.field_normalization,
                Some(&self.custom_normalizations),
            );

            // Get original names for all channels (needed for search)
            let channel_names: Vec<String> = (0..file.log.channels.len())
                .map(|idx| file.log.channels[idx].name())
                .collect();

            // Get selected channels for comparison
            let selected_channels = self.get_selected_channels().to_vec();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    for (channel_index, display_name, _is_normalized) in &sorted_channels {
                        let original_name = &channel_names[*channel_index];

                        // Filter by search (search both original and normalized names)
                        if !search_lower.is_empty()
                            && !original_name.to_lowercase().contains(&search_lower)
                            && !display_name.to_lowercase().contains(&search_lower)
                        {
                            continue;
                        }

                        // Check if already selected and get its index in selected_channels
                        let selected_idx = selected_channels.iter().position(|c| {
                            c.file_index == file_index && c.channel_index == *channel_index
                        });
                        let is_selected = selected_idx.is_some();

                        // Build the label with checkmark prefix if selected
                        let label_text = if is_selected {
                            format!("[*] {}", display_name)
                        } else {
                            format!("[ ] {}", display_name)
                        };

                        let response = ui.selectable_label(is_selected, label_text);

                        if response.clicked() {
                            if let Some(idx) = selected_idx {
                                // Already selected - remove it
                                channel_to_remove = Some(idx);
                            } else {
                                // Not selected - add it
                                channel_to_add = Some((file_index, *channel_index));
                            }
                        }
                        if response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                    }
                });

            // Handle deferred channel removal (must happen before addition to keep indices valid)
            if let Some(idx) = channel_to_remove {
                self.remove_channel(idx);
            }

            // Handle deferred channel addition
            if let Some((file_idx, channel_idx)) = channel_to_add {
                self.add_channel(file_idx, channel_idx);
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("Select a file to view channels")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
            });
        }
    }

    /// Render selected channel cards
    pub fn render_selected_channels(&mut self, ui: &mut egui::Ui) {
        ui.heading("Selected Channels");
        ui.separator();

        let mut channel_to_remove: Option<usize> = None;
        let use_normalization = self.field_normalization;
        let custom_mappings = &self.custom_normalizations;

        // Get selected channels from the active tab
        let selected_channels = self.get_selected_channels().to_vec();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for (i, selected) in selected_channels.iter().enumerate() {
                    let color = self.get_channel_color(selected.color_index);
                    let color32 = egui::Color32::from_rgb(color[0], color[1], color[2]);

                    // Get display name (normalized or original based on setting)
                    let channel_name = selected.channel.name();
                    let display_name = if use_normalization {
                        normalize_channel_name_with_custom(&channel_name, Some(custom_mappings))
                    } else {
                        channel_name
                    };

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(40, 40, 40))
                        .stroke(egui::Stroke::new(2.0, color32))
                        .rounding(5.0)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&display_name).strong().color(color32),
                                    );
                                    let close_btn = ui.small_button("x");
                                    if close_btn.clicked() {
                                        channel_to_remove = Some(i);
                                    }
                                    if close_btn.hovered() {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }
                                });

                                ui.label(
                                    egui::RichText::new(format!(
                                        "Type: {}",
                                        selected.channel.type_name()
                                    ))
                                    .color(egui::Color32::GRAY),
                                );

                                if let (Some(min), Some(max)) = (
                                    selected.channel.display_min(),
                                    selected.channel.display_max(),
                                ) {
                                    let source_unit = selected.channel.unit();
                                    let (conv_min, display_unit) =
                                        self.unit_preferences.convert_value(min, source_unit);
                                    let (conv_max, _) =
                                        self.unit_preferences.convert_value(max, source_unit);
                                    let unit_str = if display_unit.is_empty() {
                                        String::new()
                                    } else {
                                        format!(" {}", display_unit)
                                    };
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Range: {:.0}{} - {:.0}{}",
                                            conv_min, unit_str, conv_max, unit_str
                                        ))
                                        .color(egui::Color32::GRAY),
                                    );
                                }
                            });
                        });

                    ui.add_space(5.0);
                }
            });
        });

        if let Some(index) = channel_to_remove {
            self.remove_channel(index);
        }

        if selected_channels.is_empty() {
            ui.label(
                egui::RichText::new("Click channels to add them to the chart")
                    .italics()
                    .color(egui::Color32::GRAY),
            );
        }
    }
}
