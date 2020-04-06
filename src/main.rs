//use proc_macro2::{Ident, Literal, TokenStream};
//use quote::{format_ident, quote};
//use std::collections::*;
//use std::iter::FromIterator;
use winmd::*;

fn main() {
    let winmd_files = load_winmd::from_os();
    let reader = &TypeReader::new(winmd_files);

    let def = reader.resolve(("Windows.Foundation", "WwwFormUrlDecoder"));

    let t = def.into_type(reader);

    println!("{:#?}", t);

    // let mut limits: TypeLimits = Default::default();
    // limits.insert(reader, "windows.foundation");

    // let stage = TypeStage::from_limits(reader, &limits);

    // println!("count: {}", stage.0.len());

    // let tree = stage.into_tree();

    // println!("tree");

    // let stream = tree.to_stream();

    // println!("stream");

    // println!("{}", stream.to_string());
}
