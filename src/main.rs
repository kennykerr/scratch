use bindings::*;

fn main() -> winrt::Result<()> {
    clipboard()?;

    Ok(())
}

fn clipboard() -> winrt::Result<()> {
    use windows::application_model::data_transfer::*;

    let content = DataPackage::new()?;
    content.set_text("Rust/WinRT")?;
     
    Clipboard::set_content(content)?;
    Clipboard::flush()?;

    Ok(())
}
