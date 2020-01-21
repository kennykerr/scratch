use winrt::*;

import!(
    dependencies
    "os"
modules
    "windows.ui"
    //"windows.foundation"
);

// struct IUriRuntimeClass {
//     ptr: RawPtr
// }

// #[repr(C)]
// struct abi_IUriRuntimeClass {
//     __0: usize,
//     __1: usize,
//     __2: usize,
//     __3: usize,
//     __4: usize,
//     __5: usize,
//     absolute_uri: extern "system" fn(RawPtr, &mut RawPtr) -> ErrorCode,
//     display_uri: extern "system" fn(RawPtr, &mut RawPtr) -> ErrorCode,
//     domain: extern "system" fn(RawPtr, &mut RawPtr) -> ErrorCode,
// }

// impl TypeGuid for IUriRuntimeClass {
//     fn type_guid() -> &'static Guid {
//         panic!("guid of IUriRuntimeClass");
//     }
// }

// impl IUriRuntimeClass {
//     fn domain(&self) -> Result<String> {
//         unsafe {
//             let mut __ok = std::mem::zeroed();
//             ((*(*(self.ptr as *const *const abi_IUriRuntimeClass))).domain)(
//                 self.ptr, &mut __ok
//             )
//             .ok_or(__ok.into())
//         }
//     }
// }

// struct IUriRuntimeClassFactory {
//     ptr: RawPtr
// }

// #[repr(C)]
// struct abi_IUriRuntimeClassFactory {
//     __0: usize,
//     __1: usize,
//     __2: usize,
//     __3: usize,
//     __4: usize,
//     __5: usize,
//     create_uri: extern "system" fn(RawPtr, RawPtr, &mut RawPtr) -> ErrorCode,
// }

// impl TypeGuid for IUriRuntimeClassFactory {
//     fn type_guid() -> &'static Guid {
//         static GUID: winrt::Guid = winrt::Guid::from_values(
//             0x44A9796F,0x723E,0x4FDF, &[0xA2,0x18,0x03,0x3E,0x75,0xB0,0xC0,0x84]
//         );
//         &GUID
//     }
// }

// impl From<RawPtr> for IUriRuntimeClassFactory {
//     fn from(value: RawPtr) -> Self {
//         unsafe { std::mem::transmute(value) }
//     }
// }

// impl IUriRuntimeClassFactory {
//     fn create_uri<'a, __0: Into<param::String<'a>>>(&self, value: __0) -> Result<Uri> {
//         unsafe {
//             let mut __ok = std::mem::zeroed();
//             ((*(*(self.ptr as *const *const abi_IUriRuntimeClassFactory))).create_uri)(
//                 self.ptr, value.into().as_abi(), &mut __ok
//             )
//             .ok_or(__ok.into())
//         }
//     }
// }

// struct Uri {
//     ptr: RawPtr
// }

// impl TypeName for Uri {
//     fn type_name() -> &'static str {
//         "Windows.Foundation.Uri"
//     }
// }

// impl Uri {
//     fn create_uri<'a, __0: Into<param::String<'a>>>(value: __0) -> Result<Uri> {
//         factory::<Uri, IUriRuntimeClassFactory>()?.create_uri(value)
//     }

//     fn domain(&self) -> Result<String> {
//         unsafe {
//             let mut __ok = std::mem::zeroed();
//             ((*(*(self.ptr as *const *const abi_IUriRuntimeClass))).domain)(
//                 self.ptr, &mut __ok
//             )
//             .ok_or(__ok.into())
//         }
//     }
// }

// impl From<RawPtr> for Uri {
//     fn from(ptr: RawPtr) -> Self {
//         Uri { ptr }
//     }
// }

fn main() -> Result<()> {
    use windows::foundation::*;

    let uri = &Uri::create_uri("http://kennykerr.ca")?;
    println!("domain: {}", uri.domain()?);

    let d : IUriRuntimeClass = uri.into();
    println!("domain: {}", d.domain()?);
    let s : IStringable = uri.into();
    let s = s.to_string()?;
    println!("stringable: {}", s);

    println!("domain: {}", uri.domain()?);

    Ok(())
}
