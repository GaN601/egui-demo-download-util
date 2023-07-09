use eframe::{run_native, App, Frame, NativeOptions};
use egui::Context;

fn main() {
    println!("Hello, world!");
    let option = NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    run_native(
        "egui download util",
        option,
        Box::new(|_c| Box::<MainWindow>::default()),
    );
}
#[derive(Default)]
struct MainWindow {}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {}
}
