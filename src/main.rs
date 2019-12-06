winrt::import!(
    dependencies
        "os"
    modules
        // "windows.storage"
        // "windows.ui.composition"
        // "windows.ui.xaml.controls"
        "windows.foundation"
        "windows.ui"
);

fn main() -> winrt::Result<()> {
    use windows::ui::*;
    let color = Colors::alice_blue()?;

    let s = format!("{:?}", color);
    assert!(s == "Color { a: 255, r: 240, g: 248, b: 255 }");

    println!("woot!");
    Ok(())
}
