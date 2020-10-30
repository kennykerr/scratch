use bindings::*;
use winrt::*;

fn main() -> Result<()> {
    let s = Thing::into_com(Thing::new("HELLO WORLD!".into()));
    println!("s: {}", s.to_string()?);

    let b: windows::foundation::IStringable = s.try_query()?;
    println!("b: {}", b.to_string()?);

    let c: windows::foundation::IClosable = s.try_query()?;
    c.close()?;

    Ok(())
}

// TODO: implements macro must sniff out "bindings" crate/module name to figure out how to
// preamble the internal names like the abi_IInterface etc. It can do so by first looking
// up the names in the TypeReader and if not found, peel off root namespace.

//#[macros::implements(windows::foundation::IStringable,windows::foundation::IClosable)]
pub struct Thing {
    value: String,
}

impl Drop for Thing {
    fn drop(&mut self) {
        println!("~Thing");
    }
}

impl Thing {
    fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> Result<HString> {
        // Ok(self.value.into()) // TODO: why doesn't this work?

        Ok(self.value.clone().into())
    }

    pub fn close(&mut self) -> Result<()> {
        println!("closing {}", self.value);
        self.value.clear();
        Ok(())
    }
}

// TODO: This should be generated:
impl Thing {
    pub fn into_com(self) -> windows::foundation::IStringable {
        com_Thing::new(self)
    }
}

#[repr(C)]
struct com_Thing {
    vtable: (*const abi_IStringable, *const abi_IClosable),
    inner: Thing,
    count: RefCount,
}

#[allow(non_snake_case)]
impl com_Thing {
    fn new(inner: Thing) -> windows::foundation::IStringable {
        let com = Self {
            vtable: (&Self::VTABLE.0, &Self::VTABLE.1),
            count: ::winrt::RefCount::new(),
            inner,
        };

        // TODO: get rid of the all the wrapping in the projection.
        unsafe {
            let mut result: windows::foundation::IStringable = std::mem::zeroed();
            let ptr: ::std::ptr::NonNull<Self> = ::std::ptr::NonNull::new_unchecked(
                ::std::boxed::Box::into_raw(::std::boxed::Box::new(com)),
            );
            *<windows::foundation::IStringable as ::winrt::AbiTransferable>::set_abi(&mut result) =
                Some(::winrt::NonNullRawComPtr::new(ptr.cast()));
            result
        }
    }

    const VTABLE: (abi_IStringable, abi_IClosable) = (
        abi_IStringable(
            Self::IStringable_QueryInterface,
            Self::IStringable_AddRef,
            Self::IStringable_Release,
            Self::IInspectable_GetIids,
            Self::IInspectable_GetRuntimeClassName,
            Self::IInspectable_GetTrustLevel,
            Self::IStringable_ToString,
        ),
        abi_IClosable(
            Self::IClosable_QueryInterface,
            Self::IClosable_AddRef,
            Self::IClosable_Release,
            Self::IInspectable_GetIids,
            Self::IInspectable_GetRuntimeClassName,
            Self::IInspectable_GetTrustLevel,
            Self::IClosable_Close,
        ),
    );

    //
    // Common instance-specific IUnknown implementation.
    // Interfaces must forward to these after adjusting the self pointer.
    //

    fn IUnknown_QueryInterface(&mut self, iid: &Guid, interface: *mut RawPtr) -> ErrorCode {
        unsafe {
            *interface = match iid {
                &<windows::foundation::IStringable as ComInterface>::IID
                | &<::winrt::IUnknown as ::winrt::ComInterface>::IID
                | &<::winrt::IAgileObject as ::winrt::ComInterface>::IID => {
                    &mut self.vtable.0 as *mut _ as _
                }
                &<windows::foundation::IClosable as ComInterface>::IID => {
                    &mut self.vtable.1 as *mut _ as _
                }
                _ => std::ptr::null_mut(),
            };

            if (*interface).is_null() {
                winrt::ErrorCode::E_NOINTERFACE
            } else {
                self.count.add_ref();
                winrt::ErrorCode::S_OK
            }
        }
    }

    fn IUnknown_AddRef(&mut self) -> u32 {
        self.count.add_ref()
    }

    fn IUnknown_Release(&mut self) -> u32 {
        let remaining = self.count.release();

        if remaining == 0 {
            unsafe { Box::from_raw(self); }
        }

        remaining
    }

    //
    // Common type-specific IInspectable implementation.
    // Interface vtables can share these directly.
    //

    extern "system" fn IInspectable_GetIids(
        _: RawPtr,
        count: *mut u32,
        values: *mut *mut Guid,
    ) -> ErrorCode {
        // Note: even if we end up implementing this in future, it still doesn't need a this pointer
        // since the data to be returned is type- not instance-specific so can be shared for all
        // interfaces.
        unsafe {
            *count = 0;
            *values = std::ptr::null_mut();
        }
        ErrorCode(0)
    }

