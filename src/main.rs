

// struct Thing<T> {
//     ptr: *mut std::ffi::c_void,
//     __0: std::marker::PhantomData<T>
// }

// impl<T> Type<T> {
//     fn test(value: T) {}
// }

// use winmd::*;

// fn main() -> winrt::Result<()> {
//     let mut writer = RustWriter::new();
//     //writer.add_namespace("Windows.Foundation");
//     //writer.add_namespace("Windows.UI.Composition");
//     writer.add_namespace("Windows.UI");

//     let tokens = writer.write();
//     //println!("{}", tokens.to_string());

//     Ok(())
// }

// struct Thing<T: winrt::AsAbi> {

// }

winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui"
        //"windows.foundation"
);

fn main() -> winrt::Result<()> {
    // let t = Thing::<u32> { ptr:, .. Default::default() };

    //     //test_reader();

    //     let a = winrt::String::new();
    //     assert!(a.is_empty());
    //     assert!(a.len() == 0);
    //     assert!(a.as_chars().len() == 0);

    //     let hello = winrt::String::from("Hello");
    //     assert!(!hello.is_empty());
    //     assert!(hello.len() == 5);

    use windows::foundation::*;
    let a = GuidHelper::create_new_guid()?;
    println!("{:?}", a);

    let b = GuidHelper::empty()?;
    let c = Default::default();
    assert!(b == c);
    assert!(GuidHelper::equals(&b, &c)?);

    let d = winrt::Guid::from("11E158E9-778C-471F-92D0-5D54ED93855D");
    println!("{:?}", d);

    use windows::ui::*;
    let color = Colors::red()?;
    println!("{:?}", color);
    assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);
    println!("woot!");

    Ok(())
}

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
