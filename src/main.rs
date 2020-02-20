// use winmd::*;

// fn main() {
//     let mut w = RustWriter::new();
//     w.add_namespace("windows.foundation.collections");
//     w.write();
// }

// // fn main() {
// //     let r = Reader::from_os().unwrap();
// //     let class = r.resolve("Windows.AI.MachineLearning.Preview.LearningModelBindingPreview");

// //     for attribute in class.attributes(&r) {
// //         let (_, name) = attribute.name(&r);
// //         println!("{}", name);

// //         if name == "ActivatableAttribute" {
// //             let args = attribute.arguments(&r);
// //         }
// //     }
// // }

winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.composition"
        "windows.web.syndication"
        "windows.foundation.collections"
        "windows.foundation.numerics"
        "windows.graphics.capture"
        "windows.ui.xaml"
);

use windows::foundation::collections::*;
use windows::foundation::*;
use windows::graphics::capture::*;
use winrt::*;

fn main() -> winrt::Result<()> {
    println!("supported: {}", GraphicsCaptureSession::is_supported()?);

    {
        let a = GuidHelper::create_new_guid()?;
        let b = GuidHelper::create_new_guid()?;
        // TODO: the ABI for these aren't projecting correctly - not sure why they don't AV
        assert!(!GuidHelper::equals(a, b)?);
        assert!(GuidHelper::equals(a, a)?);
    }
    {
        assert!(AsyncStatus::default() == AsyncStatus::Canceled);
    }

    if false {
        use windows::foundation::numerics::*;
        use windows::ui::composition::*;
        use windows::ui::*;

        let compositor = Compositor::new()?;
        let visual = compositor.create_sprite_visual()?;

        let brush = compositor.create_color_brush_with_color(Colors::red()?)?;
        visual.set_brush(brush)?;

        visual.set_offset(Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })?;
        assert!(
            visual.offset()?
                == Vector3 {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0
                }
        );
    }

    {
        let uri = &Uri::create_uri("http://kennykerr.ca")?;
        let _ob: Object = uri.into();

        assert!(!uri.is_empty());
        let decoder = uri.query_parsed()?;
        assert!(!decoder.is_empty());

        // TODO: need generic guids!
        let v: IVectorView<IWwwFormUrlDecoderEntry> = decoder.into();
        assert!(v.is_empty());
    }

    {
        let uri = &Uri::create_uri("http://kennykerr.ca")?;
        println!("domain: {}", uri.domain()?);

        let d: IUriRuntimeClass = uri.into();
        println!("domain: {}", d.domain()?);
        println!("port: {}", d.port()?);

        let s: IStringable = uri.into();
        let value = s.to_string()?;
        println!("stringable: {}", value);

        println!("QI stringable: {}", uri.to_string()?);
    }

    {
        let object = PropertyValue::create_string("hello")?;
        let pv: IPropertyValue = object.query();
        let value = pv.get_string()?;
        println!("pv {}", value);
    }

    {
        let object = PropertyValue::create_u_int32_array(&[1, 2, 3])?;
        let pv: IPropertyValue = object.query();
        let mut array = Array::new();
        pv.get_u_int32_array(&mut array)?;

        for i in array.as_slice() {
            println!("a: {}", i);
        }
    }

    Ok(())
}
