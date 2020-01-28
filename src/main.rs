// use winmd::*;

// fn main() {
//     let mut writer = RustWriter::new();
//     writer.add_namespace("windows.foundation");
//     let output = writer.write();
// }

// winrt::import!(
//     dependencies
//         "os"
//     modules
//         //"windows.ui"
//         "windows.foundation"
//         //"windows.data.json"
// );

pub mod windows {
    pub mod foundation {
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncActionCompletedHandler {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_AsyncActionCompletedHandler {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, AsyncStatus) -> winrt::ErrorCode,
        }
        impl AsyncActionCompletedHandler {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncAction,
                async_status: AsyncStatus,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_AsyncActionCompletedHandler)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        async_status,
                    )
                    .ok()
                }
            }
        }
        impl winrt::DelegateType for AsyncActionCompletedHandler {}
        impl winrt::QueryType for AsyncActionCompletedHandler {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2767019137,
                    30409,
                    16573,
                    &[139, 230, 177, 217, 15, 178, 10, 231],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for AsyncActionCompletedHandler {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, AsyncActionCompletedHandler>> for AsyncActionCompletedHandler {
            fn into(self) -> winrt::Param<'a, AsyncActionCompletedHandler> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, AsyncActionCompletedHandler>> for &'a AsyncActionCompletedHandler {
            fn into(self) -> winrt::Param<'a, AsyncActionCompletedHandler> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncActionProgressHandler<TProgress: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_AsyncActionProgressHandler<TProgress: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                <TProgress as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TProgress>,
        }
        impl<TProgress: winrt::RuntimeType> AsyncActionProgressHandler<TProgress> {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncActionWithProgress<TProgress>,
                progress_info: &TProgress,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_AsyncActionProgressHandler<TProgress>)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        winrt::RuntimeType::abi(progress_info),
                    )
                    .ok()
                }
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::DelegateType for AsyncActionProgressHandler<TProgress> {}
        impl<TProgress: winrt::RuntimeType> winrt::QueryType for AsyncActionProgressHandler<TProgress> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1837385816,
                    3327,
                    17808,
                    &[174, 137, 149, 165, 165, 200, 180, 184],
                );
                &GUID
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::RuntimeType for AsyncActionProgressHandler<TProgress> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncActionProgressHandler<TProgress>>>
            for AsyncActionProgressHandler<TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncActionProgressHandler<TProgress>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncActionProgressHandler<TProgress>>>
            for &'a AsyncActionProgressHandler<TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncActionProgressHandler<TProgress>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncActionWithProgressCompletedHandler<TProgress: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_AsyncActionWithProgressCompletedHandler<TProgress: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, AsyncStatus) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TProgress>,
        }
        impl<TProgress: winrt::RuntimeType> AsyncActionWithProgressCompletedHandler<TProgress> {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncActionWithProgress<TProgress>,
                async_status: AsyncStatus,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_AsyncActionWithProgressCompletedHandler<TProgress>)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        async_status,
                    )
                    .ok()
                }
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::DelegateType
            for AsyncActionWithProgressCompletedHandler<TProgress>
        {
        }
        impl<TProgress: winrt::RuntimeType> winrt::QueryType
            for AsyncActionWithProgressCompletedHandler<TProgress>
        {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2617417617,
                    52356,
                    17661,
                    &[172, 38, 10, 108, 78, 85, 82, 129],
                );
                &GUID
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::RuntimeType
            for AsyncActionWithProgressCompletedHandler<TProgress>
        {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncActionWithProgressCompletedHandler<TProgress>>>
            for AsyncActionWithProgressCompletedHandler<TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncActionWithProgressCompletedHandler<TProgress>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncActionWithProgressCompletedHandler<TProgress>>>
            for &'a AsyncActionWithProgressCompletedHandler<TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncActionWithProgressCompletedHandler<TProgress>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncOperationCompletedHandler<TResult: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TResult>,
        }
        #[repr(C)]
        struct abi_AsyncOperationCompletedHandler<TResult: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, AsyncStatus) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TResult>,
        }
        impl<TResult: winrt::RuntimeType> AsyncOperationCompletedHandler<TResult> {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncOperation<TResult>,
                async_status: AsyncStatus,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_AsyncOperationCompletedHandler<TResult>)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        async_status,
                    )
                    .ok()
                }
            }
        }
        impl<TResult: winrt::RuntimeType> winrt::DelegateType for AsyncOperationCompletedHandler<TResult> {}
        impl<TResult: winrt::RuntimeType> winrt::QueryType for AsyncOperationCompletedHandler<TResult> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    4242337836,
                    58840,
                    17528,
                    &[145, 90, 77, 144, 183, 75, 131, 165],
                );
                &GUID
            }
        }
        impl<TResult: winrt::RuntimeType> winrt::RuntimeType for AsyncOperationCompletedHandler<TResult> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TResult: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationCompletedHandler<TResult>>>
            for AsyncOperationCompletedHandler<TResult>
        {
            fn into(self) -> winrt::Param<'a, AsyncOperationCompletedHandler<TResult>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TResult: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationCompletedHandler<TResult>>>
            for &'a AsyncOperationCompletedHandler<TResult>
        {
            fn into(self) -> winrt::Param<'a, AsyncOperationCompletedHandler<TResult>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncOperationProgressHandler<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_AsyncOperationProgressHandler<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                <TProgress as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            AsyncOperationProgressHandler<TResult, TProgress>
        {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncOperationWithProgress<TResult, TProgress>,
                progress_info: &TProgress,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_AsyncOperationProgressHandler<TResult, TProgress>)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        winrt::RuntimeType::abi(progress_info),
                    )
                    .ok()
                }
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::DelegateType
            for AsyncOperationProgressHandler<TResult, TProgress>
        {
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::QueryType
            for AsyncOperationProgressHandler<TResult, TProgress>
        {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1432946946,
                    2731,
                    16922,
                    &[135, 120, 248, 206, 80, 38, 215, 88],
                );
                &GUID
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::RuntimeType
            for AsyncOperationProgressHandler<TResult, TProgress>
        {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationProgressHandler<TResult, TProgress>>>
            for AsyncOperationProgressHandler<TResult, TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncOperationProgressHandler<TResult, TProgress>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationProgressHandler<TResult, TProgress>>>
            for &'a AsyncOperationProgressHandler<TResult, TProgress>
        {
            fn into(self) -> winrt::Param<'a, AsyncOperationProgressHandler<TResult, TProgress>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct AsyncOperationWithProgressCompletedHandler<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_AsyncOperationWithProgressCompletedHandler<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, AsyncStatus) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            pub fn r#invoke(
                &self,
                async_info: &IAsyncOperationWithProgress<TResult, TProgress>,
                async_status: AsyncStatus,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_AsyncOperationWithProgressCompletedHandler<
                            TResult,
                            TProgress,
                        >)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(async_info),
                        async_status,
                    )
                    .ok()
                }
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::DelegateType
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::QueryType
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3898471453,
                    27303,
                    18147,
                    &[168, 226, 240, 9, 216, 64, 198, 39],
                );
                &GUID
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::RuntimeType
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>>
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            fn into(
                self,
            ) -> winrt::Param<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>
            {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>>
            for &'a AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            fn into(
                self,
            ) -> winrt::Param<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>
            {
                winrt::Param::Ref(self)
            }
        }
        pub enum AsyncStatus {
            Canceled,
            Completed,
            Error,
            Started,
        }
        impl Default for AsyncStatus {
            fn default() -> Self {
                Self::Canceled
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct DateTime {
            pub r#universal_time: u8,
        }
        impl winrt::RuntimeCopy for DateTime {}
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct Deferral {
            ptr: winrt::ComPtr,
        }
        impl Deferral {
            pub fn r#create(handler: &DeferralCompletedHandler) -> winrt::Result<Deferral> {
                winrt::factory::<Deferral, IDeferralFactory>()?.r#create(handler)
            }
            pub fn r#complete(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IDeferral))).r#complete)(
                        self.ptr.get(),
                    )
                    .ok()
                }
            }
        }
        impl winrt::ClassType for Deferral {}
        impl winrt::QueryType for Deferral {
            fn type_guid() -> &'static winrt::Guid {
                <IDeferral as winrt::QueryType>::type_guid()
            }
        }
        impl winrt::TypeName for Deferral {
            fn type_name() -> &'static str {
                "Windows.Foundation.Deferral"
            }
        }
        impl winrt::RuntimeType for Deferral {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, Deferral>> for Deferral {
            fn into(self) -> winrt::Param<'a, Deferral> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, Deferral>> for &'a Deferral {
            fn into(self) -> winrt::Param<'a, Deferral> {
                winrt::Param::Ref(self)
            }
        }
        impl From<Deferral> for IDeferral {
            fn from(value: Deferral) -> IDeferral {
                unsafe { std::mem::transmute(value) }
            }
        }
        impl From<&Deferral> for IDeferral {
            fn from(value: &Deferral) -> IDeferral {
                unsafe { std::mem::transmute(value.clone()) }
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferral>> for Deferral {
            fn into(self) -> winrt::Param<'a, IDeferral> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferral>> for &'a Deferral {
            fn into(self) -> winrt::Param<'a, IDeferral> {
                winrt::Param::Value(self.into())
            }
        }
        impl From<Deferral> for IClosable {
            fn from(value: Deferral) -> IClosable {
                IClosable::from(&value)
            }
        }
        impl From<&Deferral> for IClosable {
            fn from(value: &Deferral) -> IClosable {
                winrt::QueryType::query(value)
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for Deferral {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for &'a Deferral {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Value(self.into())
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct DeferralCompletedHandler {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_DeferralCompletedHandler {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl DeferralCompletedHandler {
            pub fn r#invoke(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_DeferralCompletedHandler))).r#invoke)(
                        self.ptr.get(),
                    )
                    .ok()
                }
            }
        }
        impl winrt::DelegateType for DeferralCompletedHandler {}
        impl winrt::QueryType for DeferralCompletedHandler {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3979518834,
                    62408,
                    20394,
                    &[156, 251, 71, 1, 72, 218, 56, 136],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for DeferralCompletedHandler {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, DeferralCompletedHandler>> for DeferralCompletedHandler {
            fn into(self) -> winrt::Param<'a, DeferralCompletedHandler> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, DeferralCompletedHandler>> for &'a DeferralCompletedHandler {
            fn into(self) -> winrt::Param<'a, DeferralCompletedHandler> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct EventHandler<T: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<T>,
        }
        #[repr(C)]
        struct abi_EventHandler<T: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                <T as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<T>,
        }
        impl<T: winrt::RuntimeType> EventHandler<T> {
            pub fn r#invoke(&self, sender: &winrt::Object, args: &T) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_EventHandler<T>))).r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(sender),
                        winrt::RuntimeType::abi(args),
                    )
                    .ok()
                }
            }
        }
        impl<T: winrt::RuntimeType> winrt::DelegateType for EventHandler<T> {}
        impl<T: winrt::RuntimeType> winrt::QueryType for EventHandler<T> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2648818997,
                    27361,
                    4576,
                    &[132, 225, 24, 169, 5, 188, 197, 63],
                );
                &GUID
            }
        }
        impl<T: winrt::RuntimeType> winrt::RuntimeType for EventHandler<T> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, T: winrt::RuntimeType> Into<winrt::Param<'a, EventHandler<T>>> for EventHandler<T> {
            fn into(self) -> winrt::Param<'a, EventHandler<T>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, T: winrt::RuntimeType> Into<winrt::Param<'a, EventHandler<T>>> for &'a EventHandler<T> {
            fn into(self) -> winrt::Param<'a, EventHandler<T>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct EventRegistrationToken {
            pub r#value: u8,
        }
        impl winrt::RuntimeCopy for EventRegistrationToken {}
        pub struct GuidHelper {}
        impl GuidHelper {
            pub fn r#create_new_guid() -> winrt::Result<winrt::Guid> {
                winrt::factory::<GuidHelper, IGuidHelperStatics>()?.r#create_new_guid()
            }
            pub fn r#empty() -> winrt::Result<winrt::Guid> {
                winrt::factory::<GuidHelper, IGuidHelperStatics>()?.r#empty()
            }
            pub fn r#equals(target: &winrt::Guid, value: &winrt::Guid) -> winrt::Result<bool> {
                winrt::factory::<GuidHelper, IGuidHelperStatics>()?.r#equals(target, value)
            }
        }
        impl winrt::TypeName for GuidHelper {
            fn type_name() -> &'static str {
                "Windows.Foundation.GuidHelper"
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct HResult {
            pub r#value: u8,
        }
        impl winrt::RuntimeCopy for HResult {}
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IAsyncAction {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IAsyncAction {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#set_completed: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#completed: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#get_results: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IAsyncAction {
            pub fn r#set_completed(
                &self,
                handler: &AsyncActionCompletedHandler,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncAction))).r#set_completed)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(handler),
                    )
                    .ok()
                }
            }
            pub fn r#completed(&self) -> winrt::Result<AsyncActionCompletedHandler> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncAction))).r#completed)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_results(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncAction))).r#get_results)(
                        self.ptr.get(),
                    )
                    .ok()
                }
            }
        }
        impl winrt::InterfaceType for IAsyncAction {}
        impl winrt::QueryType for IAsyncAction {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1516535814,
                    33850,
                    19881,
                    &[134, 91, 157, 38, 229, 223, 173, 123],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IAsyncAction {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IAsyncAction>> for IAsyncAction {
            fn into(self) -> winrt::Param<'a, IAsyncAction> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IAsyncAction>> for &'a IAsyncAction {
            fn into(self) -> winrt::Param<'a, IAsyncAction> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IAsyncActionWithProgress<TProgress: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_IAsyncActionWithProgress<TProgress: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#set_progress: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#progress: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#set_completed: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#completed: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#get_results: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TProgress>,
        }
        impl<TProgress: winrt::RuntimeType> IAsyncActionWithProgress<TProgress> {
            pub fn r#set_progress(
                &self,
                handler: &AsyncActionProgressHandler<TProgress>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncActionWithProgress<TProgress>)))
                        .r#set_progress)(
                        self.ptr.get(), winrt::RuntimeType::abi(handler)
                    )
                    .ok()
                }
            }
            pub fn r#progress(&self) -> winrt::Result<AsyncActionProgressHandler<TProgress>> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncActionWithProgress<TProgress>)))
                        .r#progress)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#set_completed(
                &self,
                handler: &AsyncActionWithProgressCompletedHandler<TProgress>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncActionWithProgress<TProgress>)))
                        .r#set_completed)(
                        self.ptr.get(), winrt::RuntimeType::abi(handler)
                    )
                    .ok()
                }
            }
            pub fn r#completed(
                &self,
            ) -> winrt::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncActionWithProgress<TProgress>)))
                        .r#completed)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_results(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncActionWithProgress<TProgress>)))
                        .r#get_results)(self.ptr.get())
                    .ok()
                }
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::InterfaceType for IAsyncActionWithProgress<TProgress> {}
        impl<TProgress: winrt::RuntimeType> winrt::QueryType for IAsyncActionWithProgress<TProgress> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    527282776,
                    59395,
                    18593,
                    &[149, 70, 235, 115, 83, 57, 136, 132],
                );
                &GUID
            }
        }
        impl<TProgress: winrt::RuntimeType> winrt::RuntimeType for IAsyncActionWithProgress<TProgress> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, IAsyncActionWithProgress<TProgress>>>
            for IAsyncActionWithProgress<TProgress>
        {
            fn into(self) -> winrt::Param<'a, IAsyncActionWithProgress<TProgress>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, IAsyncActionWithProgress<TProgress>>>
            for &'a IAsyncActionWithProgress<TProgress>
        {
            fn into(self) -> winrt::Param<'a, IAsyncActionWithProgress<TProgress>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IAsyncInfo {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IAsyncInfo {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#id: extern "system" fn(winrt::RawPtr, *mut u32) -> winrt::ErrorCode,
            r#status: extern "system" fn(winrt::RawPtr, *mut AsyncStatus) -> winrt::ErrorCode,
            r#error_code: extern "system" fn(winrt::RawPtr, *mut HResult) -> winrt::ErrorCode,
            r#cancel: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
            r#close: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IAsyncInfo {
            pub fn r#id(&self) -> winrt::Result<u32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncInfo))).r#id)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#status(&self) -> winrt::Result<AsyncStatus> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncInfo))).r#status)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#error_code(&self) -> winrt::Result<HResult> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncInfo))).r#error_code)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#cancel(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncInfo))).r#cancel)(
                        self.ptr.get(),
                    )
                    .ok()
                }
            }
            pub fn r#close(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncInfo))).r#close)(self.ptr.get())
                        .ok()
                }
            }
        }
        impl winrt::InterfaceType for IAsyncInfo {}
        impl winrt::QueryType for IAsyncInfo {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid =
                    winrt::Guid::from_values(54, 0, 0, &[192, 0, 0, 0, 0, 0, 0, 70]);
                &GUID
            }
        }
        impl winrt::RuntimeType for IAsyncInfo {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IAsyncInfo>> for IAsyncInfo {
            fn into(self) -> winrt::Param<'a, IAsyncInfo> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IAsyncInfo>> for &'a IAsyncInfo {
            fn into(self) -> winrt::Param<'a, IAsyncInfo> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IAsyncOperationWithProgress<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        #[repr(C)]
        struct abi_IAsyncOperationWithProgress<
            TResult: winrt::RuntimeType,
            TProgress: winrt::RuntimeType,
        > {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#set_progress: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#progress: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#set_completed: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#completed: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#get_results: extern "system" fn(
                winrt::RawPtr,
                *mut <TResult as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TResult>,
            __7: std::marker::PhantomData<TProgress>,
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            IAsyncOperationWithProgress<TResult, TProgress>
        {
            pub fn r#set_progress(
                &self,
                handler: &AsyncOperationProgressHandler<TResult, TProgress>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncOperationWithProgress<TResult, TProgress>)))
                        .r#set_progress)(
                        self.ptr.get(), winrt::RuntimeType::abi(handler)
                    )
                    .ok()
                }
            }
            pub fn r#progress(
                &self,
            ) -> winrt::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncOperationWithProgress<TResult, TProgress>)))
                        .r#progress)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#set_completed(
                &self,
                handler: &AsyncOperationWithProgressCompletedHandler<TResult, TProgress>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncOperationWithProgress<TResult, TProgress>)))
                        .r#set_completed)(
                        self.ptr.get(), winrt::RuntimeType::abi(handler)
                    )
                    .ok()
                }
            }
            pub fn r#completed(
                &self,
            ) -> winrt::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>
            {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncOperationWithProgress<TResult, TProgress>)))
                        .r#completed)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_results(&self) -> winrt::Result<TResult> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IAsyncOperationWithProgress<TResult, TProgress>)))
                        .r#get_results)(
                        self.ptr.get(),
                        <TResult as winrt::RuntimeType>::set_abi(&mut __ok),
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::InterfaceType
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::QueryType
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3050321623,
                    58007,
                    18831,
                    &[186, 96, 2, 137, 231, 110, 35, 221],
                );
                &GUID
            }
        }
        impl<TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType> winrt::RuntimeType
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, IAsyncOperationWithProgress<TResult, TProgress>>>
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into(self) -> winrt::Param<'a, IAsyncOperationWithProgress<TResult, TProgress>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TResult: winrt::RuntimeType, TProgress: winrt::RuntimeType>
            Into<winrt::Param<'a, IAsyncOperationWithProgress<TResult, TProgress>>>
            for &'a IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into(self) -> winrt::Param<'a, IAsyncOperationWithProgress<TResult, TProgress>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IAsyncOperation<TResult: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TResult>,
        }
        #[repr(C)]
        struct abi_IAsyncOperation<TResult: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#set_completed: extern "system" fn(winrt::RawPtr, winrt::RawPtr) -> winrt::ErrorCode,
            r#completed: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#get_results: extern "system" fn(
                winrt::RawPtr,
                *mut <TResult as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TResult>,
        }
        impl<TResult: winrt::RuntimeType> IAsyncOperation<TResult> {
            pub fn r#set_completed(
                &self,
                handler: &AsyncOperationCompletedHandler<TResult>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncOperation<TResult>)))
                        .r#set_completed)(
                        self.ptr.get(), winrt::RuntimeType::abi(handler)
                    )
                    .ok()
                }
            }
            pub fn r#completed(&self) -> winrt::Result<AsyncOperationCompletedHandler<TResult>> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncOperation<TResult>)))
                        .r#completed)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_results(&self) -> winrt::Result<TResult> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IAsyncOperation<TResult>)))
                        .r#get_results)(
                        self.ptr.get(),
                        <TResult as winrt::RuntimeType>::set_abi(&mut __ok),
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl<TResult: winrt::RuntimeType> winrt::InterfaceType for IAsyncOperation<TResult> {}
        impl<TResult: winrt::RuntimeType> winrt::QueryType for IAsyncOperation<TResult> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2680336571,
                    58438,
                    17634,
                    &[170, 97, 156, 171, 143, 99, 106, 242],
                );
                &GUID
            }
        }
        impl<TResult: winrt::RuntimeType> winrt::RuntimeType for IAsyncOperation<TResult> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TResult: winrt::RuntimeType> Into<winrt::Param<'a, IAsyncOperation<TResult>>>
            for IAsyncOperation<TResult>
        {
            fn into(self) -> winrt::Param<'a, IAsyncOperation<TResult>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TResult: winrt::RuntimeType> Into<winrt::Param<'a, IAsyncOperation<TResult>>>
            for &'a IAsyncOperation<TResult>
        {
            fn into(self) -> winrt::Param<'a, IAsyncOperation<TResult>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IClosable {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IClosable {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#close: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IClosable {
            pub fn r#close(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IClosable))).r#close)(self.ptr.get())
                        .ok()
                }
            }
        }
        impl winrt::InterfaceType for IClosable {}
        impl winrt::QueryType for IClosable {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    819308585,
                    32676,
                    16422,
                    &[131, 187, 215, 91, 174, 78, 169, 158],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IClosable {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for IClosable {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for &'a IClosable {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IDeferral {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IDeferral {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#complete: extern "system" fn(winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IDeferral {
            pub fn r#complete(&self) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IDeferral))).r#complete)(
                        self.ptr.get(),
                    )
                    .ok()
                }
            }
        }
        impl winrt::InterfaceType for IDeferral {}
        impl winrt::QueryType for IDeferral {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3592853298,
                    15231,
                    18087,
                    &[180, 11, 79, 220, 162, 162, 198, 147],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IDeferral {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferral>> for IDeferral {
            fn into(self) -> winrt::Param<'a, IDeferral> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferral>> for &'a IDeferral {
            fn into(self) -> winrt::Param<'a, IDeferral> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IDeferralFactory {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IDeferralFactory {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IDeferralFactory {
            pub fn r#create(&self, handler: &DeferralCompletedHandler) -> winrt::Result<Deferral> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IDeferralFactory))).r#create)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(handler),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IDeferralFactory {}
        impl winrt::QueryType for IDeferralFactory {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1705110725,
                    16309,
                    18482,
                    &[140, 169, 240, 97, 178, 129, 209, 58],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IDeferralFactory {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferralFactory>> for IDeferralFactory {
            fn into(self) -> winrt::Param<'a, IDeferralFactory> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IDeferralFactory>> for &'a IDeferralFactory {
            fn into(self) -> winrt::Param<'a, IDeferralFactory> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IGetActivationFactory {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IGetActivationFactory {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#get_activation_factory: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IGetActivationFactory {
            pub fn r#get_activation_factory<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                activatable_class_id: __0,
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IGetActivationFactory)))
                        .r#get_activation_factory)(
                        self.ptr.get(),
                        activatable_class_id.into().abi(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IGetActivationFactory {}
        impl winrt::QueryType for IGetActivationFactory {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1323011810,
                    38621,
                    18855,
                    &[148, 247, 70, 7, 221, 171, 142, 60],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IGetActivationFactory {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IGetActivationFactory>> for IGetActivationFactory {
            fn into(self) -> winrt::Param<'a, IGetActivationFactory> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IGetActivationFactory>> for &'a IGetActivationFactory {
            fn into(self) -> winrt::Param<'a, IGetActivationFactory> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IGuidHelperStatics {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IGuidHelperStatics {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create_new_guid:
                extern "system" fn(winrt::RawPtr, *mut winrt::Guid) -> winrt::ErrorCode,
            r#empty: extern "system" fn(winrt::RawPtr, *mut winrt::Guid) -> winrt::ErrorCode,
            r#equals: extern "system" fn(
                winrt::RawPtr,
                winrt::Guid,
                winrt::Guid,
                *mut bool,
            ) -> winrt::ErrorCode,
        }
        impl IGuidHelperStatics {
            pub fn r#create_new_guid(&self) -> winrt::Result<winrt::Guid> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IGuidHelperStatics)))
                        .r#create_new_guid)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#empty(&self) -> winrt::Result<winrt::Guid> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IGuidHelperStatics))).r#empty)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#equals(
                &self,
                target: &winrt::Guid,
                value: &winrt::Guid,
            ) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IGuidHelperStatics))).r#equals)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(target),
                        winrt::RuntimeType::abi(value),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IGuidHelperStatics {}
        impl winrt::QueryType for IGuidHelperStatics {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1506252395,
                    44626,
                    21123,
                    &[173, 127, 161, 185, 233, 103, 138, 221],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IGuidHelperStatics {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IGuidHelperStatics>> for IGuidHelperStatics {
            fn into(self) -> winrt::Param<'a, IGuidHelperStatics> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IGuidHelperStatics>> for &'a IGuidHelperStatics {
            fn into(self) -> winrt::Param<'a, IGuidHelperStatics> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IMemoryBuffer {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IMemoryBuffer {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create_reference:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IMemoryBuffer {
            pub fn r#create_reference(&self) -> winrt::Result<IMemoryBufferReference> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IMemoryBuffer))).r#create_reference)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IMemoryBuffer {}
        impl winrt::QueryType for IMemoryBuffer {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    4223982890,
                    9307,
                    4580,
                    &[175, 152, 104, 148, 35, 38, 12, 248],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IMemoryBuffer {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBuffer>> for IMemoryBuffer {
            fn into(self) -> winrt::Param<'a, IMemoryBuffer> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBuffer>> for &'a IMemoryBuffer {
            fn into(self) -> winrt::Param<'a, IMemoryBuffer> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IMemoryBufferFactory {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IMemoryBufferFactory {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create:
                extern "system" fn(winrt::RawPtr, u32, *mut winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IMemoryBufferFactory {
            pub fn r#create(&self, capacity: u32) -> winrt::Result<MemoryBuffer> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IMemoryBufferFactory))).r#create)(
                        self.ptr.get(),
                        capacity,
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IMemoryBufferFactory {}
        impl winrt::QueryType for IMemoryBufferFactory {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    4223982891,
                    9307,
                    4580,
                    &[175, 152, 104, 148, 35, 38, 12, 248],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IMemoryBufferFactory {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBufferFactory>> for IMemoryBufferFactory {
            fn into(self) -> winrt::Param<'a, IMemoryBufferFactory> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBufferFactory>> for &'a IMemoryBufferFactory {
            fn into(self) -> winrt::Param<'a, IMemoryBufferFactory> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IMemoryBufferReference {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IMemoryBufferReference {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#capacity: extern "system" fn(winrt::RawPtr, *mut u32) -> winrt::ErrorCode,
            r#closed:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, *mut i64) -> winrt::ErrorCode,
            r#remove_closed: extern "system" fn(winrt::RawPtr, i64) -> winrt::ErrorCode,
        }
        impl IMemoryBufferReference {
            pub fn r#capacity(&self) -> winrt::Result<u32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IMemoryBufferReference))).r#capacity)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IMemoryBufferReference {}
        impl winrt::QueryType for IMemoryBufferReference {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    4223982889,
                    9307,
                    4580,
                    &[175, 152, 104, 148, 35, 38, 12, 248],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IMemoryBufferReference {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBufferReference>> for IMemoryBufferReference {
            fn into(self) -> winrt::Param<'a, IMemoryBufferReference> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBufferReference>> for &'a IMemoryBufferReference {
            fn into(self) -> winrt::Param<'a, IMemoryBufferReference> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IPropertyValue {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IPropertyValue {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#type: extern "system" fn(winrt::RawPtr, *mut PropertyType) -> winrt::ErrorCode,
            r#is_numeric_scalar: extern "system" fn(winrt::RawPtr, *mut bool) -> winrt::ErrorCode,
            r#get_u_int8: extern "system" fn(winrt::RawPtr, *mut u8) -> winrt::ErrorCode,
            r#get_int16: extern "system" fn(winrt::RawPtr, *mut i16) -> winrt::ErrorCode,
            r#get_u_int16: extern "system" fn(winrt::RawPtr, *mut u16) -> winrt::ErrorCode,
            r#get_int32: extern "system" fn(winrt::RawPtr, *mut i32) -> winrt::ErrorCode,
            r#get_u_int32: extern "system" fn(winrt::RawPtr, *mut u32) -> winrt::ErrorCode,
            r#get_int64: extern "system" fn(winrt::RawPtr, *mut i64) -> winrt::ErrorCode,
            r#get_u_int64: extern "system" fn(winrt::RawPtr, *mut u64) -> winrt::ErrorCode,
            r#get_single: extern "system" fn(winrt::RawPtr, *mut f32) -> winrt::ErrorCode,
            r#get_double: extern "system" fn(winrt::RawPtr, *mut f64) -> winrt::ErrorCode,
            r#get_char16: extern "system" fn(winrt::RawPtr, *mut u16) -> winrt::ErrorCode,
            r#get_boolean: extern "system" fn(winrt::RawPtr, *mut bool) -> winrt::ErrorCode,
            r#get_string: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#get_guid: extern "system" fn(winrt::RawPtr, *mut winrt::Guid) -> winrt::ErrorCode,
            r#get_date_time: extern "system" fn(winrt::RawPtr, *mut DateTime) -> winrt::ErrorCode,
            r#get_time_span: extern "system" fn(winrt::RawPtr, *mut TimeSpan) -> winrt::ErrorCode,
            r#get_point: extern "system" fn(winrt::RawPtr, *mut Point) -> winrt::ErrorCode,
            r#get_size: extern "system" fn(winrt::RawPtr, *mut Size) -> winrt::ErrorCode,
            r#get_rect: extern "system" fn(winrt::RawPtr, *mut Rect) -> winrt::ErrorCode,
            r#get_u_int8_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut u8) -> winrt::ErrorCode,
            r#get_int16_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut i16) -> winrt::ErrorCode,
            r#get_u_int16_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut u16) -> winrt::ErrorCode,
            r#get_int32_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut i32) -> winrt::ErrorCode,
            r#get_u_int32_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut u32) -> winrt::ErrorCode,
            r#get_int64_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut i64) -> winrt::ErrorCode,
            r#get_u_int64_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut u64) -> winrt::ErrorCode,
            r#get_single_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut f32) -> winrt::ErrorCode,
            r#get_double_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut f64) -> winrt::ErrorCode,
            r#get_char16_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut u16) -> winrt::ErrorCode,
            r#get_boolean_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut bool) -> winrt::ErrorCode,
            r#get_string_array: extern "system" fn(
                winrt::RawPtr,
                *mut u32,
                *mut *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#get_inspectable_array: extern "system" fn(
                winrt::RawPtr,
                *mut u32,
                *mut *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#get_guid_array: extern "system" fn(
                winrt::RawPtr,
                *mut u32,
                *mut *mut winrt::Guid,
            ) -> winrt::ErrorCode,
            r#get_date_time_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut DateTime) -> winrt::ErrorCode,
            r#get_time_span_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut TimeSpan) -> winrt::ErrorCode,
            r#get_point_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut Point) -> winrt::ErrorCode,
            r#get_size_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut Size) -> winrt::ErrorCode,
            r#get_rect_array:
                extern "system" fn(winrt::RawPtr, *mut u32, *mut *mut Rect) -> winrt::ErrorCode,
        }
        impl IPropertyValue {
            pub fn r#type(&self) -> winrt::Result<PropertyType> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#type)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#is_numeric_scalar(&self) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*
                      (*
                       (self . ptr . get () as * const * const
                        abi_IPropertyValue < >))) . r#is_numeric_scalar)
                    (self . ptr . get (), & mut __ok) . ok_or
                    (std :: mem :: transmute_copy (& __ok))
                }
            }
            pub fn r#get_u_int8(&self) -> winrt::Result<u8> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_u_int8)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_int16(&self) -> winrt::Result<i16> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int16)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_u_int16(&self) -> winrt::Result<u16> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_u_int16)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_int32(&self) -> winrt::Result<i32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int32)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_u_int32(&self) -> winrt::Result<u32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_u_int32)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_int64(&self) -> winrt::Result<i64> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int64)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_u_int64(&self) -> winrt::Result<u64> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_u_int64)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_single(&self) -> winrt::Result<f32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_single)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_double(&self) -> winrt::Result<f64> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_double)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_char16(&self) -> winrt::Result<u16> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_char16)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_boolean(&self) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_boolean)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_string(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_string)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_guid(&self) -> winrt::Result<winrt::Guid> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_guid)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_date_time(&self) -> winrt::Result<DateTime> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_date_time)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_time_span(&self) -> winrt::Result<TimeSpan> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_time_span)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_point(&self) -> winrt::Result<Point> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_point)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_size(&self) -> winrt::Result<Size> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_size)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_rect(&self) -> winrt::Result<Rect> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_rect)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#get_u_int8_array(&self, value: &mut winrt::Array<u8>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_u_int8_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_int16_array(&self, value: &mut winrt::Array<i16>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int16_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_u_int16_array(&self, value: &mut winrt::Array<u16>) -> winrt::Result<()> {
                unsafe {
                    ((*
                      (*
                       (self . ptr . get () as * const * const
                        abi_IPropertyValue < >))) . r#get_u_int16_array)
                    (self . ptr . get (), value . set_abi_len (), value .
                     set_abi ()) . ok ()
                }
            }
            pub fn r#get_int32_array(&self, value: &mut winrt::Array<i32>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int32_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_u_int32_array(&self, value: &mut winrt::Array<u32>) -> winrt::Result<()> {
                unsafe {
                    ((*
                      (*
                       (self . ptr . get () as * const * const
                        abi_IPropertyValue < >))) . r#get_u_int32_array)
                    (self . ptr . get (), value . set_abi_len (), value .
                     set_abi ()) . ok ()
                }
            }
            pub fn r#get_int64_array(&self, value: &mut winrt::Array<i64>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_int64_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_u_int64_array(&self, value: &mut winrt::Array<u64>) -> winrt::Result<()> {
                unsafe {
                    ((*
                      (*
                       (self . ptr . get () as * const * const
                        abi_IPropertyValue < >))) . r#get_u_int64_array)
                    (self . ptr . get (), value . set_abi_len (), value .
                     set_abi ()) . ok ()
                }
            }
            pub fn r#get_single_array(&self, value: &mut winrt::Array<f32>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_single_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_double_array(&self, value: &mut winrt::Array<f64>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_double_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_char16_array(&self, value: &mut winrt::Array<u16>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_char16_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_boolean_array(&self, value: &mut winrt::Array<bool>) -> winrt::Result<()> {
                unsafe {
                    ((*
                      (*
                       (self . ptr . get () as * const * const
                        abi_IPropertyValue < >))) . r#get_boolean_array)
                    (self . ptr . get (), value . set_abi_len (), value .
                     set_abi ()) . ok ()
                }
            }
            pub fn r#get_string_array(
                &self,
                value: &mut winrt::Array<winrt::String>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_string_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_inspectable_array(
                &self,
                value: &mut winrt::Array<winrt::Object>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue)))
                        .r#get_inspectable_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_guid_array(
                &self,
                value: &mut winrt::Array<winrt::Guid>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_guid_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_date_time_array(
                &self,
                value: &mut winrt::Array<DateTime>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue)))
                        .r#get_date_time_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_time_span_array(
                &self,
                value: &mut winrt::Array<TimeSpan>,
            ) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue)))
                        .r#get_time_span_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_point_array(&self, value: &mut winrt::Array<Point>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_point_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_size_array(&self, value: &mut winrt::Array<Size>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_size_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn r#get_rect_array(&self, value: &mut winrt::Array<Rect>) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValue))).r#get_rect_array)(
                        self.ptr.get(),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
        }
        impl winrt::InterfaceType for IPropertyValue {}
        impl winrt::QueryType for IPropertyValue {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1272349405,
                    30036,
                    16617,
                    &[154, 155, 130, 101, 78, 222, 126, 98],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IPropertyValue {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IPropertyValue>> for IPropertyValue {
            fn into(self) -> winrt::Param<'a, IPropertyValue> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IPropertyValue>> for &'a IPropertyValue {
            fn into(self) -> winrt::Param<'a, IPropertyValue> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IPropertyValueStatics {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IPropertyValueStatics {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create_empty:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_u_int8:
                extern "system" fn(winrt::RawPtr, u8, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_int16:
                extern "system" fn(winrt::RawPtr, i16, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_u_int16:
                extern "system" fn(winrt::RawPtr, u16, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_int32:
                extern "system" fn(winrt::RawPtr, i32, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_u_int32:
                extern "system" fn(winrt::RawPtr, u32, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_int64:
                extern "system" fn(winrt::RawPtr, i64, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_u_int64:
                extern "system" fn(winrt::RawPtr, u64, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_single:
                extern "system" fn(winrt::RawPtr, f32, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_double:
                extern "system" fn(winrt::RawPtr, f64, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_char16:
                extern "system" fn(winrt::RawPtr, u16, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_boolean:
                extern "system" fn(winrt::RawPtr, bool, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_string: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_inspectable: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_guid: extern "system" fn(
                winrt::RawPtr,
                winrt::Guid,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_date_time:
                extern "system" fn(winrt::RawPtr, DateTime, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_time_span:
                extern "system" fn(winrt::RawPtr, TimeSpan, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_point:
                extern "system" fn(winrt::RawPtr, Point, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_size:
                extern "system" fn(winrt::RawPtr, Size, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_rect:
                extern "system" fn(winrt::RawPtr, Rect, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#create_u_int8_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const u8,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_int16_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const i16,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_u_int16_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const u16,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_int32_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const i32,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_u_int32_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const u32,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_int64_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const i64,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_u_int64_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const u64,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_single_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const f32,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_double_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const f64,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_char16_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const u16,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_boolean_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const bool,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_string_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_inspectable_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_guid_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const winrt::Guid,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_date_time_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const DateTime,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_time_span_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const TimeSpan,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_point_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const Point,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_size_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const Size,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_rect_array: extern "system" fn(
                winrt::RawPtr,
                u32,
                *const Rect,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IPropertyValueStatics {
            pub fn r#create_empty(&self) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_empty)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int8(&self, value: u8) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int8)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int16(&self, value: i16) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int16)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int16(&self, value: u16) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int16)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int32(&self, value: i32) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int32)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int32(&self, value: u32) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int32)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int64(&self, value: i64) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int64)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int64(&self, value: u64) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int64)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_single(&self, value: f32) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_single)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_double(&self, value: f64) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_double)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_char16(&self, value: u16) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_char16)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_boolean(&self, value: bool) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_boolean)(self.ptr.get(), value, &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_string<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                value: __0,
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_string)(
                        self.ptr.get(), value.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_inspectable(
                &self,
                value: &winrt::Object,
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_inspectable)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(value),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_guid(&self, value: &winrt::Guid) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_guid)(
                        self.ptr.get(), winrt::RuntimeType::abi(value), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_date_time(&self, value: &DateTime) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_date_time)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(value),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_time_span(&self, value: &TimeSpan) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_time_span)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(value),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_point(&self, value: &Point) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_point)(
                        self.ptr.get(), winrt::RuntimeType::abi(value), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_size(&self, value: &Size) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_size)(
                        self.ptr.get(), winrt::RuntimeType::abi(value), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_rect(&self, value: &Rect) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_rect)(
                        self.ptr.get(), winrt::RuntimeType::abi(value), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int8_array(&self, value: &[u8]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int8_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int16_array(&self, value: &[i16]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int16_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int16_array(&self, value: &[u16]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int16_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int32_array(&self, value: &[i32]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int32_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int32_array(&self, value: &[u32]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int32_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_int64_array(&self, value: &[i64]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_int64_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_u_int64_array(&self, value: &[u64]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_u_int64_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_single_array(&self, value: &[f32]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_single_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_double_array(&self, value: &[f64]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_double_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_char16_array(&self, value: &[u16]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_char16_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_boolean_array(&self, value: &[bool]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_boolean_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_string_array(
                &self,
                value: &[winrt::String],
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_string_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_inspectable_array(
                &self,
                value: &[winrt::Object],
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_inspectable_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_guid_array(
                &self,
                value: &[winrt::Guid],
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_guid_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_date_time_array(
                &self,
                value: &[DateTime],
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_date_time_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_time_span_array(
                &self,
                value: &[TimeSpan],
            ) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_time_span_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_point_array(&self, value: &[Point]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_point_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_size_array(&self, value: &[Size]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_size_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_rect_array(&self, value: &[Rect]) -> winrt::Result<winrt::Object> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IPropertyValueStatics)))
                        .r#create_rect_array)(
                        self.ptr.get(),
                        value.len() as u32,
                        std::mem::transmute(value.as_ptr()),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IPropertyValueStatics {}
        impl winrt::QueryType for IPropertyValueStatics {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1654381512,
                    55602,
                    20468,
                    &[150, 185, 141, 150, 197, 193, 232, 88],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IPropertyValueStatics {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IPropertyValueStatics>> for IPropertyValueStatics {
            fn into(self) -> winrt::Param<'a, IPropertyValueStatics> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IPropertyValueStatics>> for &'a IPropertyValueStatics {
            fn into(self) -> winrt::Param<'a, IPropertyValueStatics> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IReference<T: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<T>,
        }
        #[repr(C)]
        struct abi_IReference<T: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#value: extern "system" fn(
                winrt::RawPtr,
                *mut <T as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<T>,
        }
        impl<T: winrt::RuntimeType> IReference<T> {
            pub fn r#value(&self) -> winrt::Result<T> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IReference<T>))).r#value)(
                        self.ptr.get(),
                        <T as winrt::RuntimeType>::set_abi(&mut __ok),
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl<T: winrt::RuntimeType> winrt::InterfaceType for IReference<T> {}
        impl<T: winrt::RuntimeType> winrt::QueryType for IReference<T> {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1640068870,
                    11621,
                    4576,
                    &[154, 232, 212, 133, 100, 1, 84, 114],
                );
                &GUID
            }
        }
        impl<T: winrt::RuntimeType> winrt::RuntimeType for IReference<T> {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, T: winrt::RuntimeType> Into<winrt::Param<'a, IReference<T>>> for IReference<T> {
            fn into(self) -> winrt::Param<'a, IReference<T>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, T: winrt::RuntimeType> Into<winrt::Param<'a, IReference<T>>> for &'a IReference<T> {
            fn into(self) -> winrt::Param<'a, IReference<T>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IStringable {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IStringable {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#to_string: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IStringable {
            pub fn r#to_string(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IStringable))).r#to_string)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IStringable {}
        impl winrt::QueryType for IStringable {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2520162132,
                    36534,
                    18672,
                    &[171, 206, 193, 178, 17, 230, 39, 195],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IStringable {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IStringable>> for IStringable {
            fn into(self) -> winrt::Param<'a, IStringable> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IStringable>> for &'a IStringable {
            fn into(self) -> winrt::Param<'a, IStringable> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IUriEscapeStatics {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IUriEscapeStatics {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#unescape_component: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#escape_component: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IUriEscapeStatics {
            pub fn r#unescape_component<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                to_unescape: __0,
            ) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriEscapeStatics)))
                        .r#unescape_component)(
                        self.ptr.get(), to_unescape.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#escape_component<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                to_escape: __0,
            ) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriEscapeStatics)))
                        .r#escape_component)(
                        self.ptr.get(), to_escape.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IUriEscapeStatics {}
        impl winrt::QueryType for IUriEscapeStatics {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3251909306,
                    51236,
                    17490,
                    &[167, 253, 81, 43, 195, 187, 233, 161],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IUriEscapeStatics {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriEscapeStatics>> for IUriEscapeStatics {
            fn into(self) -> winrt::Param<'a, IUriEscapeStatics> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriEscapeStatics>> for &'a IUriEscapeStatics {
            fn into(self) -> winrt::Param<'a, IUriEscapeStatics> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IUriRuntimeClass {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IUriRuntimeClass {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#absolute_uri:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#display_uri:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#domain: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#extension: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#fragment: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#host: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#password: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#path: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#query: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#query_parsed:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#raw_uri: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#scheme_name:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#user_name: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#port: extern "system" fn(winrt::RawPtr, *mut i32) -> winrt::ErrorCode,
            r#suspicious: extern "system" fn(winrt::RawPtr, *mut bool) -> winrt::ErrorCode,
            r#equals:
                extern "system" fn(winrt::RawPtr, winrt::RawPtr, *mut bool) -> winrt::ErrorCode,
            r#combine_uri: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IUriRuntimeClass {
            pub fn r#absolute_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#absolute_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#display_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#display_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#domain(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#domain)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#extension(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#extension)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#fragment(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#fragment)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#host(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#host)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#password(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#password)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#path(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#path)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#query(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#query)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#query_parsed(&self) -> winrt::Result<WwwFormUrlDecoder> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#query_parsed)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#raw_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#raw_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#scheme_name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#scheme_name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#user_name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#user_name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#port(&self) -> winrt::Result<i32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#port)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#suspicious(&self) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#suspicious)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#equals(&self, p_uri: &Uri) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#equals)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(p_uri),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#combine_uri<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                relative_uri: __0,
            ) -> winrt::Result<Uri> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#combine_uri)(
                        self.ptr.get(),
                        relative_uri.into().abi(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IUriRuntimeClass {}
        impl winrt::QueryType for IUriRuntimeClass {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2654363223,
                    18610,
                    16736,
                    &[149, 111, 199, 56, 81, 32, 187, 252],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IUriRuntimeClass {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClass>> for IUriRuntimeClass {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClass> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClass>> for &'a IUriRuntimeClass {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClass> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IUriRuntimeClassFactory {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IUriRuntimeClassFactory {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create_uri: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
            r#create_with_relative_uri: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IUriRuntimeClassFactory {
            pub fn r#create_uri<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                uri: __0,
            ) -> winrt::Result<Uri> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClassFactory)))
                        .r#create_uri)(
                        self.ptr.get(), uri.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#create_with_relative_uri<
                'a,
                __0: Into<winrt::StringParam<'a>>,
                __1: Into<winrt::StringParam<'a>>,
            >(
                &self,
                base_uri: __0,
                relative_uri: __1,
            ) -> winrt::Result<Uri> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClassFactory)))
                        .r#create_with_relative_uri)(
                        self.ptr.get(),
                        base_uri.into().abi(),
                        relative_uri.into().abi(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IUriRuntimeClassFactory {}
        impl winrt::QueryType for IUriRuntimeClassFactory {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1151957359,
                    29246,
                    20447,
                    &[162, 24, 3, 62, 117, 176, 192, 132],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IUriRuntimeClassFactory {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassFactory>> for IUriRuntimeClassFactory {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassFactory> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassFactory>> for &'a IUriRuntimeClassFactory {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassFactory> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IUriRuntimeClassWithAbsoluteCanonicalUri {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#absolute_canonical_uri:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#display_iri:
                extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IUriRuntimeClassWithAbsoluteCanonicalUri {
            pub fn r#absolute_canonical_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IUriRuntimeClassWithAbsoluteCanonicalUri)))
                        .r#absolute_canonical_uri)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#display_iri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IUriRuntimeClassWithAbsoluteCanonicalUri)))
                        .r#display_iri)(self.ptr.get(), &mut __ok)
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IUriRuntimeClassWithAbsoluteCanonicalUri {}
        impl winrt::QueryType for IUriRuntimeClassWithAbsoluteCanonicalUri {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1972213345,
                    8732,
                    18447,
                    &[163, 57, 80, 101, 102, 115, 244, 111],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri>>
            for IUriRuntimeClassWithAbsoluteCanonicalUri
        {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri>>
            for &'a IUriRuntimeClassWithAbsoluteCanonicalUri
        {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IWwwFormUrlDecoderEntry {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IWwwFormUrlDecoderEntry {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#name: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
            r#value: extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode,
        }
        impl IWwwFormUrlDecoderEntry {
            pub fn r#name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderEntry))).r#name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#value(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderEntry))).r#value)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IWwwFormUrlDecoderEntry {}
        impl winrt::QueryType for IWwwFormUrlDecoderEntry {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    308180017,
                    63096,
                    20110,
                    &[182, 112, 32, 169, 176, 108, 81, 45],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IWwwFormUrlDecoderEntry {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderEntry>> for IWwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderEntry> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderEntry>> for &'a IWwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderEntry> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IWwwFormUrlDecoderRuntimeClass {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IWwwFormUrlDecoderRuntimeClass {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#get_first_value_by_name: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IWwwFormUrlDecoderRuntimeClass {
            pub fn r#get_first_value_by_name<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                name: __0,
            ) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderRuntimeClass)))
                        .r#get_first_value_by_name)(
                        self.ptr.get(), name.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IWwwFormUrlDecoderRuntimeClass {}
        impl winrt::QueryType for IWwwFormUrlDecoderRuntimeClass {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    3562669137,
                    61989,
                    17730,
                    &[146, 150, 14, 29, 245, 210, 84, 223],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IWwwFormUrlDecoderRuntimeClass {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass>> for IWwwFormUrlDecoderRuntimeClass {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass>>
            for &'a IWwwFormUrlDecoderRuntimeClass
        {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct IWwwFormUrlDecoderRuntimeClassFactory {
            ptr: winrt::ComPtr,
        }
        #[repr(C)]
        struct abi_IWwwFormUrlDecoderRuntimeClassFactory {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            r#create_www_form_url_decoder: extern "system" fn(
                winrt::RawPtr,
                winrt::RawPtr,
                *mut winrt::RawPtr,
            ) -> winrt::ErrorCode,
        }
        impl IWwwFormUrlDecoderRuntimeClassFactory {
            pub fn r#create_www_form_url_decoder<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                query: __0,
            ) -> winrt::Result<WwwFormUrlDecoder> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get()
                        as *const *const abi_IWwwFormUrlDecoderRuntimeClassFactory)))
                        .r#create_www_form_url_decoder)(
                        self.ptr.get(),
                        query.into().abi(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::InterfaceType for IWwwFormUrlDecoderRuntimeClassFactory {}
        impl winrt::QueryType for IWwwFormUrlDecoderRuntimeClassFactory {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    1535929149,
                    9390,
                    16821,
                    &[161, 191, 240, 195, 213, 68, 132, 91],
                );
                &GUID
            }
        }
        impl winrt::RuntimeType for IWwwFormUrlDecoderRuntimeClassFactory {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClassFactory>>
            for IWwwFormUrlDecoderRuntimeClassFactory
        {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClassFactory> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClassFactory>>
            for &'a IWwwFormUrlDecoderRuntimeClassFactory
        {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClassFactory> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct MemoryBuffer {
            ptr: winrt::ComPtr,
        }
        impl MemoryBuffer {
            pub fn r#create(capacity: u32) -> winrt::Result<MemoryBuffer> {
                winrt::factory::<MemoryBuffer, IMemoryBufferFactory>()?.r#create(capacity)
            }
            pub fn r#create_reference(&self) -> winrt::Result<IMemoryBufferReference> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IMemoryBuffer))).r#create_reference)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::ClassType for MemoryBuffer {}
        impl winrt::QueryType for MemoryBuffer {
            fn type_guid() -> &'static winrt::Guid {
                <IMemoryBuffer as winrt::QueryType>::type_guid()
            }
        }
        impl winrt::TypeName for MemoryBuffer {
            fn type_name() -> &'static str {
                "Windows.Foundation.MemoryBuffer"
            }
        }
        impl winrt::RuntimeType for MemoryBuffer {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, MemoryBuffer>> for MemoryBuffer {
            fn into(self) -> winrt::Param<'a, MemoryBuffer> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, MemoryBuffer>> for &'a MemoryBuffer {
            fn into(self) -> winrt::Param<'a, MemoryBuffer> {
                winrt::Param::Ref(self)
            }
        }
        impl From<MemoryBuffer> for IMemoryBuffer {
            fn from(value: MemoryBuffer) -> IMemoryBuffer {
                unsafe { std::mem::transmute(value) }
            }
        }
        impl From<&MemoryBuffer> for IMemoryBuffer {
            fn from(value: &MemoryBuffer) -> IMemoryBuffer {
                unsafe { std::mem::transmute(value.clone()) }
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBuffer>> for MemoryBuffer {
            fn into(self) -> winrt::Param<'a, IMemoryBuffer> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IMemoryBuffer>> for &'a MemoryBuffer {
            fn into(self) -> winrt::Param<'a, IMemoryBuffer> {
                winrt::Param::Value(self.into())
            }
        }
        impl From<MemoryBuffer> for IClosable {
            fn from(value: MemoryBuffer) -> IClosable {
                IClosable::from(&value)
            }
        }
        impl From<&MemoryBuffer> for IClosable {
            fn from(value: &MemoryBuffer) -> IClosable {
                winrt::QueryType::query(value)
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for MemoryBuffer {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IClosable>> for &'a MemoryBuffer {
            fn into(self) -> winrt::Param<'a, IClosable> {
                winrt::Param::Value(self.into())
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct Point {
            pub r#x: u8,
            pub r#y: u8,
        }
        impl winrt::RuntimeCopy for Point {}
        pub enum PropertyType {
            Empty,
            UInt8,
            Int16,
            UInt16,
            Int32,
            UInt32,
            Int64,
            UInt64,
            Single,
            Double,
            Char16,
            Boolean,
            String,
            Inspectable,
            DateTime,
            TimeSpan,
            Guid,
            Point,
            Size,
            Rect,
            OtherType,
            UInt8Array,
            Int16Array,
            UInt16Array,
            Int32Array,
            UInt32Array,
            Int64Array,
            UInt64Array,
            SingleArray,
            DoubleArray,
            Char16Array,
            BooleanArray,
            StringArray,
            InspectableArray,
            DateTimeArray,
            TimeSpanArray,
            GuidArray,
            PointArray,
            SizeArray,
            RectArray,
            OtherTypeArray,
        }
        impl Default for PropertyType {
            fn default() -> Self {
                Self::Empty
            }
        }
        pub struct PropertyValue {}
        impl PropertyValue {
            pub fn r#create_empty() -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_empty()
            }
            pub fn r#create_u_int8(value: u8) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_u_int8(value)
            }
            pub fn r#create_int16(value: i16) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_int16(value)
            }
            pub fn r#create_u_int16(value: u16) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_u_int16(value)
            }
            pub fn r#create_int32(value: i32) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_int32(value)
            }
            pub fn r#create_u_int32(value: u32) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_u_int32(value)
            }
            pub fn r#create_int64(value: i64) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_int64(value)
            }
            pub fn r#create_u_int64(value: u64) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_u_int64(value)
            }
            pub fn r#create_single(value: f32) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_single(value)
            }
            pub fn r#create_double(value: f64) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_double(value)
            }
            pub fn r#create_char16(value: u16) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_char16(value)
            }
            pub fn r#create_boolean(value: bool) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_boolean(value)
            }
            pub fn r#create_string<'a, __0: Into<winrt::StringParam<'a>>>(
                value: __0,
            ) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_string(value)
            }
            pub fn r#create_inspectable(value: &winrt::Object) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_inspectable(value)
            }
            pub fn r#create_guid(value: &winrt::Guid) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_guid(value)
            }
            pub fn r#create_date_time(value: &DateTime) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_date_time(value)
            }
            pub fn r#create_time_span(value: &TimeSpan) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_time_span(value)
            }
            pub fn r#create_point(value: &Point) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_point(value)
            }
            pub fn r#create_size(value: &Size) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_size(value)
            }
            pub fn r#create_rect(value: &Rect) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_rect(value)
            }
            pub fn r#create_u_int8_array(value: &[u8]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_u_int8_array(value)
            }
            pub fn r#create_int16_array(value: &[i16]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_int16_array(value)
            }
            pub fn r#create_u_int16_array(value: &[u16]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_u_int16_array(value)
            }
            pub fn r#create_int32_array(value: &[i32]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_int32_array(value)
            }
            pub fn r#create_u_int32_array(value: &[u32]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_u_int32_array(value)
            }
            pub fn r#create_int64_array(value: &[i64]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_int64_array(value)
            }
            pub fn r#create_u_int64_array(value: &[u64]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_u_int64_array(value)
            }
            pub fn r#create_single_array(value: &[f32]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_single_array(value)
            }
            pub fn r#create_double_array(value: &[f64]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_double_array(value)
            }
            pub fn r#create_char16_array(value: &[u16]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_char16_array(value)
            }
            pub fn r#create_boolean_array(value: &[bool]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_boolean_array(value)
            }
            pub fn r#create_string_array(value: &[winrt::String]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_string_array(value)
            }
            pub fn r#create_inspectable_array(
                value: &[winrt::Object],
            ) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_inspectable_array(value)
            }
            pub fn r#create_guid_array(value: &[winrt::Guid]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_guid_array(value)
            }
            pub fn r#create_date_time_array(value: &[DateTime]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_date_time_array(value)
            }
            pub fn r#create_time_span_array(value: &[TimeSpan]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_time_span_array(value)
            }
            pub fn r#create_point_array(value: &[Point]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?
                    .r#create_point_array(value)
            }
            pub fn r#create_size_array(value: &[Size]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_size_array(value)
            }
            pub fn r#create_rect_array(value: &[Rect]) -> winrt::Result<winrt::Object> {
                winrt::factory::<PropertyValue, IPropertyValueStatics>()?.r#create_rect_array(value)
            }
        }
        impl winrt::TypeName for PropertyValue {
            fn type_name() -> &'static str {
                "Windows.Foundation.PropertyValue"
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct Rect {
            pub r#x: u8,
            pub r#y: u8,
            pub r#width: u8,
            pub r#height: u8,
        }
        impl winrt::RuntimeCopy for Rect {}
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct Size {
            pub r#width: u8,
            pub r#height: u8,
        }
        impl winrt::RuntimeCopy for Size {}
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        pub struct TimeSpan {
            pub r#duration: u8,
        }
        impl winrt::RuntimeCopy for TimeSpan {}
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct TypedEventHandler<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> {
            ptr: winrt::ComPtr,
            __6: std::marker::PhantomData<TSender>,
            __7: std::marker::PhantomData<TResult>,
        }
        #[repr(C)]
        struct abi_TypedEventHandler<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> {
            __0: usize,
            __1: usize,
            __2: usize,
            r#invoke: extern "system" fn(
                winrt::RawPtr,
                <TSender as winrt::RuntimeType>::Abi,
                <TResult as winrt::RuntimeType>::Abi,
            ) -> winrt::ErrorCode,
            __6: std::marker::PhantomData<TSender>,
            __7: std::marker::PhantomData<TResult>,
        }
        impl<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> TypedEventHandler<TSender, TResult> {
            pub fn r#invoke(&self, sender: &TSender, args: &TResult) -> winrt::Result<()> {
                unsafe {
                    ((*(*(self.ptr.get()
                        as *const *const abi_TypedEventHandler<TSender, TResult>)))
                        .r#invoke)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(sender),
                        winrt::RuntimeType::abi(args),
                    )
                    .ok()
                }
            }
        }
        impl<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> winrt::DelegateType
            for TypedEventHandler<TSender, TResult>
        {
        }
        impl<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> winrt::QueryType
            for TypedEventHandler<TSender, TResult>
        {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    2648818996,
                    27361,
                    4576,
                    &[132, 225, 24, 169, 5, 188, 197, 63],
                );
                &GUID
            }
        }
        impl<TSender: winrt::RuntimeType, TResult: winrt::RuntimeType> winrt::RuntimeType
            for TypedEventHandler<TSender, TResult>
        {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a, TSender: winrt::RuntimeType, TResult: winrt::RuntimeType>
            Into<winrt::Param<'a, TypedEventHandler<TSender, TResult>>>
            for TypedEventHandler<TSender, TResult>
        {
            fn into(self) -> winrt::Param<'a, TypedEventHandler<TSender, TResult>> {
                winrt::Param::Value(self)
            }
        }
        impl<'a, TSender: winrt::RuntimeType, TResult: winrt::RuntimeType>
            Into<winrt::Param<'a, TypedEventHandler<TSender, TResult>>>
            for &'a TypedEventHandler<TSender, TResult>
        {
            fn into(self) -> winrt::Param<'a, TypedEventHandler<TSender, TResult>> {
                winrt::Param::Ref(self)
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct Uri {
            ptr: winrt::ComPtr,
        }
        impl Uri {
            pub fn r#unescape_component<'a, __0: Into<winrt::StringParam<'a>>>(
                to_unescape: __0,
            ) -> winrt::Result<winrt::String> {
                winrt::factory::<Uri, IUriEscapeStatics>()?.r#unescape_component(to_unescape)
            }
            pub fn r#escape_component<'a, __0: Into<winrt::StringParam<'a>>>(
                to_escape: __0,
            ) -> winrt::Result<winrt::String> {
                winrt::factory::<Uri, IUriEscapeStatics>()?.r#escape_component(to_escape)
            }
            pub fn r#create_uri<'a, __0: Into<winrt::StringParam<'a>>>(
                uri: __0,
            ) -> winrt::Result<Uri> {
                winrt::factory::<Uri, IUriRuntimeClassFactory>()?.r#create_uri(uri)
            }
            pub fn r#create_with_relative_uri<
                'a,
                __0: Into<winrt::StringParam<'a>>,
                __1: Into<winrt::StringParam<'a>>,
            >(
                base_uri: __0,
                relative_uri: __1,
            ) -> winrt::Result<Uri> {
                winrt::factory::<Uri, IUriRuntimeClassFactory>()?
                    .r#create_with_relative_uri(base_uri, relative_uri)
            }
            pub fn r#absolute_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#absolute_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#display_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#display_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#domain(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#domain)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#extension(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#extension)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#fragment(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#fragment)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#host(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#host)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#password(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#password)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#path(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#path)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#query(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#query)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#query_parsed(&self) -> winrt::Result<WwwFormUrlDecoder> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#query_parsed)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#raw_uri(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#raw_uri)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#scheme_name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#scheme_name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#user_name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#user_name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#port(&self) -> winrt::Result<i32> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#port)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#suspicious(&self) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#suspicious)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#equals(&self, p_uri: &Uri) -> winrt::Result<bool> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#equals)(
                        self.ptr.get(),
                        winrt::RuntimeType::abi(p_uri),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#combine_uri<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                relative_uri: __0,
            ) -> winrt::Result<Uri> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IUriRuntimeClass))).r#combine_uri)(
                        self.ptr.get(),
                        relative_uri.into().abi(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::ClassType for Uri {}
        impl winrt::QueryType for Uri {
            fn type_guid() -> &'static winrt::Guid {
                <IUriRuntimeClass as winrt::QueryType>::type_guid()
            }
        }
        impl winrt::TypeName for Uri {
            fn type_name() -> &'static str {
                "Windows.Foundation.Uri"
            }
        }
        impl winrt::RuntimeType for Uri {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, Uri>> for Uri {
            fn into(self) -> winrt::Param<'a, Uri> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, Uri>> for &'a Uri {
            fn into(self) -> winrt::Param<'a, Uri> {
                winrt::Param::Ref(self)
            }
        }
        impl From<Uri> for IStringable {
            fn from(value: Uri) -> IStringable {
                IStringable::from(&value)
            }
        }
        impl From<&Uri> for IStringable {
            fn from(value: &Uri) -> IStringable {
                winrt::QueryType::query(value)
            }
        }
        impl<'a> Into<winrt::Param<'a, IStringable>> for Uri {
            fn into(self) -> winrt::Param<'a, IStringable> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IStringable>> for &'a Uri {
            fn into(self) -> winrt::Param<'a, IStringable> {
                winrt::Param::Value(self.into())
            }
        }
        impl From<Uri> for IUriRuntimeClass {
            fn from(value: Uri) -> IUriRuntimeClass {
                unsafe { std::mem::transmute(value) }
            }
        }
        impl From<&Uri> for IUriRuntimeClass {
            fn from(value: &Uri) -> IUriRuntimeClass {
                unsafe { std::mem::transmute(value.clone()) }
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClass>> for Uri {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClass> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClass>> for &'a Uri {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClass> {
                winrt::Param::Value(self.into())
            }
        }
        impl From<Uri> for IUriRuntimeClassWithAbsoluteCanonicalUri {
            fn from(value: Uri) -> IUriRuntimeClassWithAbsoluteCanonicalUri {
                IUriRuntimeClassWithAbsoluteCanonicalUri::from(&value)
            }
        }
        impl From<&Uri> for IUriRuntimeClassWithAbsoluteCanonicalUri {
            fn from(value: &Uri) -> IUriRuntimeClassWithAbsoluteCanonicalUri {
                winrt::QueryType::query(value)
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri>> for Uri {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri>> for &'a Uri {
            fn into(self) -> winrt::Param<'a, IUriRuntimeClassWithAbsoluteCanonicalUri> {
                winrt::Param::Value(self.into())
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct WwwFormUrlDecoder {
            ptr: winrt::ComPtr,
        }
        impl WwwFormUrlDecoder {
            pub fn r#create_www_form_url_decoder<'a, __0: Into<winrt::StringParam<'a>>>(
                query: __0,
            ) -> winrt::Result<WwwFormUrlDecoder> {
                winrt::factory::<WwwFormUrlDecoder, IWwwFormUrlDecoderRuntimeClassFactory>()?
                    .r#create_www_form_url_decoder(query)
            }
            pub fn r#get_first_value_by_name<'a, __0: Into<winrt::StringParam<'a>>>(
                &self,
                name: __0,
            ) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderRuntimeClass)))
                        .r#get_first_value_by_name)(
                        self.ptr.get(), name.into().abi(), &mut __ok
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::ClassType for WwwFormUrlDecoder {}
        impl winrt::QueryType for WwwFormUrlDecoder {
            fn type_guid() -> &'static winrt::Guid {
                <IWwwFormUrlDecoderRuntimeClass as winrt::QueryType>::type_guid()
            }
        }
        impl winrt::TypeName for WwwFormUrlDecoder {
            fn type_name() -> &'static str {
                "Windows.Foundation.WwwFormUrlDecoder"
            }
        }
        impl winrt::RuntimeType for WwwFormUrlDecoder {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, WwwFormUrlDecoder>> for WwwFormUrlDecoder {
            fn into(self) -> winrt::Param<'a, WwwFormUrlDecoder> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, WwwFormUrlDecoder>> for &'a WwwFormUrlDecoder {
            fn into(self) -> winrt::Param<'a, WwwFormUrlDecoder> {
                winrt::Param::Ref(self)
            }
        }
        impl From<WwwFormUrlDecoder> for IWwwFormUrlDecoderRuntimeClass {
            fn from(value: WwwFormUrlDecoder) -> IWwwFormUrlDecoderRuntimeClass {
                unsafe { std::mem::transmute(value) }
            }
        }
        impl From<&WwwFormUrlDecoder> for IWwwFormUrlDecoderRuntimeClass {
            fn from(value: &WwwFormUrlDecoder) -> IWwwFormUrlDecoderRuntimeClass {
                unsafe { std::mem::transmute(value.clone()) }
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass>> for WwwFormUrlDecoder {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass>> for &'a WwwFormUrlDecoder {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderRuntimeClass> {
                winrt::Param::Value(self.into())
            }
        }
        #[repr(C)]
        #[derive(Default, Clone)]
        pub struct WwwFormUrlDecoderEntry {
            ptr: winrt::ComPtr,
        }
        impl WwwFormUrlDecoderEntry {
            pub fn r#name(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderEntry))).r#name)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
            pub fn r#value(&self) -> winrt::Result<winrt::String> {
                unsafe {
                    let mut __ok = std::mem::zeroed();
                    ((*(*(self.ptr.get() as *const *const abi_IWwwFormUrlDecoderEntry))).r#value)(
                        self.ptr.get(),
                        &mut __ok,
                    )
                    .ok_or(std::mem::transmute_copy(&__ok))
                }
            }
        }
        impl winrt::ClassType for WwwFormUrlDecoderEntry {}
        impl winrt::QueryType for WwwFormUrlDecoderEntry {
            fn type_guid() -> &'static winrt::Guid {
                <IWwwFormUrlDecoderEntry as winrt::QueryType>::type_guid()
            }
        }
        impl winrt::TypeName for WwwFormUrlDecoderEntry {
            fn type_name() -> &'static str {
                "Windows.Foundation.WwwFormUrlDecoderEntry"
            }
        }
        impl winrt::RuntimeType for WwwFormUrlDecoderEntry {
            type Abi = winrt::RawPtr;
            fn abi(&self) -> Self::Abi {
                self.ptr.get()
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set()
            }
        }
        impl<'a> Into<winrt::Param<'a, WwwFormUrlDecoderEntry>> for WwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, WwwFormUrlDecoderEntry> {
                winrt::Param::Value(self)
            }
        }
        impl<'a> Into<winrt::Param<'a, WwwFormUrlDecoderEntry>> for &'a WwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, WwwFormUrlDecoderEntry> {
                winrt::Param::Ref(self)
            }
        }
        impl From<WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
            fn from(value: WwwFormUrlDecoderEntry) -> IWwwFormUrlDecoderEntry {
                unsafe { std::mem::transmute(value) }
            }
        }
        impl From<&WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
            fn from(value: &WwwFormUrlDecoderEntry) -> IWwwFormUrlDecoderEntry {
                unsafe { std::mem::transmute(value.clone()) }
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderEntry>> for WwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderEntry> {
                winrt::Param::Value(self.into())
            }
        }
        impl<'a> Into<winrt::Param<'a, IWwwFormUrlDecoderEntry>> for &'a WwwFormUrlDecoderEntry {
            fn into(self) -> winrt::Param<'a, IWwwFormUrlDecoderEntry> {
                winrt::Param::Value(self.into())
            }
        }
    }
}

