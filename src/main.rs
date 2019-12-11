winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
        "windows.ui.xaml.controls"
        "windows.ui.composition"
        "windows.ui.composition"
        "windows.ui"
        "windows.storage"
);



fn main() -> winrt::Result<()> {
    use windows::ui::*;
    let color: Color = Default::default(); // Colors::alice_blue()?;

    let s = format!("{:?}", color);
    assert!(s == "Color { a: 255, r: 240, g: 248, b: 255 }");

    println!("woot!");
    Ok(())
}

//  fn main() {

//     let reader = winmd::Reader::from_os().unwrap();

//      let ns = reader.find_namespace_lowercase("windows.foundation").unwrap();

//      println!("ns {}", ns.full_name());

//     for t in ns.interfaces() {
//         println!("type {}", t.name().unwrap());
//     }

//  }
//     let namespaces = reader.namespaces();

//     for ns in namespaces.iter() {
//         println!("namespace {}", ns.name());

//         for ns in ns.namespaces() {
//             println!("  namespace {}", ns.name());

//             for ns in ns.namespaces() {
//                 println!("    namespace {}", ns.name());
//             }
//         }
//     }

//     let ns = reader.find_namespace("Windows.UI.Composition").unwrap();
//     println!("{}", ns.name());

//     let t = reader.find_type("Windows.Foundation.IStringable").unwrap();
//     println!("{}", t.name().unwrap());
// }
