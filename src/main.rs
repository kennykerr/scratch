use winmd::*;

fn main() -> winrt::Result<()> {
    let mut writer = RustWriter::new();
    writer.add_namespace("Windows.Foundation");
    writer.add_namespace("Windows.UI.Composition");

    let tokens = writer.write();
    //println!("{}", tokens.to_string());

    Ok(())
}

// winrt::import!(
//     dependencies
//         "os"
//     modules
//         "windows.ui"
//         //"windows.foundation"
// );

// use windows::ui::*;

// fn main() -> winrt::Result<()> {
//     //test_reader();

//     let a = winrt::String::new();
//     assert!(a.is_empty());
//     assert!(a.len() == 0);
//     assert!(a.as_chars().len() == 0);

//     let hello = winrt::String::from("Hello");
//     assert!(!hello.is_empty());
//     assert!(hello.len() == 5);

//     let color = Colors::red()?;
//     println!("{:?}", color);
//     assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);
//     println!("woot!");

//     Ok(())
// }

// // fn test_reader() {
// //     let reader = winmd::Reader::from_files(&[
// //         r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd".to_string(),
// //     ])
// //     .unwrap();
// //     let t = reader
// //         .find_type("Windows.Foundation.IAsyncOperationWithProgress`2")
// //         .unwrap();
// //     let g = t.generics();

// //     if g.is_empty() {
// //         println!("{} is not generic", t.name());
// //     } else {
// //         println!("{} is generic", t.name());

// //         for param in g {
// //             print!("{}, ", param.name());
// //         }
// //     }
// // }
