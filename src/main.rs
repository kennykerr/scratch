use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::*;
use std::iter::FromIterator;
use winmd::*;

#[derive(Default)]
struct TypeLimits(BTreeSet<String>);

impl TypeLimits {
    fn insert(&mut self, reader: &Reader, namespace: &str) {
        let found = reader
            .types
            .keys()
            .find(|name| name.to_lowercase() == namespace)
            .unwrap_or_else(|| panic!("Namespace `{}` not found in winmd files", namespace));

        let mut namespace = found.as_str();
        self.0.insert(namespace.to_string());

        while let Some(pos) = namespace.rfind('.') {
            namespace = namespace.get(..pos).unwrap();

            if reader.types.contains_key(namespace) {
                self.0.insert(namespace.to_string());
            }
        }
    }
}

#[derive(Default)]
struct TypeStage(BTreeMap<TypeDef, Type>);

impl TypeStage {
    fn from_limits(reader: &Reader, limits: &TypeLimits) -> TypeStage {
        let mut stage: TypeStage = Default::default();

        for namespace in &limits.0 {
            for def in reader.namespace_types(&namespace) {
                stage.insert(reader, *def);
            }
        }

        stage
    }

    fn insert(&mut self, reader: &Reader, def: TypeDef) {
        if !self.0.contains_key(&def) {
            let name = def.name(reader);
            //println!("{}.{}", name.0, name.1);
            let info = def.info(reader);
            let depends = info.dependencies();
            self.0.insert(def, info);
            for def in depends {
                self.insert(reader, def);
            }
        }
    }

    fn into_tree(self) -> TypeTree {
        let mut tree: TypeTree = Default::default();
        self.0
            .into_iter()
            .for_each(|(_, t)| tree.insert(t.name().namespace.clone(), t));
        tree
    }
}

#[derive(Default)]
struct TypeNamespace(BTreeMap<String, TypeTree>);

impl TypeNamespace {
    fn into_stream(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, tree) in self.0.iter() {
            let name = write_ident(name);
            let tree = tree.into_stream();

            tokens.push(quote! {
                pub mod #name {
                    #tree
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}

#[derive(Default)]
struct TypeTree {
    types: Vec<Type>,
    namespaces: TypeNamespace,
}

impl TypeTree {
    fn insert(&mut self, namespace: String, t: Type) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .0
                .entry(namespace[..pos].to_string())
                .or_default()
                .insert(namespace[pos + 1..].to_string(), t);
        } else {
            self.namespaces
                .0
                .entry(namespace)
                .or_default()
                .types
                .push(t);
        }
    }

    fn into_stream(&self) -> TokenStream {
        TokenStream::from_iter(
            self.types
                .iter()
                .map(|t| t.into_stream())
                .chain(std::iter::once(self.namespaces.into_stream())),
        )
    }
}

fn main() {
    let mut reader = &Reader::from_os();

    let mut limits: TypeLimits = Default::default();
    limits.insert(reader, "windows.foundation");

    let stage = TypeStage::from_limits(reader, &limits);

    println!("count: {}", stage.0.len());

    let tree = stage.into_tree();

    println!("tree");

    let stream = tree.into_stream();

    println!("stream");

    println!("{}", stream.to_string());

    // for ns in reader.namespaces() {
    //     // println!("namespace {}", ns);

    //     // if ns != "Windows.Foundation" {
    //     //     continue;
    //     // }

    //     for t in reader.namespace_types(ns) {
    //         println!("type: {:?}", t.name(&reader),);
    //         match reader.type_info(*t) {
    //             Type::Interface(info) => println!("  interface {}", info.name.name),
    //             Type::Class(info) => println!("  class {}", info.name.name),
    //             Type::Struct(info) => println!("  struct {}", info.name.name),
    //             Type::Delegate(info) => println!("  delegate {}", info.name.name),
    //             Type::Enum(info) => println!("  enum {}", info.name.name),
    //         }
    //     }
    // }

    // let def = reader.resolve(("Windows.UI.Composition", "SpriteVisual"));

    // let info = reader.type_info(def);

    // println!("{:#?}", info);

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
