use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Media item representing any playable content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub name: String,
    pub media_type: MediaType,
    pub duration: Option<Duration>,
    pub file_path: Option<String>,
    pub thumbnail_url: Option<String>,
    pub metadata: MediaMetadata,
}

impl MediaItem {
    pub fn new(id: String, name: String, media_type: MediaType) -> Self {
        Self {
            id,
            name,
            media_type,
            duration: None,
            file_path: None,
            thumbnail_url: None,
            metadata: MediaMetadata::default(),
        }
    }
}

/// Types of media content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    Movie,
    TvShow,
    Episode,
    Music,
    Photo,
    Book,
    AudioBook,
}

/// Media metadata information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub description: Option<String>,
    pub year: Option<u32>,
    pub genres: Vec<String>,
    pub rating: Option<f32>,
    pub cast: Vec<String>,
    pub director: Option<String>,
    pub studio: Option<String>,
}

/// Library containing collections of media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub id: String,
    pub name: String,
    pub collections: Vec<Collection>,
}

impl Library {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            collections: Vec::new(),
        }
    }

    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.push(collection);
    }
}

/// Collection of related media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub collection_type: CollectionType,
    pub items: Vec<MediaItem>,
}

impl Collection {
    pub fn new(id: String, name: String, collection_type: CollectionType) -> Self {
        Self {
            id,
            name,
            collection_type,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: MediaItem) {
        self.items.push(item);
    }
}

/// Types of media collections
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectionType {
    Movies,
    TvShows,
    Music,
    Photos,
    Books,
    Mixed,
}

/// Playback state for media player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

/// Playback position and progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackProgress {
    pub position: Duration,
    pub duration: Duration,
    pub percentage: f32,
}

impl PlaybackProgress {
    pub fn new(position: Duration, duration: Duration) -> Self {
        let percentage = if duration.as_secs() > 0 {
            (position.as_secs_f32() / duration.as_secs_f32()) * 100.0
        } else {
            0.0
        };

        Self {
            position,
            duration,
            percentage,
        }
    }
}

/// User preferences and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: Theme,
    pub volume: f32,
    pub auto_play: bool,
    pub subtitle_language: Option<String>,
    pub audio_language: Option<String>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            volume: 1.0,
            auto_play: true,
            subtitle_language: None,
            audio_language: None,
        }
    }
}

/// Application theme options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub servers: Vec<crate::auth::ServerConfig>,
    pub current_user: Option<crate::auth::UserSession>,
    pub preferences: UserPreferences,
    pub cache_settings: CacheSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            current_user: None,
            preferences: UserPreferences::default(),
            cache_settings: CacheSettings::default(),
        }
    }
}

/// Cache configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
    pub max_size_mb: u64,
    pub cache_thumbnails: bool,
    pub cache_metadata: bool,
}

impl Default for CacheSettings {
    fn default() -> Self {
        Self {
            max_size_mb: 500,
            cache_thumbnails: true,
            cache_metadata: true,
        }
    }
}