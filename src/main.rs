mod window;

use crate::window::download_url::{DownloadUrl, SaveTarget};
use eframe::{run_native, App, Frame, NativeOptions};
use egui::{CentralPanel, Context};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
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
struct MainWindow {
    window_download_url: DownloadUrl,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Download").clicked() {
                self.window_download_url.is_show = true;
            }
        });

        if self.window_download_url.is_show {
            self.window_download_url.show_window(ctx);
        }

        let url = &mut self.window_download_url;
        let target = (url.urls).clone();
        if !(target.url.is_empty() || target.local_path.is_empty()) && url.is_start {
            url.is_start = false;
            tokio::spawn(async move {
                download_file_to_local_path(&target)
                    .await
                    .expect("TODO: panic message");
            });
        }
    }
}

async fn download_file_to_local_path(
    target: &SaveTarget,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(&target.local_path).join(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
    );

    let mut file = File::create(file_path)?;
    let response = reqwest::get(&target.url).await?;
    file.write_all(&response.bytes().await?)?;

    Ok(())
}
