//! LLM integration UI components.
//!
//! This module provides:
//! - Settings modal for configuring LLM endpoint
//! - Analysis panel for viewing LLM responses
//! - Integration with anomaly detection results

use eframe::egui;

use crate::anomaly::analyze_all_channels;
use crate::app::UltraLogApp;
use crate::llm::{
    spawn_llm_worker, AnalysisContext, ChannelSummary, LlmConfig, LlmRequest, LlmRequestState,
    LlmResponse, LlmResult, ACCURACY_WARNING, API_KEY_WARNING, COST_WARNING, PRIVACY_WARNING,
};
use crate::normalize::normalize_channel_name_with_custom;

impl UltraLogApp {
    /// Render the LLM settings modal window
    pub fn render_llm_settings(&mut self, ctx: &egui::Context) {
        if !self.show_llm_settings {
            return;
        }

        let mut open = true;
        let mut should_close = false;

        egui::Window::new("LLM Assistant Settings")
            .open(&mut open)
            .resizable(true)
            .collapsible(false)
            .default_width(500.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                self.render_llm_settings_content(ui, &mut should_close);
            });

        if !open || should_close {
            self.show_llm_settings = false;
        }
    }

    fn render_llm_settings_content(&mut self, ui: &mut egui::Ui, should_close: &mut bool) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(5.0);

            // Warning banner
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(60, 40, 20))
                .corner_radius(egui::CornerRadius::same(4))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("⚠️").size(18.0));
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Important")
                                    .strong()
                                    .color(egui::Color32::from_rgb(255, 200, 100)),
                            );
                            ui.label(
                                egui::RichText::new(PRIVACY_WARNING)
                                    .small()
                                    .color(egui::Color32::LIGHT_GRAY),
                            );
                        });
                    });
                });

            ui.add_space(15.0);

            // Enable toggle
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.llm_config.enabled, "");
                ui.label(
                    egui::RichText::new("Enable LLM Assistant")
                        .size(14.0)
                        .strong(),
                );
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            // Preset buttons
            ui.label(egui::RichText::new("Quick Setup:").strong());
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.button("OpenAI").clicked() {
                    self.llm_config = LlmConfig::openai_preset();
                    self.llm_config.enabled = true;
                }
                if ui.button("Ollama (local)").clicked() {
                    self.llm_config = LlmConfig::ollama_preset();
                    self.llm_config.enabled = true;
                }
                if ui.button("LM Studio (local)").clicked() {
                    self.llm_config = LlmConfig::lm_studio_preset();
                    self.llm_config.enabled = true;
                }
            });

            ui.add_space(15.0);

            // Endpoint URL
            ui.label(egui::RichText::new("Endpoint URL:").strong());
            ui.add_space(3.0);
            ui.add(
                egui::TextEdit::singleline(&mut self.llm_config.endpoint_url)
                    .hint_text("https://api.openai.com/v1")
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(10.0);

            // API Key
            ui.label(egui::RichText::new("API Key:").strong());
            ui.label(
                egui::RichText::new("Leave blank for local endpoints that don't require auth")
                    .small()
                    .color(egui::Color32::GRAY),
            );
            ui.add_space(3.0);

            let api_key_str = self.llm_config.api_key.clone().unwrap_or_default();
            let mut api_key_input = api_key_str;
            ui.add(
                egui::TextEdit::singleline(&mut api_key_input)
                    .password(true)
                    .hint_text("sk-...")
                    .desired_width(f32::INFINITY),
            );
            self.llm_config.api_key = if api_key_input.is_empty() {
                None
            } else {
                Some(api_key_input)
            };

            ui.add_space(10.0);

            // Model
            ui.label(egui::RichText::new("Model:").strong());
            ui.add_space(3.0);
            ui.add(
                egui::TextEdit::singleline(&mut self.llm_config.model)
                    .hint_text("gpt-4o-mini, llama3.2, etc.")
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(15.0);

            // Advanced settings (collapsible)
            egui::CollapsingHeader::new("Advanced Settings")
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    // Max tokens
                    ui.horizontal(|ui| {
                        ui.label("Max response tokens:");
                        ui.add(
                            egui::DragValue::new(&mut self.llm_config.max_tokens)
                                .range(100..=4000)
                                .speed(10),
                        );
                    });

                    ui.add_space(5.0);

                    // Temperature
                    ui.horizontal(|ui| {
                        ui.label("Temperature:");
                        ui.add(
                            egui::Slider::new(&mut self.llm_config.temperature, 0.0..=1.0)
                                .step_by(0.1),
                        );
                    });
                    ui.label(
                        egui::RichText::new("Lower = more deterministic, higher = more creative")
                            .small()
                            .color(egui::Color32::GRAY),
                    );

                    ui.add_space(10.0);

                    // Vision settings
                    ui.checkbox(
                        &mut self.llm_config.vision_enabled,
                        "Include chart screenshot (requires vision model)",
                    );

                    if self.llm_config.vision_enabled {
                        ui.horizontal(|ui| {
                            ui.label("Image detail:");
                            egui::ComboBox::from_id_salt("vision_detail")
                                .selected_text(&self.llm_config.vision_detail)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.llm_config.vision_detail,
                                        "low".to_string(),
                                        "Low (faster, cheaper)",
                                    );
                                    ui.selectable_value(
                                        &mut self.llm_config.vision_detail,
                                        "high".to_string(),
                                        "High (better analysis)",
                                    );
                                    ui.selectable_value(
                                        &mut self.llm_config.vision_detail,
                                        "auto".to_string(),
                                        "Auto",
                                    );
                                });
                        });
                        ui.label(
                            egui::RichText::new(
                                "Vision models: GPT-4o, Claude, Llama 3.2 Vision, Qwen-VL",
                            )
                            .small()
                            .color(egui::Color32::GRAY),
                        );
                    }
                });

            ui.add_space(15.0);

            // Caveats section
            egui::CollapsingHeader::new("⚠️ Important Caveats")
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(5.0);
                    let caveat_style = egui::RichText::new("•").small();

                    ui.horizontal_wrapped(|ui| {
                        ui.label(caveat_style.clone());
                        ui.label(egui::RichText::new(ACCURACY_WARNING).small());
                    });
                    ui.add_space(3.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(caveat_style.clone());
                        ui.label(egui::RichText::new(COST_WARNING).small());
                    });
                    ui.add_space(3.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(caveat_style);
                        ui.label(egui::RichText::new(API_KEY_WARNING).small());
                    });
                });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("Test Connection").clicked() {
                    self.test_llm_connection();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        *should_close = true;
                    }
                    if ui.button("Save").clicked() {
                        self.show_toast_success("LLM settings saved");
                        *should_close = true;
                    }
                });
            });

            ui.add_space(10.0);
        });
    }

    /// Render the LLM analysis panel (shown alongside chart)
    pub fn render_llm_panel(&mut self, ctx: &egui::Context) {
        if !self.show_llm_panel || !self.llm_config.enabled {
            return;
        }

        egui::SidePanel::right("llm_panel")
            .default_width(350.0)
            .min_width(280.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                ui.heading("LLM Analysis");
                ui.separator();
                ui.add_space(5.0);

                // Prompt input
                ui.label("Ask about the data:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.llm_user_prompt)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY)
                        .hint_text("e.g., 'What anomalies do you see?'"),
                );

                ui.add_space(5.0);

                // Analyze button
                ui.horizontal(|ui| {
                    let is_pending = matches!(self.llm_request_state, LlmRequestState::Pending);

                    ui.add_enabled_ui(!is_pending, |ui| {
                        if ui.button("Analyze").clicked() {
                            self.start_llm_analysis();
                        }
                    });

                    if is_pending {
                        ui.spinner();
                        ui.label("Analyzing...");
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Settings").clicked() {
                            self.show_llm_settings = true;
                        }
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                // Results area
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        match &self.llm_request_state {
                            LlmRequestState::Idle => {
                                if let Some(ref result) = self.llm_last_result {
                                    ui.label(
                                        egui::RichText::new("Last Analysis:")
                                            .strong()
                                            .color(egui::Color32::LIGHT_BLUE),
                                    );
                                    ui.add_space(5.0);
                                    ui.label(result);
                                } else {
                                    ui.label(
                                        egui::RichText::new(
                                            "Enter a prompt and click Analyze to get LLM insights.",
                                        )
                                        .color(egui::Color32::GRAY),
                                    );
                                }
                            }
                            LlmRequestState::Pending => {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.spinner();
                                    ui.add_space(10.0);
                                    ui.label("Waiting for LLM response...");
                                });
                            }
                            LlmRequestState::Complete(result) => match result {
                                LlmResult::Success { response, .. } => {
                                    ui.label(
                                        egui::RichText::new("Analysis Result:")
                                            .strong()
                                            .color(egui::Color32::LIGHT_GREEN),
                                    );
                                    ui.add_space(5.0);
                                    ui.label(response);
                                }
                                LlmResult::Error(e) => {
                                    ui.label(
                                        egui::RichText::new("Error:")
                                            .strong()
                                            .color(egui::Color32::from_rgb(255, 100, 100)),
                                    );
                                    ui.add_space(5.0);
                                    ui.label(e);
                                }
                            },
                        }
                    });

                // Anomaly detection section
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Local Anomaly Detection").strong());
                    ui.checkbox(&mut self.anomaly_config.enabled, "Enabled");
                });

                if self.anomaly_config.enabled {
                    ui.add_space(5.0);
                    if ui.button("Run Detection").clicked() {
                        self.run_anomaly_detection();
                    }

                    if let Some(ref results) = self.anomaly_results {
                        ui.add_space(5.0);
                        ui.label(format!(
                            "Found {} anomalies ({} critical, {} warnings)",
                            results.anomalies.len(),
                            results.critical_count,
                            results.warning_count
                        ));

                        // Show top anomalies
                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .id_salt("anomaly_list")
                            .show(ui, |ui| {
                                for anomaly in results.top_anomalies(10) {
                                    ui.label(
                                        egui::RichText::new(anomaly.display_string())
                                            .small()
                                            .color(match anomaly.severity {
                                                crate::anomaly::AnomalySeverity::Critical => {
                                                    egui::Color32::from_rgb(255, 100, 100)
                                                }
                                                crate::anomaly::AnomalySeverity::Warning => {
                                                    egui::Color32::from_rgb(255, 200, 100)
                                                }
                                                crate::anomaly::AnomalySeverity::Info => {
                                                    egui::Color32::LIGHT_BLUE
                                                }
                                            }),
                                    );
                                }
                            });
                    }
                }
            });
    }

    /// Test connection to LLM endpoint
    fn test_llm_connection(&mut self) {
        if let Err(e) = self.llm_config.validate() {
            self.show_toast_error(&format!("Invalid config: {}", e));
            return;
        }

        // Ensure worker is spawned
        if self.llm_request_sender.is_none() {
            let (tx, rx) = spawn_llm_worker();
            self.llm_request_sender = Some(tx);
            self.llm_response_receiver = Some(rx);
        }

        if let Some(ref sender) = self.llm_request_sender {
            let _ = sender.send(LlmRequest::TestConnection(self.llm_config.clone()));
            self.show_toast("Testing connection...");
        }
    }

    /// Start an LLM analysis request
    fn start_llm_analysis(&mut self) {
        if !self.llm_config.enabled {
            self.show_toast_error("LLM is not enabled");
            return;
        }

        if let Err(e) = self.llm_config.validate() {
            self.show_toast_error(&format!("Invalid config: {}", e));
            return;
        }

        // Ensure worker is spawned
        if self.llm_request_sender.is_none() {
            let (tx, rx) = spawn_llm_worker();
            self.llm_request_sender = Some(tx);
            self.llm_response_receiver = Some(rx);
        }

        // Build analysis context
        let context = self.build_analysis_context();

        if let Some(ref sender) = self.llm_request_sender {
            let _ = sender.send(LlmRequest::Analyze(self.llm_config.clone(), context));
            self.llm_request_state = LlmRequestState::Pending;
        }
    }

    /// Build analysis context from current app state
    fn build_analysis_context(&self) -> AnalysisContext {
        let time_range = self.time_range.unwrap_or((0.0, 0.0));

        // Build channel summaries
        let mut channels = Vec::new();
        for selected in self.get_selected_channels() {
            if selected.file_index >= self.files.len() {
                continue;
            }
            let file = &self.files[selected.file_index];
            let data = file.log.get_channel_data(selected.channel_index);

            if data.is_empty() {
                continue;
            }

            let min = data.iter().cloned().fold(f64::MAX, f64::min);
            let max = data.iter().cloned().fold(f64::MIN, f64::max);
            let avg = data.iter().sum::<f64>() / data.len() as f64;
            let current = *data.last().unwrap_or(&0.0);

            let channel_name = selected.channel.name();
            let display_name = if self.field_normalization {
                normalize_channel_name_with_custom(&channel_name, Some(&self.custom_normalizations))
            } else {
                channel_name
            };

            channels.push(ChannelSummary {
                name: display_name,
                min,
                max,
                avg,
                current,
                unit: None, // Could add unit detection here
            });
        }

        // Get chart image if vision is enabled
        let chart_image = if self.llm_config.vision_enabled {
            self.render_chart_to_bytes().ok()
        } else {
            None
        };

        AnalysisContext {
            time_range,
            channels,
            anomalies: self.anomaly_results.clone(),
            chart_image,
            user_prompt: self.llm_user_prompt.clone(),
        }
    }

    /// Run local anomaly detection on selected channels
    pub fn run_anomaly_detection(&mut self) {
        let mut channel_data: Vec<(String, Vec<f64>, Vec<f64>)> = Vec::new();

        for selected in self.get_selected_channels() {
            if selected.file_index >= self.files.len() {
                continue;
            }
            let file = &self.files[selected.file_index];
            let times = file.log.get_times_as_f64().to_vec();
            let data = file.log.get_channel_data(selected.channel_index);

            let channel_name = selected.channel.name();
            let display_name = if self.field_normalization {
                normalize_channel_name_with_custom(&channel_name, Some(&self.custom_normalizations))
            } else {
                channel_name
            };

            channel_data.push((display_name, times, data));
        }

        if channel_data.is_empty() {
            self.show_toast("No channels selected for analysis");
            return;
        }

        let results = analyze_all_channels(&self.anomaly_config, &channel_data);

        let msg = format!(
            "Found {} anomalies ({} critical, {} warnings, {} info)",
            results.anomalies.len(),
            results.critical_count,
            results.warning_count,
            results.info_count
        );

        self.anomaly_results = Some(results);
        self.show_toast(&msg);
    }

    /// Poll for LLM responses (call from update loop)
    pub fn poll_llm_responses(&mut self) {
        // Collect responses first to avoid borrow issues
        let responses: Vec<LlmResponse> = if let Some(ref receiver) = self.llm_response_receiver {
            let mut collected = Vec::new();
            while let Ok(response) = receiver.try_recv() {
                collected.push(response);
            }
            collected
        } else {
            Vec::new()
        };

        // Now process collected responses
        for response in responses {
            match response {
                LlmResponse::ConnectionTest(result) => match result {
                    Ok(msg) => {
                        self.show_toast_success(&format!("Connection OK: {}", msg));
                    }
                    Err(e) => {
                        self.show_toast_error(&format!("Connection failed: {}", e));
                    }
                },
                LlmResponse::Analysis(result) => {
                    if let LlmResult::Success { ref response, .. } = result {
                        self.llm_last_result = Some(response.clone());
                    }
                    self.llm_request_state = LlmRequestState::Complete(result);
                }
            }
        }
    }
}
