use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.ui"
        //"windows.foundation"
);

fn call<'a, S:Into<param::String<'a>>>(value: S) {
    let ptr = value.into().as_abi_in();

    let mut a = winrt::String::from(ptr as *mut std::ffi::c_void);

    println!("hstring {}", a);

    a.detach_abi();
}



fn main() -> Result<()> {

    let mut a = winrt::String::from("winrt string");

    let rust = &("rust".to_string());

    a = rust.into();

     call(&a);
     call(a);
     call("slice");
     call("string".to_string());
    // call(a);
    // call("rust string".into());
   // call("call_a");

    let mut uri = Uri::create_uri(&String::from("http://kennykerr.ca"))?;
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

    let d = Guid::from("11E158E9-778C-471F-92D0-5D54ED93855D");
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
