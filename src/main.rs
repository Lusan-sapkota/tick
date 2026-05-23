mod app;
mod db;
mod models;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::Database::open()?;
    let tasks = db.load_tasks()?;
    let notes = db.load_notes()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 680.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tick",
        options,
        Box::new(|_cc| Box::new(app::TickApp::new(db, tasks, notes))),
    )?;

    Ok(())
}
