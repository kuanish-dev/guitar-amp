mod audio;
mod dsp;
mod tuner;

use dsp::{Params, Settings};
use eframe::egui;
use ringbuf::traits::Consumer;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

struct App {
    params: Arc<Params>,
    settings: Settings,
    engine: Option<audio::Engine>,
    inputs: Vec<String>,
    outputs: Vec<String>,
    input: String,
    output: String,
    channel: usize,
    status: String,
    tuner: tuner::Tuner,
    reading: Option<tuner::Reading>,
    presets: Vec<(&'static str, Settings)>,
}

impl App {
    fn new() -> Self {
        let settings = Settings::clean();
        let mut app = Self {
            params: Arc::new(Params::new(&settings)),
            settings,
            engine: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            input: String::new(),
            output: String::new(),
            channel: 0,
            status: "Stopped".to_string(),
            tuner: tuner::Tuner::new(),
            reading: None,
            presets: Settings::presets(),
        };
        app.refresh_devices();
        app
    }

    fn refresh_devices(&mut self) {
        self.inputs = audio::input_names();
        self.outputs = audio::output_names();
        if !self.inputs.contains(&self.input) {
            self.input = audio::default_input_name()
                .or_else(|| self.inputs.first().cloned())
                .unwrap_or_default();
        }
        if !self.outputs.contains(&self.output) {
            self.output = audio::default_output_name()
                .or_else(|| self.outputs.first().cloned())
                .unwrap_or_default();
        }
    }

    fn start(&mut self) {
        self.engine = None;
        self.reading = None;
        self.tuner.reset();
        match audio::start(&self.input, &self.output, &self.params) {
            Ok(engine) => {
                self.status = format!("Running at {} Hz", engine.sample_rate);
                self.engine = Some(engine);
            }
            Err(e) => self.status = e,
        }
    }

    fn stop(&mut self) {
        self.engine = None;
        self.reading = None;
        self.tuner.reset();
        self.status = "Stopped".to_string();
        self.params.in_level.set(0.0);
        self.params.out_level.set(0.0);
    }
}

fn section(ui: &mut egui::Ui, label: &str, on: &mut bool, body: impl FnOnce(&mut egui::Ui)) {
    ui.separator();
    ui.checkbox(on, egui::RichText::new(label).strong());
    ui.add_enabled_ui(*on, body);
}

fn knob(ui: &mut egui::Ui, label: &str, value: &mut f32) {
    ui.horizontal(|ui| {
        ui.add_sized([90.0, 20.0], egui::Label::new(label));
        ui.spacing_mut().slider_width = 260.0;
        ui.add(egui::Slider::new(value, 0.0..=1.0).show_value(false));
        ui.label(format!("{:.0}", *value * 10.0));
    });
}

fn meter(ui: &mut egui::Ui, label: &str, level: f32) {
    ui.horizontal(|ui| {
        ui.add_sized([90.0, 16.0], egui::Label::new(label));
        let db = 20.0 * level.max(0.00001).log10();
        let amount = ((db + 60.0) / 60.0).clamp(0.0, 1.0);
        let color = if level > 0.95 {
            egui::Color32::from_rgb(220, 60, 60)
        } else {
            egui::Color32::from_rgb(80, 190, 110)
        };
        ui.add(
            egui::ProgressBar::new(amount)
                .desired_width(290.0)
                .desired_height(12.0)
                .fill(color),
        );
    });
}

fn tuner_view(ui: &mut egui::Ui, reading: Option<tuner::Reading>) {
    let green = egui::Color32::from_rgb(80, 190, 110);
    let amber = egui::Color32::from_rgb(230, 170, 50);
    let (name, detail, hint, color, cents) = match reading {
        Some(r) => {
            let (hint, color) = if r.cents.abs() <= 5.0 {
                ("In tune", green)
            } else if r.cents < 0.0 {
                ("Too low - tighten the string", amber)
            } else {
                ("Too high - loosen the string", amber)
            };
            (
                format!("{}{}", r.note, r.octave),
                format!("{:.1} Hz   {:+.0} cents", r.frequency, r.cents),
                hint,
                color,
                Some(r.cents),
            )
        }
        None => (
            "--".to_string(),
            "Strings: E2  A2  D3  G3  B3  E4".to_string(),
            "Play one string",
            ui.visuals().weak_text_color(),
            None,
        ),
    };

    ui.horizontal(|ui| {
        ui.add_sized(
            [90.0, 44.0],
            egui::Label::new(egui::RichText::new(name).size(36.0).strong().color(color)),
        );
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(hint).color(color));
            ui.label(detail);
        });
    });

    let (rect, _) = ui.allocate_exact_size(egui::vec2(390.0, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    let line = ui.visuals().weak_text_color();
    painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);
    let zone =
        egui::Rect::from_center_size(rect.center(), egui::vec2(rect.width() * 0.1, rect.height()));
    painter.rect_filled(zone, 0.0, green.gamma_multiply(0.25));
    painter.line_segment(
        [rect.center_top(), rect.center_bottom()],
        egui::Stroke::new(1.0, line),
    );
    if let Some(cents) = cents {
        let x = rect.center().x + cents.clamp(-50.0, 50.0) / 50.0 * (rect.width() / 2.0 - 3.0);
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(5.0, color),
        );
    }
}

