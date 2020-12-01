use winrt::*;
use bindings::*;

fn main() -> Result<()> {
    use windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    println!("{}", root.inner_text()?);

    Ok(())
}
