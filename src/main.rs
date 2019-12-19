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

    println!("woot!");
    Ok(())
}
