use winrt::*;

import!(
    dependencies
    "os"
modules
    "windows.ui"
    //"windows.foundation"
);

fn main() -> Result<()> {
    use windows::foundation::*;

    let uri = &Uri::create_uri("http://kennykerr.ca")?;
    println!("domain: {}", uri.domain()?);

    let d: IUriRuntimeClass = uri.into();
    println!("domain: {}", d.domain()?);
    
    let s: IStringable = uri.into();
    let s = s.to_string()?;
    println!("stringable: {}", s);

    println!("domain: {}", uri.domain()?);

    Ok(())
}
