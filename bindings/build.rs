winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::application_model::data_transfer::*
);

fn main() {
    build();
}
