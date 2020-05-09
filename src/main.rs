#![allow(overflowing_literals)]

// import!(
//     dependencies
//         "os"
//     modules
//         "windows.foundation.collections"
//         "windows.web.syndication"
// );

include!(r"c:\git\rust\dump.rs");

#[link(name = "kernel32")]
extern "system" {
    fn CreateEventW(security: RawPtr, manual: i32, initial: i32, name: RawPtr) -> RawPtr;
    fn SetEvent(handle: RawPtr) -> i32;
    fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    fn CloseHandle(handle: RawPtr) -> i32;
}

const INFINITE: u32 = 0xFFFFFFFF;

use std::sync::*;
use windows::foundation::*;
use windows::foundation::collections::*;
use windows::web::syndication::*;
use winrt::*;

fn main() -> Result<()> {
    use windows::foundation::*;
    use windows::foundation::collections::*;

    let set = &PropertySet::new()?;
    let mut invoked = false;

    // TODO: Should be able to elide the delegate construction and simply say:
    // set.map_changed(|sender, args| {...})?;
    set.map_changed(MapChangedEventHandler::<winrt::HString, winrt::Object>::new(|sender, args| {
        invoked = true;
        let map: IObservableMap<winrt::HString, winrt::Object> = set.into();
        assert!(map.as_raw() == sender.as_raw());
        assert!(args.key()? == "A");
        assert!(args.collection_change()? == CollectionChange::ItemInserted);
        Ok(())
    }))?;

    set.insert("A", PropertyValue::create_uint32(1)?)?;

    assert!(invoked);


    let uri = Uri::create_uri("http://kennykerr.ca/feed")?;
    let client = SyndicationClient::new()?;
    let operation = client.retrieve_feed_async(uri)?;

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());

        operation.set_completed(AsyncOperationWithProgressCompletedHandler::<
            SyndicationFeed,
            RetrievalProgress,
        >::new(|_operation, _status| {
            SetEvent(event);
            Ok(())
        }))?;

        WaitForSingleObject(event, INFINITE);
        CloseHandle(event);
    }

    let feed = operation.get_results()?;

    for item in feed.items()? {
        println!("{}", item.title()?.text()?);
    }

    Ok(())
}

// fn test_stringable(s: &IStringable) -> Result<()> {
//     println!("test_stringable: {}", s.to_string()?);

//     Ok(())
// }

// fn test_uri() -> Result<()> {
//     let uri = &Uri::create_uri("http://kennykerr.ca")?;
//     let s: IStringable = uri.into();

//     let abi = s.abi();

//     test_stringable(RuntimeType::from_abi(&abi))?;

//     Ok(())
// }

// // TODO: generate this for every delegate
// #[repr(C)]
// struct impl_AsyncActionCompletedHandler<F: FnMut(&IAsyncAction, AsyncStatus) -> winrt::Result<()>> {
//     vtable: *const abi_AsyncActionCompletedHandler,
//     count: RefCount,
//     invoke: F,
// }

// impl<F: FnMut(&IAsyncAction, AsyncStatus) -> winrt::Result<()>>
//     impl_AsyncActionCompletedHandler<F>
// {
//     const VTABLE: abi_AsyncActionCompletedHandler =
//         abi_AsyncActionCompletedHandler {
//             unknown_query_interface: impl_AsyncActionCompletedHandler::<F>::unknown_query_interface,
//             unknown_add_ref: impl_AsyncActionCompletedHandler::<F>::unknown_add_ref,
//             unknown_release: impl_AsyncActionCompletedHandler::<F>::unknown_release,
//             invoke: impl_AsyncActionCompletedHandler::<F>::invoke,
//         };

//     fn make(invoke: F) -> AsyncActionCompletedHandler {
//         let delegate = Self {
//             vtable: &Self::VTABLE,
//             count: RefCount::new(1),
//             invoke,
//         };

//         let mut result = AsyncActionCompletedHandler::default();
//         unsafe {
//             *result.set_abi() =
//                 Box::into_raw(Box::new(delegate)) as *const *const abi_AsyncActionCompletedHandler;
//         }
//         result
//     }

//     extern "system" fn unknown_query_interface(
//         this: ::winrt::RawComPtr<::winrt::IUnknown>,
//         iid: &::winrt::Guid,
//         interface: *mut ::winrt::RawPtr,
//     ) -> ErrorCode {
//         unsafe {
//             let this = this as *const Self as *mut Self;

//             if *iid == <AsyncActionCompletedHandler as ::winrt::ComInterface>::IID
//                 || *iid == <::winrt::IUnknown as ::winrt::ComInterface>::IID
//                 || *iid == <::winrt::IAgileObject as ::winrt::ComInterface>::IID
//             {
//                 *interface = this as ::winrt::RawPtr;
//                 (*this).count.add_ref();
//                 return ErrorCode(0);
//             }

//             *interface = std::ptr::null_mut();
//             ErrorCode(0x80004002)
//         }
//     }

//     extern "system" fn unknown_add_ref(this: ::winrt::RawComPtr<::winrt::IUnknown>) -> u32 {
//         unsafe {
//             let this = this as *const Self as *mut Self;
//             (*this).count.add_ref()
//         }
//     }

//     extern "system" fn unknown_release(this: ::winrt::RawComPtr<::winrt::IUnknown>) -> u32 {
//         unsafe {
//             let this = this as *const Self as *mut Self;
//             let remaining = (*this).count.release();

//             if remaining == 0 {
//                 Box::from_raw(this);
//             }

//             remaining
//         }
//     }

//     extern "system" fn invoke(
//         this: *const *const abi_AsyncActionCompletedHandler,
//         sender: <r#IAsyncAction as ::winrt::RuntimeType>::Abi,
//         args: <r#AsyncStatus as ::winrt::RuntimeType>::Abi,
//     ) -> ::winrt::ErrorCode {
//         unsafe {
//             let this = this as *const Self as *mut Self;
//             ((*this).invoke)(&IAsyncAction::default(), AsyncStatus::Completed).into()
//         }
//     }
// }
