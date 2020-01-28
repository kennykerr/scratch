//use winrt::*;

//use winmd::*;
// fn main() {

//     let mut writer = RustWriter::new();
//     //writer.add_namespace(""windows.ui.composition"");
//     writer.add_namespace("Windows.Foundation.Collections");
//     let output = writer.write();
// }

winrt::import!(
    dependencies
        "os"
    modules
        //"windows.ui"
        "windows.foundation"
        //"windows.data.json"
);

//use windows::foundation::collections::*;
use windows::foundation::*;
use winrt::*;

fn main() -> winrt::Result<()> {
    // let uri = &Uri::create_uri("http://kennykerr.ca")?;
    // println!("domain: {}", uri.domain()?);

    // let d: IUriRuntimeClass = uri.into();
    // println!("domain: {}", d.domain()?);
    // println!("port: {}", d.port()?);

    // let s: IStringable = uri.into();
    // let value = s.to_string()?;
    // println!("stringable: {}", value);

    // //let o: Object = s.into();
    // //let tn = o.type_name()?;

    // unsafe {
    //     type vcall = extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode;
    //     let abi = s.abi() as *const *const vcall;
    //     // Or the preceding two lines in one:
    //     // let abi = s.abi() as *const *const extern "system" fn (winrt :: RawPtr, * mut winrt :: RawPtr,) -> winrt :: ErrorCode;
    //     let abi = (*abi).offset(6);

    //     let mut hstring = winrt::String::new();
    //     let hr = (*abi)(s.abi(), hstring.set_abi());
    //     println!("hr: {} string: {}", hr.0, hstring);

    //     // let mut hstring = std::ptr::null_mut();
    //     // let hr = (*abi)(s.abi(), &mut hstring);
    //     // let hstring: winrt::String = std::mem::transmute(hstring);
    //     // println!("hr: {} string: {}", hr.0, hstring);
    // }

    // println!("domain: {}", uri.domain()?);

    // let _v = uri.query_parsed()?;

    // call(uri)?;
    // call(uri.clone())?;
    // //call(d);
    // call(&s)?;
    // call(Uri::create_uri("http://kennykerr.ca")?)?;

    // let o: winrt::Object = s.query();
    // let s: IStringable = o.query();
    // println!("o: {}", s.to_string()?);

    // unsafe {
    //     let v: IVector<i32> = std::mem::zeroed();
    //     if false {
    //         v.at(123);
    //     }
    // }

    Ok(())
}

fn call<'a, T: Into<winrt::Param<'a, IStringable>>>(_s: T) -> winrt::Result<()> {
    Ok(())
}
