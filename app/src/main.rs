use app::app::Flopper;

fn main() -> eframe::Result<()> {

    let opts = eframe::NativeOptions::default();
    eframe::run_native(
        "Flopper",
        opts,
        Box::new(|cc| Box::new(Flopper::new(cc)))
    )
}