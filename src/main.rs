include!(concat!(env!("OUT_DIR"), "/winrt.rs"));

fn main() -> winrt::Result<()> {
    use windows::application_model::data_transfer::*;

    let content = DataPackage::new()?;
    content.set_text("Rust/WinRT")?;
     
    Clipboard::set_content(content)?;
    Clipboard::flush()?;

    let view = Clipboard::get_content()?;

    for format in view.available_formats()? {
        println!("{}", format);
    }

    //println!("{}", view.get_text_async()?.get()?);

    Ok(())
}
