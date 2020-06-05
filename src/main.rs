use bindings::windows::application_model::data_transfer::{Clipboard, DataPackage};
use bindings::windows::data::xml::dom::XmlDocument;
use bindings::windows::foundation::Uri;
use bindings::windows::web::syndication::SyndicationClient;

fn main() -> winrt::Result<()> {
    clipboard()?;
    xml()?;
    feed()?;
    Ok(())
}

fn clipboard() -> winrt::Result<()> {
    let content = DataPackage::new()?;

    Clipboard::set_content(content)?;
    Clipboard::flush()?;

    // "Rust/WinRT" is now on the clipboard

    Ok(())
}

fn xml() -> winrt::Result<()> {
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
    let uri = Uri::create_uri("https://kennykerr.ca/feed")?;

    let client = SyndicationClient::new()?;

    let feed = client.retrieve_feed_async(uri)?.get()?;
    for item in feed.items()?.into_iter().take(3) {
        println!("title: {}", item.title()?.text()?);
    }

    Ok(())
}