use bindings::*;

fn main() -> winrt::Result<()> {
    clipboard()?;
    xml()?;

    Ok(())
}

fn xml() -> winrt::Result<()> {
    use windows::data::xml::dom::XmlDocument;
    let doc = XmlDocument::new()?;

    doc.load_xml(
        "<things>
            <color>red</color>
            <animal>bird</animal>
            <color>blue</color>
        </things>",
    )?;

    let root = doc.document_element()?;
    let colors = root.get_elements_by_tag_name("color")?;

    for color in colors {
        println!("{}", color.inner_text()?);
    }

    Ok(())
}

fn clipboard() -> winrt::Result<()> {
    use windows::application_model::data_transfer::*;

    let content = DataPackage::new()?;
    content.set_text("Rust/WinRT")?;

    Clipboard::set_content(content)?;
    Clipboard::flush()?;

    // "Rust/WinRT" is now on the clipboard

    Ok(())
}
