use konnektoren_tui::prelude::App;
use ratatui::{prelude::Terminal, widgets::Widget};
use ratframe::NewCC;
use ratframe::RataguiBackend;

#[cfg(not(target_arch = "wasm32"))]
use ratframe::native_setup;

#[cfg(target_arch = "wasm32")]
use ratframe::wasm_setup;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_setup(WebTui::default());
}
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    native_setup(WebTui::default())
}
pub struct WebTui {
    terminal: Terminal<RataguiBackend>,
    app: App,
}

impl Default for WebTui {
    fn default() -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 80);
        let terminal = Terminal::new(backend).unwrap();

        let app = App::new();
        Self { terminal, app }
    }
}

impl NewCC for WebTui {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 80);
        let terminal = Terminal::new(backend).unwrap();
        let app = App::new();
        Self { terminal, app }
    }
    //matches index.html
    fn canvas_id() -> String {
        "the_canvas_id".into()
    }
}

impl eframe::App for WebTui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.terminal
            .draw(|frame| {
                let area = frame.size();
                self.app.render(area, frame.buffer_mut());
            })
            .expect("epic fail");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::ArrowLeft)) {
                self.app.previous_question();
            }
            if ui.input(|i| i.key_released(egui::Key::ArrowRight)) {
                self.app.next_question();
            }
            if ui.input(|i| i.key_released(egui::Key::Num0)) {
                self.app.solve_option(0).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num1)) {
                self.app.solve_option(1).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num2)) {
                self.app.solve_option(2).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num3)) {
                self.app.solve_option(3).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num4)) {
                self.app.solve_option(4).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num5)) {
                self.app.solve_option(5).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num6)) {
                self.app.solve_option(6).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num7)) {
                self.app.solve_option(7).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num8)) {
                self.app.solve_option(8).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Num9)) {
                self.app.solve_option(9).unwrap();
            }
            if ui.input(|i| i.key_released(egui::Key::Tab)) {
                self.app.next_challenge();
            }
            if ui.input(|i| i.modifiers.shift && i.key_released(egui::Key::Tab)) {
                self.app.previous_challenge();
            }
            if ui.input(|i| i.key_released(egui::Key::M)) {
                self.app.toggl_map();
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.draw_buttons(ui);
        });
    }
}

impl WebTui {
    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            if ui.button("Previous Question").clicked() {
                self.app.previous_question();
            }
            if ui.button("Next Question").clicked() {
                self.app.next_question();
            }
            if ui.button("Next Challenge").clicked() {
                self.app.next_challenge();
            }
            if ui.button("Previous Challenge").clicked() {
                self.app.previous_challenge();
            }
            if ui.button("Toggle Map").clicked() {
                self.app.toggl_map();
            }
        });
        ui.separator();
        ui.label("Select an Option:");
        ui.horizontal(|ui| {
            for option_number in 0..10 {
                if ui.button(format!("{}", option_number)).clicked() {
                    self.app.solve_option(option_number).unwrap_or_default();
                }
            }
        });
    }
}
