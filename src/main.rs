winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui"
);

fn main() -> winrt::Result<()> {
    use windows::ui::*;
    let color = Colors::red()?;

    println!("{:?}", color);
    //assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);

    println!("woot!");
    Ok(())
}
