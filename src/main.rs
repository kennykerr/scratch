winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui"
);

use windows::ui::*;

fn main() -> winrt::Result<()> {
    let color = Colors::red()?;

    println!("{:?}", color);
    assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);

    println!("woot!");
    Ok(())
}
