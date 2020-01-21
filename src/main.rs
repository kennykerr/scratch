use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

use windows::foundation::*;

struct A{}
struct B{}

impl AsRef<B> for B {
    fn as_ref(&self) -> &B {
        self
    }
}

impl AsRef<B> for A {
    fn as_ref(&self) -> &B {
        &B{}
    }
}

fn call<T: AsRef<B>>(value: T) {
    println!("call");
}

// fn call<T:AsRef<IStringable>>(s: T) -> Result<()> {
//     println!("call {}", s.as_ref().to_string()?);

//     Ok(())
// }

fn main() -> Result<()> {

    call(B{});
    let b = B{};
    call(&b);
    call(A{});
    let a = A{};
    call(&a);

    // let uri = &Uri::create_uri("http://kennykerr.ca")?;
    // println!("domain: {}", uri.domain()?);

    // let d: IUriRuntimeClass = uri.into();
    // println!("domain: {}", d.domain()?);
    // println!("port: {}", d.port()?);
    
    // let s: IStringable = uri.into();
    // let value = s.to_string()?;
    // println!("stringable: {}", value);

    // println!("domain: {}", uri.domain()?);

    // call(&s);
    // call(s);

    Ok(())
}
