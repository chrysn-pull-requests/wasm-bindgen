#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = USB , typescript_type = "USB")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Usb` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Usb;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "USB" , js_name = onconnect)]
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onconnect(this: &Usb) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "USB" , js_name = onconnect)]
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onconnect(this: &Usb, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "USB" , js_name = ondisconnect)]
    #[doc = "Getter for the `ondisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/ondisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ondisconnect(this: &Usb) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "USB" , js_name = ondisconnect)]
    #[doc = "Setter for the `ondisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/ondisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ondisconnect(this: &Usb, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "USB" , js_name = getDevices)]
    #[doc = "The `getDevices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/getDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_devices(this: &Usb) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDeviceRequestOptions")]
    # [wasm_bindgen (method , structural , js_class = "USB" , js_name = requestDevice)]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USB/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`, `UsbDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `UsbDevice`. It can be converted like `let result: UsbDevice = result.await.into();`."]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device(this: &Usb, options: &UsbDeviceRequestOptions) -> ::js_sys::Promise;
}
