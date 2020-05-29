winrt::import!(
    dependencies
        os
        nuget: Win2D.uwp
    types
        microsoft::graphics::canvas::text::CanvasTypography
);

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() -> winrt::Result<()> {
    microsoft::graphics::canvas::text::CanvasTypography::new()?;

    Ok(())
}