//use windows::foundation::collections::*;
use windows::foundation::*;
use winrt::*;

fn main() -> winrt::Result<()> {
    {
        let uri = &Uri::create_uri("http://kennykerr.ca")?;
        println!("domain: {}", uri.domain()?);

        let d: IUriRuntimeClass = uri.into();
        println!("domain: {}", d.domain()?);
        println!("port: {}", d.port()?);

        let s: IStringable = uri.into();
        let value = s.to_string()?;
        println!("stringable: {}", value);
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

    // //let o: Object = s.into();
    // //let tn = o.type_name()?;

    // unsafe {
    //     type vcall = extern "system" fn(winrt::RawPtr, *mut winrt::RawPtr) -> winrt::ErrorCode;
    //     let abi = s.abi() as *const *const vcall;
    //     // Or the preceding two lines in one:
    //     // let abi = s.abi() as *const *const extern "system" fn (winrt :: RawPtr, * mut winrt :: RawPtr,) -> winrt :: ErrorCode;
    //     let abi = (*abi).offset(6);

    //     let mut hstring = winrt::String::new();
    //     let hr = (*abi)(s.abi(), hstring.set_abi());
    //     println!("hr: {} string: {}", hr.0, hstring);

    //     // let mut hstring = std::ptr::null_mut();
    //     // let hr = (*abi)(s.abi(), &mut hstring);
    //     // let hstring: winrt::String = std::mem::transmute(hstring);
    //     // println!("hr: {} string: {}", hr.0, hstring);
    // }

    // println!("domain: {}", uri.domain()?);

    // let _v = uri.query_parsed()?;

    // call(uri)?;
    // call(uri.clone())?;
    // //call(d);
    // call(&s)?;
    // call(Uri::create_uri("http://kennykerr.ca")?)?;

    // let o: winrt::Object = s.query();
    // let s: IStringable = o.query();
    // println!("o: {}", s.to_string()?);

    // unsafe {
    //     let v: IVector<i32> = std::mem::zeroed();
    //     if false {
    //         v.at(123);
    //     }
    // }

    Ok(())
}

fn call<'a, T: Into<winrt::Param<'a, IStringable>>>(_s: T) -> winrt::Result<()> {
    Ok(())
}
