
slint::slint! {
    export { MainWindow } from "ui/app-window.slint";
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}