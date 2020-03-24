use winmd::*;

fn main() {
    let reader = Reader::from_os();

    // for ns in reader.namespaces() {
    //     // println!("namespace {}", ns);

    //     // if ns != "Windows.Foundation" {
    //     //     continue;
    //     // }

    //     for t in reader.namespace_types(ns) {
    //         println!("type: {}.{}", reader.type_def_namespace(*t), reader.type_def_name(*t));
    //         match reader.type_info(*t) {
    //             Type::Interface(info) => println!("  interface {}", info.ident.name),
    //             Type::Class(info) => println!("  class {}", info.ident.name),
    //             Type::Struct(info) => println!("  struct {}", info.ident.name),
    //             Type::Delegate(info) => println!("  delegate {}", info.ident.name),
    //             Type::Enum(info) => {
    //                 println!("  enum {}", info.ident.name);
    //                 // for f in info.fields {
    //                 //     println!("    {}", f.name);
    //                 // }
    //             }
    //         }
    //     }
    // }

    let row = reader.type_def_from_type_name("Windows.Foundation", "Uri");

    let info = reader.type_info(row);

    println!("{:?}", info);

    // for def in reader.namespace_types("Windows.Foundation") {
    //     println!("  {}", def.name());
    // }

    // let def = reader.resolve_type_name("Windows.Foundation.Uri");
    // println!("\n{}: {}", def.namespace(), def.name());

    // let def = reader.resolve_type_name("Windows.Foundation.Point");
    // for field in def.fields() {
    //     println!("field: {}", field.name());
    // }

    // let def = reader.resolve_type_name("Windows.UI.Composition.SpriteVisual");
    // let def = def.extends();
    // println!("{}.{}", def.namespace(), def.name());

    // for ns in reader.namespaces() {
    //     println!("{}", ns);

    // }
}

// use windows::foundation::collections::*;
// use windows::foundation::*;
// use winrt::*;

// use windows::foundation::traits as wf_traits;

// import!(
//     dependencies:
//         "os"
//     modules:
//         "windows.foundation.collections"
// );

// // TODO:
// //  The class macro adds a pointer to a static vtbl for each interface.
// //  Implements the From trait to allow conversion to wrapped COM object
// // from any Stringable by moving it into the COM object.
// #[class(
//     implements(IStringable, IClosable),
//     type_name("Windows.Foundation.Stringable")
// )]
// struct Stringable {
//     value: String,
// }

// impl wf_traits::IStringable for Stringable {
//     fn to_string(&self) -> Result<HString> {
//         Ok((&self.value).into())
//     }
// }

// // impl TypeGuid for IVectorView<IWwwFormUrlDecoderEntry> {
// //     fn type_guid() -> &'static Guid {
// //         static GUID: Guid = Guid::from_values(
// //             0x00000035,
// //             0x0000,
// //             0x0000,
// //             &[0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
// //         );
// //         &GUID
// //     }
// // }

// use sha1::*;

// fn main() -> Result<()> {
//     let hash = Sha1::from("hello world").digest().bytes();
//     println!("{:?}", hash.get(..16));

//     let uri = &Uri::create_uri("https://kennykerr.ca")?;
//     println!("{}", uri.domain()?);
//     println!("{}", uri.to_string()?);

//     let s: IStringable = uri.into();
//     println!("{}", s.to_string()?);

//     let decoder = uri.query_parsed()?;

//     let o: Object = uri.into();

//     //let size = decoder.size();

//     let g = <IStringable as winrt::TypeGuid>::type_guid();

//     Ok(())
// }

// // use winrt::*;

// // fn call<'a, T:Into<Param<'a, Guid>>>(value:T) {

// // }

// // fn main(){
// //     let g : Guid = Default::default();
// //     call(g);
// // }
