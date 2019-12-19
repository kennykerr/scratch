// winrt::import!(
//     dependencies
//         "os"
//     modules
//         "windows.ui"
// );

// use windows::ui::*;

// fn main() -> winrt::Result<()> {
//     let color = Colors::red()?;

//     println!("{:?}", color);

//     println!("woot!");
//     Ok(())
// }

fn main() {
    use winmd::*;

    let reader = Reader::from_os().unwrap();
    let t = reader.find_type("Windows.Foundation.IStringable").unwrap();
    let a = t.find_attribute("Windows.Foundation.Metadata.GuidAttribute").unwrap();

    for (_, sig) in a.arguments() {
        match sig {
            ArgumentSig::U8(value) => print!(" {:x}", value),
            ArgumentSig::U16(value) => print!(" {:x}", value),
            ArgumentSig::U32(value) => print!(" {:x}", value),
            _ => panic!(),
        }
    }
}