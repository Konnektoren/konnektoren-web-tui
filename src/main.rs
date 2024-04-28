use konnektoren_tui::prelude::App;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Paragraph, Widget},
};
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
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();

        let app = App::new();
        Self { terminal, app }
    }
}

impl NewCC for WebTui {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();
        Self { terminal, app }
    }
    //matches index.html
    fn canvas_id() -> String {
        "the_canvas_id".into()
    }
}

impl eframe::App for WebTui {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal
            .draw(|frame| {
                let area = frame.size();
                self.app.render(area, frame.buffer_mut());
            })
            .expect("epic fail");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                self.app.exit();
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::T)) {
                ()
            }
            //KeyCode::Char(c) => app.on_key(c),
        });
    }
}
