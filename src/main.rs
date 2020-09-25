fn main() -> winrt::Result<()> {
    println!("done!");
    Ok(())
}

#[macros::implements(
    windows::foundation::Uri,
    windows::foundation::IStringable,
    windows::foundation::{IClosable, IPropertyValue},
    windows::ui::{
        composition::{SpriteVisual, ISpriteVisual},
        xaml::Application
    }
)]
pub struct Thing {}
