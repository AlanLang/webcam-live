use tracing::info;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStream};
use gloo_utils::format::JsValueSerdeExt;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream {
            el
        }
    }

    pub fn log(&self) {
        info!("VideoStream::log");
    }

    pub async fn set_video_src(&self, video_constraints: &serde_json::Value){
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no `navigator.media_devices` exists");
        info!("devices: {:?}", devices);
        web_sys::console::log_1(&devices);

        let mut constraints = web_sys::MediaStreamConstraints::new();
        constraints.video(&JsValue::from_serde(video_constraints).unwrap());
        constraints.audio(&JsValue::from_serde(&false).unwrap());

        let media = JsFuture::from(devices.get_user_media_with_constraints(&constraints).unwrap()).await.unwrap();
        let media_stream = media.unchecked_into::<MediaStream>();
        self.el.set_src_object(Some(&media_stream));
    }
}