    extern "system" fn IInspectable_GetRuntimeClassName(
        _: RawPtr,
        _value: *mut RawPtr,
    ) -> ErrorCode {
        panic!("IInspectable_GetRuntimeClassName"); // TODO: class name or first interface (from implements macro)
    }

    extern "system" fn IInspectable_GetTrustLevel(_: RawPtr, value: *mut i32) -> ErrorCode {
        // Note: even if we end up implementing this in future, it still doesn't need a this pointer
        // since the data to be returned is type- not instance-specific so can be shared for all
        // interfaces.
        unsafe {
            *value = 0;
        }
        ErrorCode(0)
    }

    //
    // IStringable virtual functions.
    //

    extern "system" fn IStringable_QueryInterface(
        this: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        unsafe {
            let this = (this as *mut RawPtr).offset(0) as *mut Self;
            (*this).IUnknown_QueryInterface(iid, interface)
        }
    }

    extern "system" fn IStringable_AddRef(this: RawPtr) -> u32 {
        unsafe {
            let this = (this as *mut RawPtr).offset(0) as *mut Self;
            (*this).IUnknown_AddRef()
        }
    }

    extern "system" fn IStringable_Release(this: RawPtr) -> u32 {
        unsafe {
            let this = (this as *mut RawPtr).offset(0) as *mut Self;
            (*this).IUnknown_Release()
        }
    }

    extern "system" fn IStringable_ToString(
        this: RawPtr,
        value: *mut <::winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe {
            let this = (this as *mut RawPtr).offset(0) as *mut Self;

            // TODO: winrt::Result needs to be more than just a type alias so we can clean this up with a simple into_abi method.
            match (*this).inner.to_string() {
                ::std::result::Result::Ok(ok__) => {
                    *value = <HString as AbiTransferable>::into_abi(ok__);
                    ErrorCode(0)
                }
                ::std::result::Result::Err(err) => err.into(),
            }
        }
    }

    //
    // IClosable virtual functions.
    //

    extern "system" fn IClosable_QueryInterface(
        this: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        unsafe {
            let this = (this as *mut RawPtr).offset(-1) as *mut Self;
            (*this).IUnknown_QueryInterface(iid, interface)
        }
    }

    extern "system" fn IClosable_AddRef(this: RawPtr) -> u32 {
        unsafe {
            let this = (this as *mut RawPtr).offset(-1) as *mut Self;
            (*this).IUnknown_AddRef()
        }
    }

    extern "system" fn IClosable_Release(this: RawPtr) -> u32 {
        unsafe {
            let this = (this as *mut RawPtr).offset(-1) as *mut Self;
            (*this).IUnknown_Release()
        }
    }

    extern "system" fn IClosable_Close(this: RawPtr) -> ErrorCode {
        unsafe {
            let this = (this as *mut RawPtr).offset(-1) as *mut Self;
            (*this).inner.close().into()
        }
    }
}
#[allow(non_camel_case_types)]
type IUnknown_QueryInterface =
    extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode;

    #[allow(non_camel_case_types)]
    type IUnknown_AddRef = extern "system" fn(this: RawPtr) -> u32;

    #[allow(non_camel_case_types)]
    type IUnknown_Release = extern "system" fn(this: RawPtr) -> u32;

    #[allow(non_camel_case_types)]
    type IInspectable_GetIids =
    extern "system" fn(this: RawPtr, count: *mut u32, values: *mut *mut Guid) -> ErrorCode;
    #[allow(non_camel_case_types)]
    type IInspectable_GetRuntimeClassName =
    extern "system" fn(this: RawPtr, value: *mut RawPtr) -> ErrorCode;
    #[allow(non_camel_case_types)]
    type IInspectable_GetTrustLevel = extern "system" fn(this: RawPtr, value: *mut i32) -> ErrorCode;

#[repr(C)]
pub struct abi_IStringable(
    IUnknown_QueryInterface,
    IUnknown_AddRef,
    IUnknown_Release,
    IInspectable_GetIids,
    IInspectable_GetRuntimeClassName,
    IInspectable_GetTrustLevel,
    extern "system" fn(
        this: RawPtr,
        value: *mut <::winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> ErrorCode,
);

#[repr(C)]
pub struct abi_IClosable(
    IUnknown_QueryInterface,
    IUnknown_AddRef,
    IUnknown_Release,
    IInspectable_GetIids,
    IInspectable_GetRuntimeClassName,
    IInspectable_GetTrustLevel,
    extern "system" fn(this: RawPtr) -> ErrorCode,
);
