use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

use windows::foundation::*;

fn call<'a, T: Into<Param<'a, IStringable>>>(s: T) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let uri = &Uri::create_uri("http://kennykerr.ca")?;
    println!("domain: {}", uri.domain()?);

    let d: IUriRuntimeClass = uri.into();
    println!("domain: {}", d.domain()?);
    println!("port: {}", d.port()?);

    let s: IStringable = uri.into();
    let value = s.to_string()?;
    println!("stringable: {}", value);

    println!("domain: {}", uri.domain()?);

    call(uri);
    //call(d);
    call(&s);
    call(Uri::create_uri("http://kennykerr.ca")?);

    Ok(())
}
