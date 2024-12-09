use eframe::egui;
use egui::ViewportBuilder;
use egui_plot::{Line, Plot, PlotPoints};

mod agent;
mod env;

use env::IsingEnvironment;

struct IsingApp {
    env: IsingEnvironment,
    temperature: f64,
    j_coupling: f64,
    running: bool,
    steps_per_frame: usize,
}

impl IsingApp {
    fn new() -> Self {
        Self {
            env: IsingEnvironment::new(50, 2.0, 1.0),
            temperature: 2.0,
            j_coupling: 1.0,
            running: false,
            steps_per_frame: 100,
        }
    }
}

impl eframe::App for IsingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            for _ in 0..self.steps_per_frame {
                self.env.step();
            }
            ctx.request_repaint();
        }

        // 右侧控制面板
        egui::SidePanel::right("controls")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Ising Model Controls");
                
                ui.add(egui::Slider::new(&mut self.temperature, 0.1..=10.0).text("Temperature"));
                if self.temperature != self.env.temperature {
                    self.env.temperature = self.temperature;
                }

                ui.add(egui::Slider::new(&mut self.j_coupling, -2.0..=2.0).text("J Coupling"));
                if self.j_coupling != self.env.j_coupling {
                    self.env.j_coupling = self.j_coupling;
                }

                ui.add(egui::Slider::new(&mut self.steps_per_frame, 1..=500)
                    .text("Steps per Frame")
                    .logarithmic(true));

                if ui.button(if self.running { "Stop" } else { "Start" }).clicked() {
                    self.running = !self.running;
                }

                if ui.button("Reset").clicked() {
                    self.env = IsingEnvironment::new(50, self.temperature, self.j_coupling);
                }
            });

        // 中央面板显示晶格
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_width().min(ui.available_height());
            let cell_size = size / self.env.size as f32;
            
            let painter = ui.painter();
            let rect = egui::Rect::from_min_size(
                ui.cursor().min,
                egui::vec2(size, size),
            );

            for i in 0..self.env.size {
                for j in 0..self.env.size {
                    let x = rect.min.x + j as f32 * cell_size;
                    let y = rect.min.y + i as f32 * cell_size;
                    let color = if self.env.lattice[i][j].spin > 0 {
                        egui::Color32::from_gray(180)
                    } else {
                        egui::Color32::BLACK
                    };
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(cell_size, cell_size),
                        ),
                        0.0,
                        color,
                    );
                }
            }
            ui.allocate_rect(rect, egui::Sense::hover());
        });

        // 底部面板用于显示能量和磁化强度图
        egui::TopBottomPanel::bottom("plots")
            .default_height(250.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Energy");
                        let energy_points: PlotPoints = self.env.energy_history
                            .iter()
                            .enumerate()
                            .map(|(i, &e)| [i as f64, e])
                            .collect();
                        
                        Plot::new("energy_plot")
                            .height(200.0)
                            .width(ui.available_width() / 2.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(Line::new(energy_points).name("Energy"));
                            });
                    });

                    ui.vertical(|ui| {
                        ui.heading("Magnetization");
                        let mag_points: PlotPoints = self.env.magnetization_history
                            .iter()
                            .enumerate()
                            .map(|(i, &m)| [i as f64, m])
                            .collect();

                        Plot::new("magnetization_plot")
                            .height(200.0)
                            .width(ui.available_width())
                            .show(ui, |plot_ui| {
                                plot_ui.line(Line::new(mag_points).name("Magnetization"));
                            });
                    });
                });
            });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Ising Model Simulation",
        native_options,
        Box::new(|_cc| Box::new(IsingApp::new()))
    )
}
