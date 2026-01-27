use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CollectionType {
    Movies,
    TvShows,
    Music,
    MusicVideos,
    Trailers,
    HomeVideos,
    BoxSets,
    Books,
    Photos,
    LiveTv,
    Playlists,
    Folders,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserItemDataDto {
    pub playback_position_ticks: Option<i64>,
    pub play_count: Option<i32>,
    pub is_favorite: Option<bool>,
    pub played: Option<bool>,
    pub played_percentage: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemPerson {
    pub id: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    pub primary_image_tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameIdPair {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalUrl {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDto {
    pub id: String,
    pub name: Option<String>,
    pub server_id: Option<String>,
    #[serde(rename = "Type")]
    pub type_: String, // This is required
    pub collection_type: Option<CollectionType>,
    pub is_folder: Option<bool>,
    pub parent_id: Option<String>,
    pub overview: Option<String>,
    pub production_year: Option<i32>,
    pub premiere_date: Option<String>,
    pub run_time_ticks: Option<i64>,
    pub community_rating: Option<f32>,
    pub official_rating: Option<String>,
    pub image_tags: Option<HashMap<String, String>>,
    pub backdrop_image_tags: Option<Vec<String>>,
    pub user_data: Option<UserItemDataDto>,
    pub series_id: Option<String>,
    pub series_name: Option<String>,
    pub season_id: Option<String>,
    pub season_name: Option<String>,
    pub genres: Option<Vec<String>>,
    pub studios: Option<Vec<NameIdPair>>,
    pub people: Option<Vec<BaseItemPerson>>,
    pub taglines: Option<Vec<String>>,
    pub index_number: Option<i32>,
    pub parent_index_number: Option<i32>,
    pub child_count: Option<i32>,
    pub external_urls: Option<Vec<ExternalUrl>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDtoQueryResult {
    pub items: Vec<BaseItemDto>,
    pub total_record_count: Option<i32>,
    pub start_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub local_address: Option<String>,
    pub server_name: String,
    pub version: String,
    pub product_name: String,
    pub operating_system: Option<String>,
    pub id: String,
    pub startup_wizard_completed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicUserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
    pub primary_image_tag: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateRequest {
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub user: UserInfo,
    pub access_token: String,
    pub server_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DlnaProfileType {
    Video,
    Audio,
    Photo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubtitleDeliveryMethod {
    Encode,
    Embed,
    External,
    Drop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DirectPlayProfile {
    pub container: Option<String>,
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    #[serde(rename = "Type")]
    pub type_: DlnaProfileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranscodingProfile {
    pub container: String,
    #[serde(rename = "Type")]
    pub type_: DlnaProfileType,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub protocol: Option<String>,
    pub estimate_content_length: Option<bool>,
    pub transcoding_seek_info: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubtitleProfile {
    pub format: String,
    pub method: SubtitleDeliveryMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceProfile {
    pub name: Option<String>,
    pub max_streaming_bitrate: Option<i64>,
    pub max_static_bitrate: Option<i64>,
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    pub transcoding_profiles: Vec<TranscodingProfile>,
    pub subtitle_profiles: Vec<SubtitleProfile>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackInfoRequest {
    pub user_id: String,
    pub max_streaming_bitrate: Option<i64>,
    pub start_time_ticks: Option<i64>,
    pub audio_stream_index: Option<i32>,
    pub subtitle_stream_index: Option<i32>,
    pub max_audio_channels: Option<i32>,
    pub media_source_id: Option<String>,
    pub enable_direct_play: Option<bool>,
    pub enable_direct_stream: Option<bool>,
    pub enable_transcoding: Option<bool>,
    pub auto_open_live_stream: Option<bool>,
    pub device_profile: DeviceProfile,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackInfoResponse {
    pub media_sources: Vec<MediaSourceInfo>,
    pub play_session_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum MediaStreamType {
    Video,
    Audio,
    Subtitle,
    EmbeddedImage,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaStream {
    pub index: i32,
    #[serde(rename = "Type")]
    pub type_: MediaStreamType,
    pub codec: Option<String>,
    pub language: Option<String>,
    pub display_title: Option<String>,
    pub is_default: Option<bool>,
    pub is_forced: Option<bool>,
    pub is_external: Option<bool>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub bit_rate: Option<i64>,
    pub channels: Option<i32>,
    pub sample_rate: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSourceInfo {
    pub id: String,
    pub name: Option<String>,
    pub container: Option<String>,
    pub size: Option<i64>,
    pub bitrate: Option<i64>,
    pub supports_direct_play: Option<bool>,
    pub supports_direct_stream: Option<bool>,
    pub supports_transcoding: Option<bool>,
    pub direct_stream_url: Option<String>,
    pub transcoding_url: Option<String>,
    pub media_streams: Option<Vec<MediaStream>>,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
}

// ============================================================================
// Playback Reporting Types (Phase 16)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayMethod {
    DirectPlay,
    DirectStream,
    Transcode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepeatMode {
    RepeatNone,
    RepeatAll,
    RepeatOne,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStartInfo {
    pub item_id: String,
    pub media_source_id: Option<String>,
    pub play_session_id: Option<String>,
    pub play_method: PlayMethod,
    pub can_seek: bool,
    pub position_ticks: i64,
    pub audio_stream_index: Option<i32>,
    pub subtitle_stream_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStopInfo {
    pub item_id: String,
    pub media_source_id: Option<String>,
    pub play_session_id: Option<String>,
    pub position_ticks: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackProgressInfo {
    pub item_id: String,
    pub media_source_id: Option<String>,
    pub play_session_id: Option<String>,
    pub position_ticks: i64,
    pub is_paused: bool,
    pub is_muted: bool,
    pub volume_level: Option<i32>,
    pub play_method: PlayMethod,
    pub repeat_mode: RepeatMode,
    pub can_seek: bool,
    pub audio_stream_index: Option<i32>,
    pub subtitle_stream_index: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_serialize_device_profile() {
        let profile = DeviceProfile {
            name: Some("Crabfin".to_string()),
            max_streaming_bitrate: Some(120000000),
            max_static_bitrate: None,
            direct_play_profiles: vec![DirectPlayProfile {
                container: Some("mp4,mkv".to_string()),
                audio_codec: Some("aac,ac3".to_string()),
                video_codec: Some("h264,hevc".to_string()),
                type_: DlnaProfileType::Video,
            }],
            transcoding_profiles: vec![],
            subtitle_profiles: vec![SubtitleProfile {
                format: "srt".to_string(),
                method: SubtitleDeliveryMethod::External,
            }],
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("\"Name\":\"Crabfin\""));
        assert!(json.contains("\"MaxStreamingBitrate\":120000000"));
        assert!(json.contains("\"Type\":\"Video\""));
        assert!(json.contains("\"Method\":\"External\""));
    }

    #[test]
    fn test_serialize_direct_play_profile() {
        let profile = DirectPlayProfile {
            container: Some("mp4".to_string()),
            audio_codec: None,
            video_codec: Some("h264".to_string()),
            type_: DlnaProfileType::Video,
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("\"Container\":\"mp4\""));
        assert!(json.contains("\"Type\":\"Video\""));
        assert!(json.contains("\"VideoCodec\":\"h264\""));
    }

    #[test]
    fn test_serialize_subtitle_profile() {
        let profile = SubtitleProfile {
            format: "ass".to_string(),
            method: SubtitleDeliveryMethod::Embed,
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("\"Format\":\"ass\""));
        assert!(json.contains("\"Method\":\"Embed\""));
    }

    #[test]
    fn test_deserialize_public_server_info() {
        let json = r#"{
            "LocalAddress": "http://172.18.0.9:8096",
            "ServerName": "Local",
            "Version": "10.11.6",
            "ProductName": "Jellyfin Server",
            "OperatingSystem": "Linux",
            "Id": "3b77d74a3bca411b924cf99fecb915e9",
            "StartupWizardCompleted": true
        }"#;

        let info: PublicServerInfo =
            from_str(json).expect("Failed to deserialize PublicServerInfo");

        assert_eq!(info.server_name, "Local");
        assert_eq!(info.version, "10.11.6");
        assert_eq!(info.product_name, "Jellyfin Server");
        assert_eq!(info.id, "3b77d74a3bca411b924cf99fecb915e9");
        assert!(info.startup_wizard_completed);
    }

    #[test]
    fn test_serialize_authenticate_request() {
        let req = AuthenticateRequest {
            username: "testuser".into(),
            password: "password123".into(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""Username":"testuser""#));
        assert!(json.contains(r#""Pw":"password123""#));
    }

    #[test]
    fn test_deserialize_authentication_result() {
        let json = r#"{
            "User": {
                "Id": "user-123",
                "Name": "testuser",
                "HasPassword": true
            },
            "AccessToken": "token-abc-123",
            "ServerId": "server-xyz-789"
        }"#;

        let result: AuthenticationResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.access_token, "token-abc-123");
        assert_eq!(result.server_id, "server-xyz-789");
        assert_eq!(result.user.id, "user-123");
        assert_eq!(result.user.name, "testuser");
        assert!(result.user.has_password);
    }

    #[test]
    fn test_deserialize_collection_type() {
        let movies: CollectionType = serde_json::from_str(r#""movies""#).unwrap();
        assert_eq!(movies, CollectionType::Movies);

        let unknown: CollectionType = serde_json::from_str(r#""future_type""#).unwrap();
        assert_eq!(unknown, CollectionType::Unknown);
    }

    #[test]
    fn test_deserialize_user_item_data() {
        let json = r#"{
            "PlaybackPositionTicks": 1000,
            "PlayCount": 5,
            "IsFavorite": true,
            "Played": true
        }"#;

        let data: UserItemDataDto = serde_json::from_str(json).unwrap();
        assert_eq!(data.playback_position_ticks, Some(1000));
        assert_eq!(data.play_count, Some(5));
        assert!(data.is_favorite.unwrap());
        assert!(data.played.unwrap());
    }

    #[test]
    fn test_deserialize_base_item_dto() {
        let json = r#"{
            "Id": "item-123",
            "Name": "Test Movie",
            "Type": "Movie",
            "ProductionYear": 2023,
            "RunTimeTicks": 72000000000
        }"#;

        let item: BaseItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-123");
        assert_eq!(item.name, Some("Test Movie".to_string()));
        assert_eq!(item.type_, "Movie");
        assert_eq!(item.production_year, Some(2023));
        assert_eq!(item.run_time_ticks, Some(72000000000));
    }

    #[test]
    fn test_deserialize_query_result() {
        let json = r#"{
            "Items": [
                {
                    "Id": "item-1",
                    "Name": "Item 1",
                    "Type": "Movie"
                },
                {
                    "Id": "item-2",
                    "Name": "Item 2",
                    "Type": "Series"
                }
            ],
            "TotalRecordCount": 2,
            "StartIndex": 0
        }"#;

        let result: BaseItemDtoQueryResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total_record_count, Some(2));
        assert_eq!(result.items[0].name, Some("Item 1".to_string()));
    }

    #[test]
    fn test_deserialize_base_item_person() {
        let json = r#"{
            "Id": "person-1",
            "Name": "Actor Name",
            "Role": "Hero",
            "Type": "Actor",
            "PrimaryImageTag": "tag-123"
        }"#;

        let person: BaseItemPerson = serde_json::from_str(json).unwrap();
        assert_eq!(person.id, Some("person-1".to_string()));
        assert_eq!(person.name, Some("Actor Name".to_string()));
        assert_eq!(person.role, Some("Hero".to_string()));
        assert_eq!(person.type_, Some("Actor".to_string()));
        assert_eq!(person.primary_image_tag, Some("tag-123".to_string()));
    }

    #[test]
    fn test_deserialize_base_item_dto_with_details() {
        let json = r#"{
            "Id": "item-123",
            "Name": "Test Movie",
            "Type": "Movie",
            "Genres": ["Action", "Adventure"],
            "Studios": [
                { "Id": "studio-1", "Name": "Studio One" }
            ],
            "People": [
                { "Name": "Actor 1", "Type": "Actor" }
            ],
            "Taglines": ["Just when you thought it was safe..."],
            "IndexNumber": 1,
            "ParentIndexNumber": 2,
            "ChildCount": 5,
            "ExternalUrls": [
                { "Name": "IMDb", "Url": "https://imdb.com/title/tt1234567" }
            ]
        }"#;

        let item: BaseItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-123");
        assert_eq!(item.name, Some("Test Movie".to_string()));
        assert_eq!(item.genres.unwrap(), vec!["Action", "Adventure"]);
        assert_eq!(
            item.studios.unwrap()[0].name,
            Some("Studio One".to_string())
        );
        assert_eq!(item.people.unwrap()[0].name, Some("Actor 1".to_string()));
        assert_eq!(
            item.taglines.unwrap()[0],
            "Just when you thought it was safe..."
        );
        assert_eq!(item.index_number, Some(1));
        assert_eq!(item.parent_index_number, Some(2));
        assert_eq!(item.child_count, Some(5));
        assert_eq!(
            item.external_urls.unwrap()[0].name,
            Some("IMDb".to_string())
        );
    }

    #[test]
    fn test_serialize_playback_info_request() {
        let profile = DeviceProfile {
            name: Some("Test".to_string()),
            max_streaming_bitrate: None,
            max_static_bitrate: None,
            direct_play_profiles: vec![],
            transcoding_profiles: vec![],
            subtitle_profiles: vec![],
        };

        let request = PlaybackInfoRequest {
            user_id: "user-123".to_string(),
            max_streaming_bitrate: Some(120000000),
            start_time_ticks: Some(0),
            audio_stream_index: None,
            subtitle_stream_index: None,
            max_audio_channels: Some(6),
            media_source_id: None,
            enable_direct_play: Some(true),
            enable_direct_stream: Some(true),
            enable_transcoding: Some(true),
            auto_open_live_stream: Some(true),
            device_profile: profile,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"UserId\":\"user-123\""));
        assert!(json.contains("\"MaxStreamingBitrate\":120000000"));
        assert!(json.contains("\"EnableDirectPlay\":true"));
        assert!(json.contains("\"DeviceProfile\":{"));
    }

    #[test]
    fn test_deserialize_playback_info_response() {
        let json = r#"{
            "MediaSources": [
                {
                    "Id": "source-1",
                    "SupportsDirectPlay": true
                }
            ],
            "PlaySessionId": "session-abc-123"
        }"#;

        let response: PlaybackInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.play_session_id,
            Some("session-abc-123".to_string())
        );
        assert_eq!(response.media_sources.len(), 1);
        assert_eq!(response.media_sources[0].id, "source-1");
    }

    #[test]
    fn test_deserialize_media_stream() {
        let json = r#"{
            "Index": 0,
            "Type": "Video",
            "Codec": "h264",
            "Language": "eng",
            "DisplayTitle": "1080p H.264",
            "IsDefault": true,
            "Height": 1080,
            "Width": 1920,
            "BitRate": 5000000
        }"#;

        let stream: MediaStream = serde_json::from_str(json).unwrap();
        assert_eq!(stream.index, 0);
        assert_eq!(stream.type_, MediaStreamType::Video);
        assert_eq!(stream.codec, Some("h264".to_string()));
        assert_eq!(stream.height, Some(1080));
        assert_eq!(stream.width, Some(1920));
    }

    #[test]
    fn test_deserialize_media_stream_audio() {
        let json = r#"{
            "Index": 1,
            "Type": "Audio",
            "Codec": "aac",
            "Language": "eng",
            "DisplayTitle": "English AAC 5.1",
            "IsDefault": true,
            "Channels": 6,
            "SampleRate": 48000
        }"#;

        let stream: MediaStream = serde_json::from_str(json).unwrap();
        assert_eq!(stream.type_, MediaStreamType::Audio);
        assert_eq!(stream.channels, Some(6));
    }

    #[test]
    fn test_deserialize_media_source_info() {
        let json = r#"{
            "Id": "source-123",
            "Name": "Movie.mkv",
            "Container": "mkv",
            "Size": 1500000000,
            "Bitrate": 8000000,
            "SupportsDirectPlay": true,
            "SupportsDirectStream": true,
            "SupportsTranscoding": true,
            "DirectStreamUrl": "/Videos/123/stream.mkv?static=true",
            "MediaStreams": [
                {"Index": 0, "Type": "Video", "Codec": "h264"},
                {"Index": 1, "Type": "Audio", "Codec": "aac"}
            ],
            "ETag": "abc123"
        }"#;

        let source: MediaSourceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(source.id, "source-123");
        assert_eq!(source.container, Some("mkv".to_string()));
        assert_eq!(source.supports_direct_play, Some(true));
        assert!(source.media_streams.is_some());
        assert_eq!(source.media_streams.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_media_stream_type_unknown() {
        let json = r#"{"Index": 0, "Type": "FutureType"}"#;
        let stream: MediaStream = serde_json::from_str(json).unwrap();
        assert_eq!(stream.type_, MediaStreamType::Unknown);
    }

    #[test]
    fn test_serialize_play_method() {
        assert_eq!(
            serde_json::to_string(&PlayMethod::DirectPlay).unwrap(),
            "\"DirectPlay\""
        );
        assert_eq!(
            serde_json::to_string(&PlayMethod::DirectStream).unwrap(),
            "\"DirectStream\""
        );
        assert_eq!(
            serde_json::to_string(&PlayMethod::Transcode).unwrap(),
            "\"Transcode\""
        );
    }

    #[test]
    fn test_serialize_repeat_mode() {
        assert_eq!(
            serde_json::to_string(&RepeatMode::RepeatNone).unwrap(),
            "\"RepeatNone\""
        );
        assert_eq!(
            serde_json::to_string(&RepeatMode::RepeatAll).unwrap(),
            "\"RepeatAll\""
        );
        assert_eq!(
            serde_json::to_string(&RepeatMode::RepeatOne).unwrap(),
            "\"RepeatOne\""
        );
    }

    #[test]
    fn test_serialize_playback_start_info() {
        let info = PlaybackStartInfo {
            item_id: "item-123".to_string(),
            media_source_id: Some("source-456".to_string()),
            play_session_id: Some("session-789".to_string()),
            play_method: PlayMethod::DirectPlay,
            can_seek: true,
            position_ticks: 0,
            audio_stream_index: Some(1),
            subtitle_stream_index: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"ItemId\":\"item-123\""));
        assert!(json.contains("\"MediaSourceId\":\"source-456\""));
        assert!(json.contains("\"PlaySessionId\":\"session-789\""));
        assert!(json.contains("\"PlayMethod\":\"DirectPlay\""));
        assert!(json.contains("\"CanSeek\":true"));
        assert!(json.contains("\"PositionTicks\":0"));
    }

    #[test]
    fn test_serialize_playback_stop_info() {
        let info = PlaybackStopInfo {
            item_id: "item-123".to_string(),
            media_source_id: Some("source-456".to_string()),
            play_session_id: Some("session-789".to_string()),
            position_ticks: 100_000_000, // 10 seconds
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"ItemId\":\"item-123\""));
        assert!(json.contains("\"MediaSourceId\":\"source-456\""));
        assert!(json.contains("\"PlaySessionId\":\"session-789\""));
        assert!(json.contains("\"PositionTicks\":100000000"));
    }

    #[test]
    fn test_serialize_playback_progress_info() {
        let info = PlaybackProgressInfo {
            item_id: "item-123".to_string(),
            media_source_id: Some("source-456".to_string()),
            play_session_id: Some("session-789".to_string()),
            position_ticks: 50_000_000, // 5 seconds
            is_paused: false,
            is_muted: false,
            volume_level: Some(100),
            play_method: PlayMethod::DirectPlay,
            repeat_mode: RepeatMode::RepeatNone,
            can_seek: true,
            audio_stream_index: Some(1),
            subtitle_stream_index: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"ItemId\":\"item-123\""));
        assert!(json.contains("\"PositionTicks\":50000000"));
        assert!(json.contains("\"IsPaused\":false"));
        assert!(json.contains("\"IsMuted\":false"));
        assert!(json.contains("\"VolumeLevel\":100"));
        assert!(json.contains("\"RepeatMode\":\"RepeatNone\""));
    }
}
