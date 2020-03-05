use windows::foundation::*;
use winrt::*;

import!(
    dependencies:
        "os"
    modules:
        "windows.foundation.collections"
);

// TODO:
//  The class macro adds a pointer to a static vtbl for each interface.
//  Implements the From trait to allow conversion to wrapped COM object
// from any Stringable by moving it into the COM object.
#[class(
    implements(IStringable, IClosable),
    type_name("Windows.Foundation.Stringable")
)]
struct Stringable {
    value: String,
}

impl traits::IStringable for Stringable {
    fn to_string(&self) -> Result<HString> {
        Ok((&self.value).into())
    }
}

fn main() -> Result<()> {
    let uri = Uri::create_uri("https://kennykerr.ca")?;
    println!("{}", uri.domain()?);
    println!("{}", uri.to_string()?);

    let s: IStringable = uri.into();
    println!("{}", s.to_string()?);

    Ok(())
}

// use winrt::*;

// fn call<'a, T:Into<Param<'a, Guid>>>(value:T) {

// }

// fn main(){
//     let g : Guid = Default::default();
//     call(g);
// }
