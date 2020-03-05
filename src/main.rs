winrt::import!(
    dependencies:
        "os"
    modules:
        "windows.foundation.collections"
);

use winrt::*;
use windows::foundation::*;

fn main() -> Result<()> {
    let uri = Uri::create_uri("https://kennykerr.ca")?;
    println!("{}", uri.domain()?);
    println!("{}", uri.to_string()?);

    Ok(())
}

// use winrt::*;

// fn call<'a, T:Into<Param<'a, Guid>>>(value:T) {

// }

// fn main(){
//     let g : Guid = Default::default();
//     call(g);
// }
