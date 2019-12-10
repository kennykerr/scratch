winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
        "windows.ui.xaml.controls"
);

// fn main() -> winrt::Result<()> {
//     use windows::ui::*;
//     let color = Colors::alice_blue()?;

//     let s = format!("{:?}", color);
//     assert!(s == "Color { a: 255, r: 240, g: 248, b: 255 }");

//     println!("woot!");
//     Ok(())
// }

// fn module_to_namespace<'a>(reader: &'a winmd::Reader, module: &str) -> &'a str {
//     let namespaces = reader.namespaces();
//     for segment in module.split('.') {

//     }
// }

 fn main() {
    //  let reader = winmd::Reader::from_os().unwrap();

    // let ns = reader.find_namespace_lowercase("windows.ui.composition").unwrap();

    // println!("{}", ns.full_name());

 }
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
