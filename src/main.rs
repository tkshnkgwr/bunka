fn main() {
    #[cfg(not(feature = "gui"))]
    {
        bunka::cli::run_cli();
    }

    #[cfg(feature = "gui")]
    {
        bunka::gui::run_gui();
    }
}
