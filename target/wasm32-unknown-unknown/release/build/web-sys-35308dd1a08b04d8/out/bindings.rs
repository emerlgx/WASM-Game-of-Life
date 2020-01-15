#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MessageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MessageEvent {
    obj: Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MessageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MessageEvent {
        fn describe() {
            JsValue::describe();
        }
    }
    impl core::ops::Deref for MessageEvent {
        type Target = Object;
        #[inline]
        fn deref(&self) -> &Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MessageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MessageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MessageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MessageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MessageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MessageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MessageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MessageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MessageEvent {
        #[inline]
        fn from(obj: JsValue) -> MessageEvent {
            MessageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MessageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MessageEvent> for MessageEvent {
        #[inline]
        fn as_ref(&self) -> &MessageEvent {
            self
        }
    }
    impl From<MessageEvent> for JsValue {
        #[inline]
        fn from(obj: MessageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MessageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MessageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MessageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MessageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MessageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MessageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MessageEvent> for Object {
    #[inline]
    fn from(obj: MessageEvent) -> Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Object> for MessageEvent {
    #[inline]
    fn as_ref(&self) -> &Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MessageEvent as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MessageEvent, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MessageEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event(&self, type_: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_message_event_MessageEvent(self_, type_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles(&self, type_: &str, bubbles: bool) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                __widl_f_init_message_event_with_bubbles_MessageEvent(self_, type_, bubbles)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
                    self_, type_, bubbles, cancelable,
                )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
                    self_, type_, bubbles, cancelable, data,
                )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::JsValue {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_MessageEvent(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_MessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_event_id_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `lastEventId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn last_event_id(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_event_id_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_event_id_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_event_id_MessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `source` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn source(&self) -> Option<::js_sys::Object> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_MessageEvent(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ports_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MessageEvent {
    #[allow(bad_style)]
    #[doc = "The `ports` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn ports(&self) -> ::js_sys::Array {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ports_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ports_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ports_MessageEvent(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Worker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Worker {
    obj: Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Worker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Worker {
        fn describe() {
            JsValue::describe();
        }
    }
    impl core::ops::Deref for Worker {
        type Target = Object;
        #[inline]
        fn deref(&self) -> &Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Worker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Worker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Worker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Worker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Worker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Worker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Worker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Worker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Worker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Worker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Worker {
        #[inline]
        fn from(obj: JsValue) -> Worker {
            Worker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Worker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Worker> for Worker {
        #[inline]
        fn as_ref(&self) -> &Worker {
            self
        }
    }
    impl From<Worker> for JsValue {
        #[inline]
        fn from(obj: Worker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Worker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Worker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Worker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Worker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Worker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Worker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Worker> for Object {
    #[inline]
    fn from(obj: Worker) -> Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Object> for Worker {
    #[inline]
    fn as_ref(&self) -> &Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Worker as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn new(script_url: &str) -> Result<Worker, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Worker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Worker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                __widl_f_new_Worker(script_url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Worker as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn post_message(
        &self,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                __widl_f_post_message_Worker(self_, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_with_transfer_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Worker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn post_message_with_transfer(
        &self,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_with_transfer_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transfer: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_with_transfer_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transfer: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            drop(transfer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                let transfer =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        transfer,
                    );
                __widl_f_post_message_with_transfer_Worker(self_, message, transfer)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_terminate_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `terminate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn terminate(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_terminate_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_terminate_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_terminate_Worker(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_Worker(self_, onmessage)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessageerror_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onmessageerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_Worker(self_, onmessageerror)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_Worker(self_, onerror)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_14bf07b037243da7: [u8; 2270usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b".\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58\"}\xA8\x08\0\0\0\0\x18\0\0\x02\x0CMessageEvent\x1E__widl_instanceof_MessageEvent\0\0\0\0\x19__widl_f_new_MessageEvent\x01\0\0\x01\x0CMessageEvent\0\x01\x01\x05type_\x03new\0\0\0(__widl_f_init_message_event_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x02\x05self_\x05type_\x10initMessageEvent\0\0\05__widl_f_init_message_event_with_bubbles_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x03\x05self_\x05type_\x07bubbles\x10initMessageEvent\0\0\0D__widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x04\x05self_\x05type_\x07bubbles\ncancelable\x10initMessageEvent\0\0\0M__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x05\x05self_\x05type_\x07bubbles\ncancelable\x04data\x10initMessageEvent\0\0\0X__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x06\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\x10initMessageEvent\0\0\0j__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x07\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x10initMessageEvent\0\0\0\x1A__widl_f_data_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\x1C__widl_f_origin_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0#__widl_f_last_event_id_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x0BlastEventId\x01\x01\x05self_\x0BlastEventId\0\0\0\x1C__widl_f_source_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x06source\x01\x01\x05self_\x06source\0\0\0\x1B__widl_f_ports_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x05ports\x01\x01\x05self_\x05ports\0\0\x02\x06Worker\x18__widl_instanceof_Worker\0\0\0\0\x13__widl_f_new_Worker\x01\0\0\x01\x06Worker\0\x01\x01\nscript_url\x03new\0\0\0\x1C__widl_f_post_message_Worker\x01\0\0\x01\x06Worker\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\0*__widl_f_post_message_with_transfer_Worker\x01\0\0\x01\x06Worker\x01\0\0\x01\x03\x05self_\x07message\x08transfer\x0BpostMessage\0\0\0\x19__widl_f_terminate_Worker\0\0\0\x01\x06Worker\x01\0\0\x01\x01\x05self_\tterminate\0\0\0\x19__widl_f_onmessage_Worker\0\0\0\x01\x06Worker\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0\x1D__widl_f_set_onmessage_Worker\0\0\0\x01\x06Worker\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\x1E__widl_f_onmessageerror_Worker\0\0\0\x01\x06Worker\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\0\"__widl_f_set_onmessageerror_Worker\0\0\0\x01\x06Worker\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0\x17__widl_f_onerror_Worker\0\0\0\x01\x06Worker\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1B__widl_f_set_onerror_Worker\0\0\0\x01\x06Worker\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x18web-sys-cdb8795b9a05e44c\0"
};
pub mod console {
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_assert_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data(condition: bool, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_assert_with_condition_and_data_(condition, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <bool as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_0(condition: bool) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_0_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_0_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                __widl_f_assert_with_condition_and_data_0_(condition)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_1(condition: bool, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_1_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_1_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_assert_with_condition_and_data_1_(condition, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_2(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_2_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_2_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_assert_with_condition_and_data_2_(condition, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_3(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_3_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_3_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_assert_with_condition_and_data_3_(condition, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_4(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_4_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_4_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_assert_with_condition_and_data_4_(
                    condition, data_1, data_2, data_3, data_4,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_5(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_5_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_5_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_assert_with_condition_and_data_5_(
                    condition, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_6(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_6_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_6_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_assert_with_condition_and_data_6_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_7(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_7_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_7_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_assert_with_condition_and_data_7_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_clear_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.clear()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/clear)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn clear() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_clear_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_reset_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_reset_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_debug_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_debug_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_debug_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_debug_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_debug_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_debug_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_debug_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_debug_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_debug_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dir_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dir_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dir_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dir_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dir_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dir_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dir_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dir_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dir_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dirxml_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dirxml_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dirxml_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dirxml_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dirxml_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dirxml_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dirxml_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dirxml_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dirxml_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_error_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_error_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_error_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_error_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_error_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_error_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_error_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_error_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_error_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_exception_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_exception_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_exception_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_exception_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_exception_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_exception_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_exception_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_exception_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_exception_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_collapsed_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_collapsed_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_collapsed_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_collapsed_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_collapsed_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_collapsed_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_collapsed_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_collapsed_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_collapsed_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_info_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_info_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_info_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_info_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_info_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_info_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_info_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_info_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_info_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_log_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_log_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_log_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_log_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_log_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_log_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_log_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_log_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_log_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_end_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_end_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_end_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_end_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_end_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_end_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_end_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_end_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_end_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_table_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_table_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_table_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_table_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_table_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_table_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_table_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_table_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_table_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_end_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_log_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data(label: &str, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_time_log_with_label_and_data_(label, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_0(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_0_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_0_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_log_with_label_and_data_0_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_1(label: &str, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_1_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_1_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_time_log_with_label_and_data_1_(label, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_2(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_2_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_2_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_time_log_with_label_and_data_2_(label, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_3(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_3_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_3_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_time_log_with_label_and_data_3_(label, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_4(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_4_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_4_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_time_log_with_label_and_data_4_(label, data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_5(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_5_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_5_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_time_log_with_label_and_data_5_(
                    label, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_6(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_6_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_6_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_time_log_with_label_and_data_6_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_7(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_7_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_7_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_time_log_with_label_and_data_7_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_stamp_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_with_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp_with_data(data: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_with_data_(
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_with_data_(
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_time_stamp_with_data_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_trace_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_trace_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_trace_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_trace_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_trace_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_trace_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_trace_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_trace_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_trace_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_warn_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_warn_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_warn_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_warn_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_warn_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_warn_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_warn_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_warn_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_warn_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[allow(non_upper_case_globals)]
    #[cfg(target_arch = "wasm32")]
    #[link_section = "__wasm_bindgen_unstable"]
    #[doc(hidden)]
    #[allow(clippy::all)]
    pub static __WASM_BINDGEN_GENERATED_c41155db8043fe1b: [u8; 10756usize] = {
        static _INCLUDED_FILES: &[&str] = &[];
        * b".\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58\"}\xCE)\0\0\0\0\x9E\x01\0\x01\x07console\0\x10__widl_f_assert_\0\0\0\0\x01\0\x06assert\0\x01\x07console\0(__widl_f_assert_with_condition_and_data_\0\x01\0\0\x01\x02\tcondition\x04data\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_0_\0\0\0\0\x01\x01\tcondition\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_1_\0\0\0\0\x01\x02\tcondition\x06data_1\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_2_\0\0\0\0\x01\x03\tcondition\x06data_1\x06data_2\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_3_\0\0\0\0\x01\x04\tcondition\x06data_1\x06data_2\x06data_3\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_4_\0\0\0\0\x01\x05\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_5_\0\0\0\0\x01\x06\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_6_\0\0\0\0\x01\x07\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_7_\0\0\0\0\x01\x08\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06assert\0\x01\x07console\0\x0F__widl_f_clear_\0\0\0\0\x01\0\x05clear\0\x01\x07console\0\x0F__widl_f_count_\0\0\0\0\x01\0\x05count\0\x01\x07console\0\x1A__widl_f_count_with_label_\0\0\0\0\x01\x01\x05label\x05count\0\x01\x07console\0\x15__widl_f_count_reset_\0\0\0\0\x01\0\ncountReset\0\x01\x07console\0 __widl_f_count_reset_with_label_\0\0\0\0\x01\x01\x05label\ncountReset\0\x01\x07console\0\x0F__widl_f_debug_\0\x01\0\0\x01\x01\x04data\x05debug\0\x01\x07console\0\x11__widl_f_debug_0_\0\0\0\0\x01\0\x05debug\0\x01\x07console\0\x11__widl_f_debug_1_\0\0\0\0\x01\x01\x06data_1\x05debug\0\x01\x07console\0\x11__widl_f_debug_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05debug\0\x01\x07console\0\x11__widl_f_debug_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05debug\0\x01\x07console\0\x11__widl_f_debug_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05debug\0\x01\x07console\0\x11__widl_f_debug_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05debug\0\x01\x07console\0\x11__widl_f_debug_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05debug\0\x01\x07console\0\x11__widl_f_debug_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05debug\0\x01\x07console\0\r__widl_f_dir_\0\x01\0\0\x01\x01\x04data\x03dir\0\x01\x07console\0\x0F__widl_f_dir_0_\0\0\0\0\x01\0\x03dir\0\x01\x07console\0\x0F__widl_f_dir_1_\0\0\0\0\x01\x01\x06data_1\x03dir\0\x01\x07console\0\x0F__widl_f_dir_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03dir\0\x01\x07console\0\x0F__widl_f_dir_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03dir\0\x01\x07console\0\x0F__widl_f_dir_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03dir\0\x01\x07console\0\x0F__widl_f_dir_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03dir\0\x01\x07console\0\x0F__widl_f_dir_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03dir\0\x01\x07console\0\x0F__widl_f_dir_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03dir\0\x01\x07console\0\x10__widl_f_dirxml_\0\x01\0\0\x01\x01\x04data\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_0_\0\0\0\0\x01\0\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_1_\0\0\0\0\x01\x01\x06data_1\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06dirxml\0\x01\x07console\0\x0F__widl_f_error_\0\x01\0\0\x01\x01\x04data\x05error\0\x01\x07console\0\x11__widl_f_error_0_\0\0\0\0\x01\0\x05error\0\x01\x07console\0\x11__widl_f_error_1_\0\0\0\0\x01\x01\x06data_1\x05error\0\x01\x07console\0\x11__widl_f_error_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05error\0\x01\x07console\0\x11__widl_f_error_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05error\0\x01\x07console\0\x11__widl_f_error_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05error\0\x01\x07console\0\x11__widl_f_error_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05error\0\x01\x07console\0\x11__widl_f_error_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05error\0\x01\x07console\0\x11__widl_f_error_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05error\0\x01\x07console\0\x13__widl_f_exception_\0\x01\0\0\x01\x01\x04data\texception\0\x01\x07console\0\x15__widl_f_exception_0_\0\0\0\0\x01\0\texception\0\x01\x07console\0\x15__widl_f_exception_1_\0\0\0\0\x01\x01\x06data_1\texception\0\x01\x07console\0\x15__widl_f_exception_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\texception\0\x01\x07console\0\x15__widl_f_exception_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\texception\0\x01\x07console\0\x15__widl_f_exception_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\texception\0\x01\x07console\0\x15__widl_f_exception_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\texception\0\x01\x07console\0\x15__widl_f_exception_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\texception\0\x01\x07console\0\x15__widl_f_exception_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\texception\0\x01\x07console\0\x0F__widl_f_group_\0\x01\0\0\x01\x01\x04data\x05group\0\x01\x07console\0\x11__widl_f_group_0_\0\0\0\0\x01\0\x05group\0\x01\x07console\0\x11__widl_f_group_1_\0\0\0\0\x01\x01\x06data_1\x05group\0\x01\x07console\0\x11__widl_f_group_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05group\0\x01\x07console\0\x11__widl_f_group_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05group\0\x01\x07console\0\x11__widl_f_group_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05group\0\x01\x07console\0\x11__widl_f_group_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05group\0\x01\x07console\0\x11__widl_f_group_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05group\0\x01\x07console\0\x11__widl_f_group_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05group\0\x01\x07console\0\x19__widl_f_group_collapsed_\0\x01\0\0\x01\x01\x04data\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_0_\0\0\0\0\x01\0\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_1_\0\0\0\0\x01\x01\x06data_1\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x0EgroupCollapsed\0\x01\x07console\0\x13__widl_f_group_end_\0\0\0\0\x01\0\x08groupEnd\0\x01\x07console\0\x0E__widl_f_info_\0\x01\0\0\x01\x01\x04data\x04info\0\x01\x07console\0\x10__widl_f_info_0_\0\0\0\0\x01\0\x04info\0\x01\x07console\0\x10__widl_f_info_1_\0\0\0\0\x01\x01\x06data_1\x04info\0\x01\x07console\0\x10__widl_f_info_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04info\0\x01\x07console\0\x10__widl_f_info_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04info\0\x01\x07console\0\x10__widl_f_info_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04info\0\x01\x07console\0\x10__widl_f_info_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04info\0\x01\x07console\0\x10__widl_f_info_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04info\0\x01\x07console\0\x10__widl_f_info_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04info\0\x01\x07console\0\r__widl_f_log_\0\x01\0\0\x01\x01\x04data\x03log\0\x01\x07console\0\x0F__widl_f_log_0_\0\0\0\0\x01\0\x03log\0\x01\x07console\0\x0F__widl_f_log_1_\0\0\0\0\x01\x01\x06data_1\x03log\0\x01\x07console\0\x0F__widl_f_log_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03log\0\x01\x07console\0\x0F__widl_f_log_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03log\0\x01\x07console\0\x0F__widl_f_log_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03log\0\x01\x07console\0\x0F__widl_f_log_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03log\0\x01\x07console\0\x0F__widl_f_log_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03log\0\x01\x07console\0\x0F__widl_f_log_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03log\0\x01\x07console\0\x11__widl_f_profile_\0\x01\0\0\x01\x01\x04data\x07profile\0\x01\x07console\0\x13__widl_f_profile_0_\0\0\0\0\x01\0\x07profile\0\x01\x07console\0\x13__widl_f_profile_1_\0\0\0\0\x01\x01\x06data_1\x07profile\0\x01\x07console\0\x13__widl_f_profile_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x07profile\0\x01\x07console\0\x13__widl_f_profile_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x07profile\0\x01\x07console\0\x13__widl_f_profile_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x07profile\0\x01\x07console\0\x13__widl_f_profile_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07profile\0\x01\x07console\0\x13__widl_f_profile_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07profile\0\x01\x07console\0\x13__widl_f_profile_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07profile\0\x01\x07console\0\x15__widl_f_profile_end_\0\x01\0\0\x01\x01\x04data\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_0_\0\0\0\0\x01\0\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_1_\0\0\0\0\x01\x01\x06data_1\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\nprofileEnd\0\x01\x07console\0\x0F__widl_f_table_\0\x01\0\0\x01\x01\x04data\x05table\0\x01\x07console\0\x11__widl_f_table_0_\0\0\0\0\x01\0\x05table\0\x01\x07console\0\x11__widl_f_table_1_\0\0\0\0\x01\x01\x06data_1\x05table\0\x01\x07console\0\x11__widl_f_table_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05table\0\x01\x07console\0\x11__widl_f_table_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05table\0\x01\x07console\0\x11__widl_f_table_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05table\0\x01\x07console\0\x11__widl_f_table_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05table\0\x01\x07console\0\x11__widl_f_table_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05table\0\x01\x07console\0\x11__widl_f_table_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05table\0\x01\x07console\0\x0E__widl_f_time_\0\0\0\0\x01\0\x04time\0\x01\x07console\0\x19__widl_f_time_with_label_\0\0\0\0\x01\x01\x05label\x04time\0\x01\x07console\0\x12__widl_f_time_end_\0\0\0\0\x01\0\x07timeEnd\0\x01\x07console\0\x1D__widl_f_time_end_with_label_\0\0\0\0\x01\x01\x05label\x07timeEnd\0\x01\x07console\0\x12__widl_f_time_log_\0\0\0\0\x01\0\x07timeLog\0\x01\x07console\0&__widl_f_time_log_with_label_and_data_\0\x01\0\0\x01\x02\x05label\x04data\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_0_\0\0\0\0\x01\x01\x05label\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_1_\0\0\0\0\x01\x02\x05label\x06data_1\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_2_\0\0\0\0\x01\x03\x05label\x06data_1\x06data_2\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_3_\0\0\0\0\x01\x04\x05label\x06data_1\x06data_2\x06data_3\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_4_\0\0\0\0\x01\x05\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_5_\0\0\0\0\x01\x06\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_6_\0\0\0\0\x01\x07\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_7_\0\0\0\0\x01\x08\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07timeLog\0\x01\x07console\0\x14__widl_f_time_stamp_\0\0\0\0\x01\0\ttimeStamp\0\x01\x07console\0\x1E__widl_f_time_stamp_with_data_\0\0\0\0\x01\x01\x04data\ttimeStamp\0\x01\x07console\0\x0F__widl_f_trace_\0\x01\0\0\x01\x01\x04data\x05trace\0\x01\x07console\0\x11__widl_f_trace_0_\0\0\0\0\x01\0\x05trace\0\x01\x07console\0\x11__widl_f_trace_1_\0\0\0\0\x01\x01\x06data_1\x05trace\0\x01\x07console\0\x11__widl_f_trace_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05trace\0\x01\x07console\0\x11__widl_f_trace_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05trace\0\x01\x07console\0\x11__widl_f_trace_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05trace\0\x01\x07console\0\x11__widl_f_trace_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05trace\0\x01\x07console\0\x11__widl_f_trace_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05trace\0\x01\x07console\0\x11__widl_f_trace_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05trace\0\x01\x07console\0\x0E__widl_f_warn_\0\x01\0\0\x01\x01\x04data\x04warn\0\x01\x07console\0\x10__widl_f_warn_0_\0\0\0\0\x01\0\x04warn\0\x01\x07console\0\x10__widl_f_warn_1_\0\0\0\0\x01\x01\x06data_1\x04warn\0\x01\x07console\0\x10__widl_f_warn_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04warn\0\x01\x07console\0\x10__widl_f_warn_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04warn\0\x01\x07console\0\x10__widl_f_warn_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04warn\0\x01\x07console\0\x10__widl_f_warn_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04warn\0\x01\x07console\0\x10__widl_f_warn_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04warn\0\x01\x07console\0\x10__widl_f_warn_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04warn\0\0\0\0\x18web-sys-cdb8795b9a05e44c\0"
    };
}
