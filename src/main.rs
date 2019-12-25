winrt::import!(
    dependencies
        "os"
    modules
        //"windows.ui"
        "windows.foundation"
);

use windows::ui::*;

fn main() -> winrt::Result<()> {
    //test_reader();

    let color = Colors::red()?;
    println!("{:?}", color);
    assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);
    println!("woot!");

    Ok(())
}

fn test_reader() {
    let reader = winmd::Reader::from_files(&[
        r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd".to_string(),
    ])
    .unwrap();
    let t = reader
        .find_type("Windows.Foundation.IAsyncOperationWithProgress`2")
        .unwrap();
    let g = t.generics();

    if g.is_empty() {
        println!("{} is not generic", t.name());
    } else {
        println!("{} is generic", t.name());

        for param in g {
            print!("{}, ", param.name());
        }
    }
}
