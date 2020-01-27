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
        "windows.ui"
        "windows.foundation.collections"
        "windows.data.json"
);

use windows::foundation::collections::*;
use windows::foundation::*;
use winrt::*;

// struct abi_IStringable
//         {
//             __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
//             __5 : usize, to_string : extern "system" fn
//             (winrt :: RawPtr, * mut winrt :: RawPtr,) -> winrt :: ErrorCode,
//         }

// struct Thing {
//     value: i32
// }

// impl Thing {
//     fn abi(&self) -> i32 {
//         self.value
//     }

//     fn set_abi(&mut self) -> &mut i32 {
//         &mut self.value
//     }
// }

fn main() -> winrt::Result<()> {



    unsafe {
        let s1 = winrt::String::from("s1");
        let s2 : winrt::RawPtr = std::mem::transmute(s1);

        let mut s3 = winrt::String::new();
        *s3.set_abi() = s2;
        //s3 = std::mem::transmute(s2);
        println!("s3: {}", s3);
    }


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
    //     type vcall = extern "system" fn (winrt :: RawPtr, * mut winrt :: RawPtr,) -> winrt :: ErrorCode;
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
    //     let v: IVector::<i32> = std::mem::zeroed();
    //     if false {
    //     v.at(123);
    //     }
    //  }

    Ok(())
}

fn call<'a, T: Into<winrt::Param<'a, IStringable>>>(_s: T) -> winrt::Result<()> {
    Ok(())
}
