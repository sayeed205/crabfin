use crate::state::ImageStoreGlobal;
use gpui::*;
use jellyfin::client::{AuthenticatedClient, ImageParams, ImageType};

pub fn image_display(
    item_id: String,
    image_type: ImageType,
    tag: String,
    width: u32,
    height: Option<u32>,
    client: AuthenticatedClient,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let store = cx.global::<ImageStoreGlobal>().0.clone();

    // 1. Full Image Params
    let full_params = ImageParams {
        tag: tag.clone(),
        fill_width: Some(width),
        fill_height: height,
        quality: Some(90),
        ..Default::default()
    };
    let full_url = client.image_url(&item_id, image_type, &full_params);
    let full_key = format!("{}-{}-{}-{}", item_id, image_type, tag, width);

    // 2. Blur Image Params (small)
    let blur_width = 32;
    let blur_params = ImageParams {
        tag: tag.clone(),
        fill_width: Some(blur_width),
        // Aspect ratio preserved if height is None, otherwise scale
        fill_height: height.map(|h| (h * blur_width) / width),
        quality: Some(50),
        ..Default::default()
    };
    let blur_url = client.image_url(&item_id, image_type, &blur_params);
    let blur_key = format!("{}-{}-{}-{}", item_id, image_type, tag, blur_width);

    // Query Store
    let full_image = store.update(cx, |s, cx| s.get_image(full_url, full_key, cx));
    let blur_image = store.update(cx, |s, cx| s.get_image(blur_url, blur_key, cx));

    div()
        .size_full()
        .bg(rgb(0x181825)) // Placeholder background
        .child(if let Some(render_image) = full_image {
            img(render_image).into_any_element()
        } else if let Some(render_image) = blur_image {
            img(render_image).into_any_element()
        } else {
            div().into_any_element()
        })
}
