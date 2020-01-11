
winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui"
        //"windows.foundation"
);

use winrt::AsAbi;

fn main() -> winrt::Result<()> {
    let mut uri = Uri::create_uri(&winrt::String::from("http://kennykerr.ca"))?;
    let uri = IUriRuntimeClass::from(uri.detach_abi());
    println!("uri: {}", uri.domain()?);

    use windows::foundation::*;
    let a = GuidHelper::create_new_guid()?;
    println!("{:?}", a);

    let b = GuidHelper::empty()?;
    let c = Default::default();
    assert!(b == c);
    assert!(GuidHelper::equals(&b, &c)?);
    println!("{:?}", b);

    let d = winrt::Guid::from("11E158E9-778C-471F-92D0-5D54ED93855D");
    println!("{:?}", d);

    use windows::ui::*;
    let color = Colors::red()?;
    println!("{:?}", color);
    assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);
    println!("woot!");

    Ok(())
}

// fn main() -> winrt::Result<()> {
//     use winmd::*;
//     let mut writer = RustWriter::new();
//     //writer.add_namespace("Windows.Foundation");
//     //writer.add_namespace("Windows.UI.Composition");
//     writer.add_namespace("Windows.UI");

//     let tokens = writer.write();
//     //println!("{}", tokens.to_string());

//     Ok(())
// }
