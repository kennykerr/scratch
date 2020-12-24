use winrt::*;
use bindings::*;

fn main() -> Result<()> {
    use windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    println!("{}", root.inner_text()?);

    let reader = winmd::TypeReader::get();

    let apis = reader.expect_type_def(("Windows.Foundation", "IStringable"));

    let extends = apis.extends();

    println!("{:?}", extends.name());

    Ok(())
}
