use serde_json::json;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use webcam_live::VideoStream;

fn main() {
    println!("Hello, world!");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| view!(ctx, Video()));
}

#[component]
fn Video<G: Html>(ctx: BoundedScope) -> View<G> {
    let video_ref: &NodeRef<G> = create_node_ref(ctx);

    on_mount(ctx, move || {
        spawn_local_scoped(ctx, async move {
            let el = video_ref.get::<DomNode>().unchecked_into();
            let video_stream = VideoStream::new(el);
            video_stream
                .set_video_src(&json!({
                    "audio": false,
                    "video": {
                        "width": 640,
                        "height": 480,
                        "facingMode": "user"
                    }
                }))
                .await;
        });
    });

    view! (ctx,
        div {
            video(
                ref=video_ref,
                class="border border-gray-400",
                width="640",
                height="480",
                autoplay=true
            )
        }
    )
}
