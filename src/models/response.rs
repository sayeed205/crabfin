use serde::{Deserialize, Serialize};
use super::jellyfin::BaseItem;

/// Generic response wrapper for paginated results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    #[serde(rename = "Items")]
    pub items: Vec<T>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: i32,
    #[serde(rename = "StartIndex")]
    pub start_index: i32,
}

/// Response for items query
pub type ItemsResponse = QueryResult<BaseItem>;

/// Response for search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "SearchHints")]
    pub search_hints: Vec<SearchHint>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: i32,
}

/// Search hint result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHint {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MatchedTerm")]
    pub matched_term: Option<String>,
    #[serde(rename = "IndexNumber")]
    pub index_number: Option<i32>,
    #[serde(rename = "ProductionYear")]
    pub production_year: Option<i32>,
    #[serde(rename = "ParentIndexNumber")]
    pub parent_index_number: Option<i32>,
    #[serde(rename = "PrimaryImageTag")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ThumbImageTag")]
    pub thumb_image_tag: Option<String>,
    #[serde(rename = "ThumbImageItemId")]
    pub thumb_image_item_id: Option<String>,
    #[serde(rename = "BackdropImageTag")]
    pub backdrop_image_tag: Option<String>,
    #[serde(rename = "BackdropImageItemId")]
    pub backdrop_image_item_id: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: String,
    #[serde(rename = "IsFolder")]
    pub is_folder: Option<bool>,
    #[serde(rename = "RunTimeTicks")]
    pub runtime_ticks: Option<i64>,
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    #[serde(rename = "Series")]
    pub series: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Album")]
    pub album: Option<String>,
    #[serde(rename = "AlbumId")]
    pub album_id: Option<String>,
    #[serde(rename = "AlbumArtist")]
    pub album_artist: Option<String>,
    #[serde(rename = "Artists")]
    pub artists: Vec<String>,
    #[serde(rename = "SongCount")]
    pub song_count: Option<i32>,
    #[serde(rename = "EpisodeCount")]
    pub episode_count: Option<i32>,
    #[serde(rename = "ChannelId")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
    #[serde(rename = "PrimaryImageAspectRatio")]
    pub primary_image_aspect_ratio: Option<f64>,
}

/// Response for playback info requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackInfoResponse {
    #[serde(rename = "MediaSources")]
    pub media_sources: Vec<super::jellyfin::MediaSourceInfo>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<String>,
}

/// Library information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "CollectionType")]
    pub collection_type: Option<String>,
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "PrimaryImageTag")]
    pub primary_image_tag: Option<String>,
}

/// Response for library queries
pub type LibrariesResponse = Vec<Library>;

/// Generic API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<String>,
    #[serde(rename = "StackTrace")]
    pub stack_trace: Option<String>,
}

/// Response for system ping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    #[serde(rename = "Value")]
    pub value: String,
}

/// Response for session queries
pub type SessionsResponse = Vec<super::jellyfin::SessionInfo>;

/// Response for user queries
pub type UsersResponse = Vec<super::jellyfin::UserInfo>;

/// Image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "ImageType")]
    pub image_type: String,
    #[serde(rename = "ImageIndex")]
    pub image_index: Option<i32>,
    #[serde(rename = "ImageTag")]
    pub image_tag: String,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "BlurHash")]
    pub blur_hash: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "Width")]
    pub width: Option<i32>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
}

/// Response for image queries
pub type ImagesResponse = Vec<ImageInfo>;