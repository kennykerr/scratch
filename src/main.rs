use winrt::*;
use bindings::*;

fn main() -> Result<()> {
    use windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    println!("{}", root.inner_text()?);

    Ok(())
}


// mod windows {
//     pub use winrt::foundation;
// }

// use winrt::Interface; // for .cast()
// use windows::foundation::Uri;
// use std::cmp::Ordering;

// struct Thing(i32);

// impl Ord for Thing {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.0.cmd(&other.0)
//     }
// }

// // impl PartialEq for Thing {
// //     fn eq(&self, other: &Self) -> bool {
// //         self.0 == other.0
// //     }
// // }

// fn main() -> Result<()> {
//     let mut map = std::collections::BTreeMap::new();
//     map.insert(Thing(1), 1);
//     map.insert(Thing(2), 2);

//     Ok(())
// }

// // #[winrt::implement(windows::foundation::{IStringable, IClosable})]
// // #[derive(Debug)]
// // struct Thing (
// //     String,
// // );

// // impl Drop for Thing {
// //     fn drop(&mut self) {
// //         println!("drop: {}", self.0);
// //     }
// // }

// // impl Thing {
// //     fn to_string(&self) -> Result<winrt::HString> {
// //         Ok(winrt::HString::from(&self.0))
// //     }

// //     fn close(&self) -> Result<()> {
// //         println!("close: {}", self.0);
// //         Ok(())
// //     }
// // }
