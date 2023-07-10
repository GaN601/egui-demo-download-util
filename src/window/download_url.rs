use egui::Context;
use egui::Window;

#[derive(Default)]
pub struct DownloadUrl {
    pub is_show: bool,
    pub urls: SaveTarget,
    pub is_start: bool,
}
#[derive(Default, Clone)]
pub struct SaveTarget {
    pub url: String,
    pub local_path: String,
}

impl DownloadUrl {
    pub fn show_window(&mut self, ctx: &Context) {
        let text = &mut self.urls;
        let _ = Window::new("Download Url")
            .open(&mut self.is_show.clone())
            .show(ctx, |ui| {
                ui.heading("Download Url");
                ui.text_edit_singleline(&mut text.url);
                ui.text_edit_singleline(&mut text.local_path);
                if ui.button("Select Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        text.local_path = path.display().to_string();
                    }
                }
                if ui.button("Download").clicked() {
                    self.is_show = false;
                    self.is_start = true;
                }
            });
    }
}
