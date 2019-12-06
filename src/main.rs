winrt::import!(
    dependencies
        "os"
    modules
        "windows.storage"
        "windows.ui.composition"
);

fn main() -> winrt::Result<()> {
    use windows::ui::*;
    let color = Colors::alice_blue()?;

    assert!(color.a == 255);
    assert!(color.r == 240);
    assert!(color.g == 248);
    assert!(color.b == 255);

    println!("woot!");

    Ok(())
}
