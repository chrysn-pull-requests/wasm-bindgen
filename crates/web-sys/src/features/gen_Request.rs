#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Request , typescript_type = "Request")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Request` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub type Request;
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = method)]
    #[doc = "Getter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn method(this: &Request) -> String;
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = url)]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn url(this: &Request) -> String;
    #[cfg(feature = "Headers")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = headers)]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `Request`*"]
    pub fn headers(this: &Request) -> Headers;
    #[cfg(feature = "RequestDestination")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = destination)]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestDestination`*"]
    pub fn destination(this: &Request) -> RequestDestination;
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = referrer)]
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn referrer(this: &Request) -> String;
    #[cfg(feature = "ReferrerPolicy")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = referrerPolicy)]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `Request`*"]
    pub fn referrer_policy(this: &Request) -> ReferrerPolicy;
    #[cfg(feature = "RequestMode")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = mode)]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestMode`*"]
    pub fn mode(this: &Request) -> RequestMode;
    #[cfg(feature = "RequestCredentials")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = credentials)]
    #[doc = "Getter for the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestCredentials`*"]
    pub fn credentials(this: &Request) -> RequestCredentials;
    #[cfg(feature = "RequestCache")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = cache)]
    #[doc = "Getter for the `cache` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestCache`*"]
    pub fn cache(this: &Request) -> RequestCache;
    #[cfg(feature = "RequestRedirect")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = redirect)]
    #[doc = "Getter for the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*"]
    pub fn redirect(this: &Request) -> RequestRedirect;
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = integrity)]
    #[doc = "Getter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn integrity(this: &Request) -> String;
    #[cfg(feature = "AbortSignal")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = signal)]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `Request`*"]
    pub fn signal(this: &Request) -> AbortSignal;
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = bodyUsed)]
    #[doc = "Getter for the `bodyUsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn body_used(this: &Request) -> bool;
    #[cfg(feature = "ReadableStream")]
    # [wasm_bindgen (structural , method , getter , js_class = "Request" , js_name = body)]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Request`*"]
    pub fn body(this: &Request) -> Option<ReadableStream>;
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_request(input: &Request) -> Result<Request, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_str(input: &str) -> Result<Request, JsValue>;
    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;
    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = clone)]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn clone(this: &Request) -> Result<Request, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = arrayBuffer)]
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: ArrayBuffer`. It can be converted like `let result: :: js_sys :: ArrayBuffer = result?.await.into();`."]
    pub fn array_buffer(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = blob)]
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Blob`. It can be converted like `let result: Blob = result?.await.into();`."]
    pub fn blob(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = formData)]
    #[doc = "The `formData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `FormData`. It can be converted like `let result: FormData = result?.await.into();`."]
    pub fn form_data(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = json)]
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Object`. It can be converted like `let result: :: js_sys :: Object = result?.await.into();`."]
    pub fn json(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Request" , js_name = text)]
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `String`. It can be converted like `let result: String = result?.await.into();`."]
    pub fn text(this: &Request) -> Result<::js_sys::Promise, JsValue>;
}
