//use winrt::*;

//use winmd::*;
// fn main() {

//     let mut writer = RustWriter::new();
//     //writer.add_namespace("Windows.UI");
//     writer.add_namespace("Windows.Foundation.Collections");
//     let output = writer.write();
// }

winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui"
        "windows.foundation.collections"
        //"windows.data.json"
);

//use windows::foundation::collections::*;
use windows::foundation::*;
use winrt::QueryType;

fn main() -> winrt::Result<()> {
    let uri = &Uri::create_uri("http://kennykerr.ca")?;
    println!("domain: {}", uri.domain()?);

    let d: IUriRuntimeClass = uri.into();
    println!("domain: {}", d.domain()?);
    println!("port: {}", d.port()?);

    let s: IStringable = uri.into();
    let value = s.to_string()?;
    println!("stringable: {}", value);

    println!("domain: {}", uri.domain()?);

    let _v = uri.query_parsed()?;

    call(uri)?;
    call(uri.clone())?;
    //call(d);
    call(&s)?;
    call(Uri::create_uri("http://kennykerr.ca")?)?;

    let o: winrt::Object = s.query();
    let s: IStringable = o.query();
    println!("o: {}", s.to_string()?);

    Ok(())
}

fn call<'a, T: Into<winrt::Param<'a, IStringable>>>(_s: T) -> winrt::Result<()> {
    Ok(())
}
