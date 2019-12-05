winrt::import!(
    dependencies
        "../folder/containing/winmd/files"
        "../path/to/specific.winmd"
        "10.0" // alias for latest Win10 SDK
        "10.0.19038.0" // alias for specific Win10 SDK internally fulfilled via NuGet
        "http://some-url-pointing-to-winmd-file"
        "http://some-url-pointing-to-zip-file-containing-winmd-files"
        "https://www.nuget.org/api/v2/package/Win2D.uwp/1.24.0"
        "https://www.nuget.org/api/v2/package/Microsoft.Windows.SDK.Contracts/10.0.18362.2005"
    modules // omit this to get everything
        "kittens"
        "windows.storage"
        "windows.ui.composition"
);

fn main() -> winrt::Result<()> {
    use windows::ui::*;
    let color = Colors::alice_blue()?;

    assert!(color.a == 255);
    assert!(color.r == 240);
    assert!(color.g == 248);
    assert!(color.b == 255);

    println!("woot!");

    Ok(())
}
