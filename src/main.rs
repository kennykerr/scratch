use bindings::*;

fn main() -> winrt::Result<()> {
    clipboard()?;
    xml()?;
    feed()?;

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
        println!("color: {}", color.inner_text()?);
    }

    Ok(())
}

fn feed() -> winrt::Result<()> {
    use windows::foundation::Uri;
    use windows::web::syndication::*;

    let uri = Uri::create_uri("https://kennykerr.ca/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.retrieve_feed_async(uri)?.get()?;

    for item in feed.items()?.into_iter().take(3) {
        println!("title: {}", item.title()?.text()?);
    }

    Ok(())
}
