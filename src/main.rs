use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::*;
use std::iter::FromIterator;
use winmd::*;

fn main() {
    let reader = &Reader::from_os();

    let def = reader.resolve(("Windows.Foundation", "IStringable"));
    def.attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute")).arguments(reader);

    let mut limits: TypeLimits = Default::default();
    limits.insert(reader, "windows.foundation");

    let stage = TypeStage::from_limits(&limits, reader);

    println!("count: {}", stage.0.len());

    let tree = stage.into_tree();

    println!("tree");

    let stream = tree.to_stream();

    println!("stream");

    println!("{}", stream.to_string());
}
