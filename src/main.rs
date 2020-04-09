//use proc_macro2::{Ident, Literal, TokenStream};
//use quote::{format_ident, quote};
//use std::collections::*;
//use std::iter::FromIterator;
//use winmd::*;

use winmd::*;
use winrt::*;

// import!(
//     dependencies
//         "os"
//     modules
//         "windows.foundation"
// );

//use windows::foundation::*;

fn main() {
  //  let uri = Uri::default();

     let winmd_files = load_winmd::from_os();
     let reader = &TypeReader::new(winmd_files);
     reader.resolve(("Windows.Foundation", "WwwFormUrlDecoder")).into_type(reader);


    // let mut limits = TypeLimits::default();
    // limits.insert(reader, "windows.foundation");

    // let stage = TypeStage::from_limits(reader, &limits);

    // println!("count: {}", stage.0.len());

    // let tree = stage.into_tree();

    // println!("tree");

    // let stream = tree.to_stream();

    // println!("stream");

    // println!("{}", stream.to_string());
}
