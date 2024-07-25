slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    
    //Define variable as a new AppWindow element
    let ui = AppWindow::new()?;
    
    //Run AppWindow element with or without parameters
    ui.run()
}
