// // import!(
// //     dependencies
// //         "os"
// //     modules
// //         "windows.foundation.collections"
// // );

// use winrt::*;
// //use windows::foundation::*;

// fn main() -> Result<()> {

//     // let uri = Uri::create_uri("https://kennykerr.ca")?;
//     // println!("{}", uri.domain()?);
//     // println!("{}", uri.to_string()?);

//     Ok(())
// }

trait RuntimeType { // All WinRT types
    type ValueType;
    type AbiType;
}

impl RuntimeType for i32 {
    type ValueType = Self;
    type AbiType = Self;
}

#[derive(PartialEq)]
struct Button {} // Some WinRT class

impl RuntimeType for Button {
    type ValueType = Self;
    type AbiType = usize;
}

// Represents an interface value type (backed by a COM ptr)
struct Rc<T: ?Sized> {
    placeholder: std::marker::PhantomData<T>,
}

// Some non-generic WinRT interface
trait IStringable {
    fn to_string(&self) -> String;
}

// Some generic WinRT interface
trait IVector<T: RuntimeType> {
    fn get_at(&self, index: u32) -> T::ValueType;
}

impl RuntimeType for dyn IStringable {
    type ValueType = Rc<Self>;
    type AbiType = usize;
}
impl<T> RuntimeType for dyn IVector<T> {
    type ValueType = Rc<Self>;
    type AbiType = usize;
}

impl IStringable for Rc<dyn IStringable> {
    fn to_string(&self) -> String {
        panic!("call abi");
    }
}
impl<T: RuntimeType> IVector<T> for Rc<dyn IVector<T>> {
    fn get_at(&self, _index: u32) -> T::ValueType {
        panic!("call abi");
    }
}

fn call1(v: &Rc<dyn IVector<i32>>) {
    assert!(v.get_at(1) == 0);
}

fn call2(v: &Rc<dyn IVector<Button>>) {
    assert!(v.get_at(2) == Button {});
}

fn call3(v: &Rc<dyn IVector<dyn IStringable>>) {
    // v.get_at(3);
}

fn call4(s: &Rc<dyn IStringable>) {
    s.to_string();
}

fn main() {}
