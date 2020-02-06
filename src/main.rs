// use winmd::*;

// fn main() {
//     let mut w = RustWriter::new();
//     w.add_namespace("windows.foundation.collections");
//     w.write();
// }

// // fn main() {
// //     let r = Reader::from_os().unwrap();
// //     let class = r.resolve("Windows.AI.MachineLearning.Preview.LearningModelBindingPreview");

// //     for attribute in class.attributes(&r) {
// //         let (_, name) = attribute.name(&r);
// //         println!("{}", name);

// //         if name == "ActivatableAttribute" {
// //             let args = attribute.arguments(&r);
// //         }
// //     }
// // }

winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation.collections"
        "windows.ui.composition"
);

use windows::foundation::collections::*;
use windows::foundation::*;
use winrt::*;

// // impl QueryType for IVectorView<IWwwFormUrlDecoderEntry> {

// // }

// // impl QueryType for IVectorView<IWwwFormUrlDecoderEntry> {
// //     fn type_guid() -> &'static winrt::Guid {
// //         static GUID: winrt::Guid = winrt::Guid::from_values(
// //             // b1f00d3b-1f06-5117-93ea-2a0d79116701
// //             0xb1f00d3b,
// //             0x1f06,
// //             0x5117,
// //             &[0x93, 0xea, 0x2a, 0x0d, 0x79, 0x11, 0x67, 0x01],
// //         );
// //         &GUID
// //     }
// // }

// fn c() -> Option<u32> {
//     Some(123)
// }

// fn b() -> Option<u32> {
//     let v = c()?;
//     Some(v)
// }

// fn a() -> Option<u32> {
//     let v = b()?;
//     Some(v)
// }

fn main() -> winrt::Result<()> {
    {
        assert!(AsyncStatus::default() == AsyncStatus::Canceled);
    }

    {
        let uri = Uri::create_uri("http://kennykerr.ca")?;

        assert!(!uri.is_empty());
        let decoder = uri.query_parsed()?;
        assert!(!decoder.is_empty());

        //decoder.size();

        // TODO: need generic guids!
        // let v: IVectorView<IWwwFormUrlDecoderEntry> = decoder.into();
        // assert!(v.is_empty());
    }

    {
        let uri = &Uri::create_uri("http://kennykerr.ca")?;
        println!("domain: {}", uri.domain()?);

        let d: IUriRuntimeClass = uri.into();
        println!("domain: {}", d.domain()?);
        println!("port: {}", d.port()?);

        let s: IStringable = uri.into();
        let value = s.to_string()?;
        println!("stringable: {}", value);

        println!("QI stringable: {}", uri.to_string()?);
    }

    {
        let object = PropertyValue::create_string("hello")?;
        let pv: IPropertyValue = object.query();
        let value = pv.get_string()?;
        println!("pv {}", value);
    }

    {
        let object = PropertyValue::create_u_int32_array(&[1, 2, 3])?;
        let pv: IPropertyValue = object.query();
        let mut array = Array::new();
        pv.get_u_int32_array(&mut array)?;

        for i in array.as_slice() {
            println!("a: {}", i);
        }
    }

    //     {}

    //     // //let o: Object = s.into();
    //     // //let tn = o.type_name()?;

    //     // unsafe {
    //     //     type vcall = extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode;
    //     //     let abi = s.abi() as *const *const vcall;
    //     //     // Or the preceding two lines in one:
    //     //     // let abi = s.abi() as *const *const extern "system" fn (winrt :: RawPtr, * mut winrt :: RawPtr,) -> winrt :: ErrorCode;
    //     //     let abi = (*abi).offset(6);

    //     //     let mut hstring = winrt::String::new();
    //     //     let hr = (*abi)(s.abi(), hstring.set_abi());
    //     //     println!("hr: {} string: {}", hr.0, hstring);

    //     //     // let mut hstring = std::ptr::null_mut();
    //     //     // let hr = (*abi)(s.abi(), &mut hstring);
    //     //     // let hstring: winrt::String = std::mem::transmute(hstring);
    //     //     // println!("hr: {} string: {}", hr.0, hstring);
    //     // }

    //     // println!("domain: {}", uri.domain()?);

    //     // let _v = uri.query_parsed()?;

    //     // call(uri)?;
    //     // call(uri.clone())?;
    //     // //call(d);
    //     // call(&s)?;
    //     // call(Uri::create_uri("http://kennykerr.ca")?)?;

    //     // let o: winrt::Object = s.query();
    //     // let s: IStringable = o.query();
    //     // println!("o: {}", s.to_string()?);

    //     // unsafe {
    //     //     let v: IVector<i32> = std::mem::zeroed();
    //     //     if false {
    //     //         v.at(123);
    //     //     }
    //     // }

    Ok(())
}

// // fn call<'a, T: Into<winrt::Param<'a, IStringable>>>(_s: T) -> winrt::Result<()> {
// //     Ok(())
// // }
