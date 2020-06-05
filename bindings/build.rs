winrt::build!(
    dependencies
        os
    types
        windows::application_model::data_transfer::*
        windows::ui::composition::*
        windows::data::xml::dom::*
);

fn main() {
    build();
}
