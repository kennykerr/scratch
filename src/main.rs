
//  use winmd::*;
//  use proc_macro2::TokenStream;

//     fn interfaces(r:&Reader, t:&TypeDef) -> Vec<InterfaceInfo> {
//         let mut result = Vec::<InterfaceInfo>::new();
    
//         for i in t.interfaces(r) {
//             let default = i.has_attribute(r, "Windows.Foundation.Metadata", "DefaultAttribute");
//             let overridable = i.has_attribute(r, "Windows.Foundation.Metadata", "OverridableAttribute");
//             let mut generics = Vec::new();
    
//             let definition = match i.interface(r) {
//                 TypeDefOrRef::TypeDef(value) => value,
//                 TypeDefOrRef::TypeRef(value) => value.resolve(r),
//                 TypeDefOrRef::TypeSpec(value) => {

//                     let sig = value.signature(r);

    
//                     sig.sig_type().resolve(r)
//                 }
//             };
    
//             if let Err(index) = result.binary_search_by_key(&definition, |info|info.definition) {
//                 let exclusive = definition.has_attribute(r, "Windows.Foundation.Metadata", "ExclusiveToAttribute");
//                 result.insert(index, InterfaceInfo{definition, generics, default, overridable, exclusive});
//             }
//         }
    
//         result
//     }


// struct InterfaceInfo {
//     definition: TypeDef,
//     generics: Vec<Vec<TokenStream>>,
//     default: bool,
//     overridable: bool,
//     exclusive: bool,
// }



// fn main() {
//     let r = &Reader::from_os().unwrap();
//     let t = r.resolve("Windows.Foundation.WwwFormUrlDecoder");

//     for i in interfaces(r, &t) {
//         println!("{} - default:{}", i.definition.name(r), i.default);
//     }
// }

 use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.ui"
        //"windows.foundation"
);

fn call<'a, S: Into<param::String<'a>>>(value: S) {
    // TODO: maybe custom Into trait that avoids the .into()
    // Maybe a AsParamAbi
    let ptr = value.into().as_abi_in();

    let mut a = winrt::String::from(ptr as *mut std::ffi::c_void);

    println!("hstring {}", a);

    a.detach_abi();
}

struct Thing {}

impl<'a> Into<winrt::param::String<'a>> for Thing {
    fn into(self) -> winrt::param::String<'a> {
        param::String::Ref("Thing")
    }
}

fn main() -> Result<()> {
    let mut a = winrt::String::from("winrt string");

    let rust = &("rust".to_string());

    a = rust.into();

    call(Thing {});
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
//     writer.add_namespace("Windows.Foundation.Collections");
//     //writer.add_namespace("Windows.UI.Composition");
//     //writer.add_namespace("Windows.UI");

//     let tokens = writer.write();
//     //println!("{}", tokens.to_string());

//     Ok(())
// }
