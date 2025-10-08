//! Media models
//!
//! This module contains data structures for media items, libraries, and metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Media item type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    /// Movie
    Movie,
    /// TV Series
    Series,
    /// TV Episode
    Episode,
    /// Music Album
    Album,
    /// Music Track
    Audio,
    /// Photo
    Photo,
    /// Video
    Video,
    /// Collection/Folder
    Folder,
    /// Playlist
    Playlist,
}

/// Base media item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    /// Unique item identifier
    pub id: String,
    /// Item name/title
    pub name: String,
    /// Item type
    pub media_type: MediaType,
    /// Item overview/description
    pub overview: Option<String>,
    /// Release/air date
    pub premiere_date: Option<String>,
    /// Runtime in minutes
    pub runtime_ticks: Option<u64>,
    /// Community rating (0.0-10.0)
    pub community_rating: Option<f32>,
    /// Official rating (G, PG, R, etc.)
    pub official_rating: Option<String>,
    /// Production year
    pub production_year: Option<u32>,
    /// Genres
    pub genres: Vec<String>,
    /// Studios/Networks
    pub studios: Vec<String>,
    /// People (actors, directors, etc.)
    pub people: Vec<PersonInfo>,
    /// Image tags for different image types
    pub image_tags: HashMap<String, String>,
    /// Whether item has been played
    pub is_played: bool,
    /// Play count
    pub play_count: Option<u32>,
    /// User data
    pub user_data: Option<UserData>,
}

/// Person information (actor, director, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonInfo {
    /// Person name
    pub name: String,
    /// Person ID
    pub id: Option<String>,
    /// Role (Actor, Director, Writer, etc.)
    pub role: Option<String>,
    /// Character name (for actors)
    pub character: Option<String>,
    /// Person type
    pub person_type: String,
}

/// User-specific data for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    /// Whether item is favorite
    pub is_favorite: bool,
    /// User rating (0.0-10.0)
    pub rating: Option<f32>,
    /// Playback position in ticks
    pub playback_position_ticks: Option<u64>,
    /// Last played date
    pub last_played_date: Option<String>,
    /// Whether item is played
    pub played: bool,
    /// Play count
    pub play_count: u32,
}

/// Library information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// Library ID
    pub id: String,
    /// Library name
    pub name: String,
    /// Library type (movies, tvshows, music, etc.)
    pub collection_type: String,
    /// Library location paths
    pub locations: Vec<String>,
    /// Whether library is enabled
    pub is_enabled: bool,
    /// Item count in library
    pub item_count: Option<u32>,
}

/// Media stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStream {
    /// Stream index
    pub index: u32,
    /// Stream type (Video, Audio, Subtitle)
    pub stream_type: StreamType,
    /// Codec name
    pub codec: Option<String>,
    /// Language code
    pub language: Option<String>,
    /// Display title
    pub display_title: Option<String>,
    /// Whether stream is default
    pub is_default: bool,
    /// Whether stream is forced
    pub is_forced: bool,
    /// Whether stream is external
    pub is_external: bool,
}

/// Stream type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StreamType {
    /// Video stream
    Video,
    /// Audio stream
    Audio,
    /// Subtitle stream
    Subtitle,
}

/// Playback information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackInfo {
    /// Media sources available for playback
    pub media_sources: Vec<MediaSource>,
    /// Play session ID
    pub play_session_id: Option<String>,
    /// Error code if any
    pub error_code: Option<String>,
}

/// Media source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSource {
    /// Source ID
    pub id: String,
    /// Source path
    pub path: Option<String>,
    /// Source type (Default, Grouping, Placeholder)
    pub source_type: String,
    /// Container format
    pub container: Option<String>,
    /// File size in bytes
    pub size: Option<u64>,
    /// Bitrate
    pub bitrate: Option<u32>,
    /// Media streams
    pub media_streams: Vec<MediaStream>,
    /// Whether source supports direct play
    pub supports_direct_play: bool,
    /// Whether source supports direct stream
    pub supports_direct_stream: bool,
    /// Whether source supports transcoding
    pub supports_transcoding: bool,
}

impl MediaItem {
    /// Get runtime in a human-readable format
    pub fn runtime_formatted(&self) -> Option<String> {
        self.runtime_ticks.map(|ticks| {
            let minutes = ticks / 600_000_000; // Convert from ticks to minutes
            let hours = minutes / 60;
            let remaining_minutes = minutes % 60;

            if hours > 0 {
                format!("{}h {}m", hours, remaining_minutes)
            } else {
                format!("{}m", remaining_minutes)
            }
        })
    }

    /// Check if item has been watched/played
    pub fn is_watched(&self) -> bool {
        self.user_data
            .as_ref()
            .map(|data| data.played)
            .unwrap_or(self.is_played)
    }

    /// Get user rating if available
    pub fn user_rating(&self) -> Option<f32> {
        self.user_data.as_ref().and_then(|data| data.rating)
    }

    /// Check if item is marked as favorite
    pub fn is_favorite(&self) -> bool {
        self.user_data
            .as_ref()
            .map(|data| data.is_favorite)
            .unwrap_or(false)
    }
}