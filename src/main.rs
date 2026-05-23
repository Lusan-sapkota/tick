mod app;
mod db;
mod models;
mod theme;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::Database::open()?;
    let tasks = db.load_tasks()?;
    let notes = db.load_notes()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([860.0, 560.0]),
        depth_buffer: 0,
        stencil_buffer: 0,
        multisampling: 0,
        vsync: false,
        ..Default::default()
    };

    eframe::run_native(
        "Tick",
        options,
        Box::new(|cc| {
            theme::apply_theme(&cc.egui_ctx);
            Box::new(app::TickApp::new(db, tasks, notes))
        }),
    )?;

    Ok(())
}