fn device_picker(ui: &mut egui::Ui, label: &str, names: &[String], selected: &mut String) -> bool {
    let before = selected.clone();
    ui.horizontal(|ui| {
        ui.add_sized([90.0, 20.0], egui::Label::new(label));
        egui::ComboBox::from_id_salt(label)
            .width(290.0)
            .selected_text(selected.as_str())
            .show_ui(ui, |ui| {
                for name in names {
                    ui.selectable_value(selected, name.clone(), name);
                }
            });
    });
    before != *selected
}

impl App {
    fn left(&mut self, ui: &mut egui::Ui) {
        let mut changed = device_picker(ui, "Input", &self.inputs, &mut self.input);
        changed |= device_picker(ui, "Output", &self.outputs, &mut self.output);

        let channels = self.params.input_channels.load(Ordering::Relaxed);
        if channels > 1 {
            ui.horizontal(|ui| {
                ui.add_sized([90.0, 20.0], egui::Label::new("Input jack"));
                for i in 0..channels.min(8) {
                    ui.selectable_value(&mut self.channel, i, format!("{}", i + 1));
                }
            });
        }
        self.channel = self.channel.min(channels.saturating_sub(1));
        self.params
            .input_channel
            .store(self.channel, Ordering::Relaxed);

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let running = self.engine.is_some();
            let text = if running { "Stop" } else { "Start" };
            if ui
                .add_sized([90.0, 28.0], egui::Button::new(text))
                .clicked()
            {
                if running {
                    self.stop();
                } else {
                    self.start();
                }
            } else if changed && running {
                self.start();
            }
            if ui.button("Refresh devices").clicked() {
                self.refresh_devices();
            }
            ui.label(&self.status);
        });

        ui.add_space(6.0);
        meter(ui, "In", self.params.in_level.get());
        meter(ui, "Out", self.params.out_level.get());

        ui.separator();
        if let Some(engine) = &mut self.engine {
            self.tuner.push(engine.tap.pop_iter());
            self.reading = self.tuner.update(engine.sample_rate as f32);
        }
        tuner_view(ui, self.reading);

        ui.separator();
        ui.label(egui::RichText::new("Presets").strong());
        ui.horizontal_wrapped(|ui| {
            for (name, preset) in &self.presets {
                if ui
                    .selectable_label(self.settings == *preset, *name)
                    .clicked()
                {
                    self.settings = *preset;
                }
            }
        });

        ui.separator();
        ui.label(egui::RichText::new("Amp").strong());
        let s = &mut self.settings;
        knob(ui, "Gain", &mut s.gain);
        knob(ui, "Bass", &mut s.bass);
        knob(ui, "Mid", &mut s.mid);
        knob(ui, "Tone", &mut s.tone);
        knob(ui, "Volume", &mut s.volume);

        section(ui, "Gate", &mut s.gate_on, |ui| {
            knob(ui, "Threshold", &mut s.gate);
        });

        let reduction = if s.limiter_on && self.engine.is_some() {
            -20.0 * self.params.reduction.get().max(0.001).log10()
        } else {
            0.0
        };
        section(ui, "Limiter", &mut s.limiter_on, |ui| {
            knob(ui, "Ceiling", &mut s.limiter);
            ui.horizontal(|ui| {
                ui.add_sized([90.0, 16.0], egui::Label::new("Reduction"));
                ui.add(
                    egui::ProgressBar::new((reduction / 24.0).clamp(0.0, 1.0))
                        .desired_width(290.0)
                        .desired_height(12.0)
                        .fill(egui::Color32::from_rgb(230, 170, 50)),
                );
            });
        });
    }

    fn right(&mut self, ui: &mut egui::Ui) {
        let s = &mut self.settings;
        ui.label(egui::RichText::new("Effects").strong());
        section(ui, "Compressor", &mut s.comp_on, |ui| {
            knob(ui, "Amount", &mut s.comp);
        });
        section(ui, "Auto wah", &mut s.wah_on, |ui| {
            knob(ui, "Sensitivity", &mut s.wah);
        });
        section(ui, "Distortion", &mut s.drive_on, |ui| {
            knob(ui, "Drive", &mut s.drive);
        });
        section(ui, "Phaser", &mut s.phaser_on, |ui| {
            knob(ui, "Speed", &mut s.phaser_rate);
            knob(ui, "Depth", &mut s.phaser_depth);
        });
        section(ui, "Chorus", &mut s.chorus_on, |ui| {
            knob(ui, "Speed", &mut s.chorus_rate);
            knob(ui, "Depth", &mut s.chorus_depth);
        });
        section(ui, "Tremolo", &mut s.tremolo_on, |ui| {
            knob(ui, "Speed", &mut s.tremolo_rate);
            knob(ui, "Depth", &mut s.tremolo_depth);
        });
        section(ui, "Delay", &mut s.delay_on, |ui| {
            knob(ui, "Time", &mut s.delay_time);
            knob(ui, "Repeats", &mut s.delay_feedback);
            knob(ui, "Mix", &mut s.delay_mix);
        });
        section(ui, "Reverb", &mut s.reverb_on, |ui| {
            knob(ui, "Amount", &mut s.reverb);
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Guitar Amp");
                ui.add_space(6.0);
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(420.0);
                        self.left(ui);
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.set_width(420.0);
                        self.right(ui);
                    });
                });
                self.params.store(&self.settings);
            });
        });
        ui.ctx().request_repaint_after(Duration::from_millis(33));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 760.0])
            .with_min_inner_size([460.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Guitar Amp",
        options,
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
