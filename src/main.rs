import!(
    dependencies
        "os"
    modules
        "windows.foundation.collections"
);

use windows::foundation::*;
use winrt::*;

struct Thing {}

impl traits::IStringable for Thing {
    fn to_string(&self) -> Result<HString> {
        panic!("call abi");
    }
}

fn main() -> Result<()> {
    let uri = Uri::create_uri("https://kennykerr.ca")?;
    println!("{}", uri.domain()?);
    println!("{}", uri.to_string()?);

    Ok(())
}

// mod winrt {
//     pub trait RuntimeType {
//         type Abi;
//     }

//     impl RuntimeType for i32 {
//         type Abi = Self;
//     }

//     pub struct Rc<T: ?Sized> {
//         placeholder: std::marker::PhantomData<T>,
//     }

//     pub type RawPtr = *mut std::ffi::c_void;
// }

// pub mod windows {
//     pub mod foundation {
//         pub struct Button {
//             ptr: crate::winrt::RawPtr,
//             a: abi::IStringable,
//         }

//         impl crate::winrt::RuntimeType for Button {
//             type Abi = crate::winrt::RawPtr;
//         }

//         mod abi {
//             pub struct IStringable {
//                 __base : [usize ; 6],
//                 to_string : extern "system" fn (crate::winrt::RawPtr, * mut crate::winrt::RawPtr,) -> i32,
//             }
//         }

//         pub mod traits {
//             pub trait IStringable {
//                 fn to_string(&self) -> String;
//             }

//             pub trait IVector<T: crate::winrt::RuntimeType> {
//                 fn get_at(&self, index: u32) -> T;
//                 fn test(s: &super::IStringable);
//             }
//         }

//         pub struct IStringable {
//             ptr: crate::winrt::RawPtr,
//         }
//         pub struct IVector<T: crate::winrt::RuntimeType> {
//             ptr: crate::winrt::RawPtr,
//             placeholder: std::marker::PhantomData<T>,
//         }

//         impl crate::winrt::RuntimeType for IStringable {
//             type Abi = crate::winrt::RawPtr;
//         }
//         impl<T: crate::winrt::RuntimeType> crate::winrt::RuntimeType for IVector<T> {
//             type Abi = crate::winrt::RawPtr;
//         }

//         impl IStringable {
//             pub fn to_string(&self) -> String {
//                 panic!("call abi");
//             }
//         }
//         impl<T: crate::winrt::RuntimeType> IVector<T> {
//             pub fn get_at(&self, _index: u32) -> T {
//                 panic!("call abi");
//             }
//         }
//     }
// }

// use windows::foundation::*;

// struct Custom {}
// impl traits::IStringable for Custom {
//     fn to_string(&self) -> String {
//         panic!("call abi");
//     }
// }

// fn call1(v: &IVector<i32>) {
//     assert!(v.get_at(1) == 0);
// }

// fn call2(v: &IVector<Button>) {
//     v.get_at(2);
// }

// fn call3(v: &IVector<IStringable>) {
//     v.get_at(3).to_string();
// }

// fn call4(s: &IStringable) {
//     s.to_string();
// }

// fn main() {}
