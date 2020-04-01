use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::*;
use std::iter::FromIterator;
use winmd::*;

fn main() {
    let mut reader = &Reader::from_os();

    let def = reader.resolve(("Windows.Foundation", "Point"));
    let info = def.info(reader);
    println!("{:#?}", info);
    let stream = info.to_stream();
    println!("{}", stream.to_string());

    // let mut limits: TypeLimits = Default::default();
    // limits.insert(reader, "windows.foundation");

    // let stage = TypeStage::from_limits(reader, &limits);

    // println!("count: {}", stage.0.len());

    // let tree = stage.to_tree();

    // println!("tree");

    // let stream = tree.to_stream();

    // println!("stream");

    // println!("{}", stream.to_string());
}
