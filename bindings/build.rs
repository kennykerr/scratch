winrt::build!(
    dependencies
        os
    types
        windows::application_model::data_transfer::*
        windows::ui::composition::*
        windows::data::xml::dom::*
        windows::web::syndication::*
);

fn main() {
    build();
}
