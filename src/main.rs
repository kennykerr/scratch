use windows::foundation::*;
use windows::foundation::collections::*;
use winrt::*;

use windows::foundation::traits as wf_traits;

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

impl wf_traits::IStringable for Stringable {
    fn to_string(&self) -> Result<HString> {
        Ok((&self.value).into())
    }
}

// impl TypeGuid for IVectorView<IWwwFormUrlDecoderEntry> {
//     fn type_guid() -> &'static Guid {
//         static GUID: Guid = Guid::from_values(
//             0x00000035,
//             0x0000,
//             0x0000,
//             &[0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
//         );
//         &GUID
//     }
// }

fn main() -> Result<()> {
    let uri = &Uri::create_uri("https://kennykerr.ca")?;
    println!("{}", uri.domain()?);
    println!("{}", uri.to_string()?);

    let s: IStringable = uri.into();
    println!("{}", s.to_string()?);

    let decoder = uri.query_parsed()?;

    //let size = decoder.size();

    

    Ok(())
}

// use winrt::*;

// fn call<'a, T:Into<Param<'a, Guid>>>(value:T) {

// }

// fn main(){
//     let g : Guid = Default::default();
//     call(g);
// }
