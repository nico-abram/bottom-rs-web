use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub mod bottom;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Get document and body
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Encode
    let encode_input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    encode_input.set_type("text");
    let encode_button = document
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    encode_button.set_text_content(Some("Encode"));
    let encode_output = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    encode_output.set_type("text");

    body.append_child(&encode_input)?;
    body.append_child(&encode_output)?;

    let on_click = gloo_events::EventListener::new(&encode_button, "click", move |_event| {
        encode_output.set_value(&bottom::encode_string(&encode_input.value()));
    });

    on_click.forget();

    body.append_child(&encode_button)?;

    let br = document.create_element("br")?;
    body.append_child(&br)?;

    // Decode
    let decode_input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    decode_input.set_type("text");
    let decode_button = document
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    decode_button.set_text_content(Some("Decode"));
    let decode_output = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    decode_output.set_type("text");

    body.append_child(&decode_input)?;
    body.append_child(&decode_output)?;

    let on_click = gloo_events::EventListener::new(&decode_button, "click", move |_event| {
        decode_output.set_value(
            &bottom::decode_string(&decode_input.value()).unwrap_or("ERROR DECODING".into()),
        );
    });

    on_click.forget();

    body.append_child(&decode_button)?;

    Ok(())
}
