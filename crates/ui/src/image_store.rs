use std::collections::HashSet;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;
use image::{Frame, RgbaImage};
use lru::LruCache;

pub struct ImageStore {
    memory_cache: LruCache<String, Arc<RenderImage>>,
    disk_cache_dir: PathBuf,
    loading: HashSet<String>,
    http_client: reqwest::Client,
    runtime_handle: tokio::runtime::Handle,
}

impl ImageStore {
    pub fn new(cx: &mut App, runtime_handle: tokio::runtime::Handle) -> Entity<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("crabfin")
            .join("images");

        std::fs::create_dir_all(&cache_dir).ok();

        cx.new(|_| Self {
            memory_cache: LruCache::new(NonZeroUsize::new(50).unwrap()),
            disk_cache_dir: cache_dir,
            loading: HashSet::new(),
            http_client: reqwest::Client::new(),
            runtime_handle,
        })
    }

    pub fn get_image(
        &mut self,
        url: String,
        cache_key: String,
        cx: &mut Context<Self>,
    ) -> Option<Arc<RenderImage>> {
        if let Some(image) = self.memory_cache.get(&cache_key) {
            return Some(image.clone());
        }

        if !self.loading.contains(&cache_key) {
            self.load_image(url, cache_key, cx);
        }

        None
    }

    fn load_image(&mut self, url: String, cache_key: String, cx: &mut Context<Self>) {
        self.loading.insert(cache_key.clone());

        let client = self.http_client.clone();
        let disk_dir = self.disk_cache_dir.clone();
        let key = cache_key.clone();
        let handle = self.runtime_handle.clone();

        cx.spawn(move |model: WeakEntity<ImageStore>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            let key_clone = key.clone();
            async move {
                let result = handle
                    .spawn(async move {
                        // Try load from disk
                        let disk_path = disk_dir.join(&key_clone);
                        if disk_path.exists() {
                            if let Ok(bytes) = std::fs::read(&disk_path) {
                                return Some(bytes);
                            }
                        }

                        // Fetch from network
                        if let Ok(res) = client.get(&url).send().await {
                            if let Ok(bytes) = res.bytes().await {
                                let bytes_vec = bytes.to_vec();
                                // Save to disk
                                let _ = std::fs::write(&disk_path, &bytes_vec);
                                return Some(bytes_vec);
                            }
                        }

                        None
                    })
                    .await
                    .unwrap_or(None);

                if let Some(bytes) = result {
                    // Decode off main thread
                    let task = cx.background_executor().spawn(async move {
                        if let Ok(image) = image::load_from_memory(&bytes) {
                            let mut rgba = image.to_rgba8();
                            rgb_to_bgr(&mut rgba);
                            Some(rgba)
                        } else {
                            None
                        }
                    });

                    if let Some(rgba) = task.await {
                        model
                            .update(&mut cx, |store, cx| {
                                let render_image = Arc::new(RenderImage::new(vec![Frame::new(rgba)]));
                                store.memory_cache.put(key.clone(), render_image);
                                store.loading.remove(&key);
                                cx.notify();
                            })
                            .ok();
                    } else {
                        model
                            .update(&mut cx, |store, _| {
                                store.loading.remove(&key);
                            })
                            .ok();
                    }
                } else {
                    model
                        .update(&mut cx, |store, _| {
                            store.loading.remove(&key);
                        })
                        .ok();
                }
            }
        })
        .detach();
    }
}

fn rgb_to_bgr(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        pixel.0 = [b, g, r, a];
    }
}
