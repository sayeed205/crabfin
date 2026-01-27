//! Playback API for Jellyfin stream negotiation.
//!
//! This module provides functions to negotiate playback with a Jellyfin server,
//! including device profile handling and stream URL construction.

use crate::client::AuthenticatedClient;
use crate::error::Result;
use crate::models::{
    DeviceProfile, DirectPlayProfile, DlnaProfileType, PlaybackInfoRequest, PlaybackInfoResponse,
    PlaybackProgressInfo, PlaybackStartInfo, PlaybackStopInfo, SubtitleDeliveryMethod,
    SubtitleProfile,
};

/// Convert seconds to Jellyfin ticks (1 second = 10,000,000 ticks)
pub fn ticks_from_seconds(seconds: f64) -> i64 {
    (seconds * 10_000_000.0) as i64
}

/// Convert Jellyfin ticks to seconds
pub fn seconds_from_ticks(ticks: i64) -> f64 {
    ticks as f64 / 10_000_000.0
}

/// Get playback info for an item, negotiating the best stream based on device capabilities.
///
/// This sends the device profile to the server and receives back information about
/// available media sources and how to play them (direct play, direct stream, or transcode).
pub async fn get_playback_info(
    client: &AuthenticatedClient,
    item_id: &str,
    device_profile: DeviceProfile,
) -> Result<PlaybackInfoResponse> {
    let request = PlaybackInfoRequest {
        user_id: client.user_id().to_string(),
        max_streaming_bitrate: device_profile.max_streaming_bitrate,
        start_time_ticks: Some(0),
        audio_stream_index: None,
        subtitle_stream_index: None,
        max_audio_channels: Some(6),
        media_source_id: None,
        enable_direct_play: Some(true),
        enable_direct_stream: Some(true),
        enable_transcoding: Some(true),
        auto_open_live_stream: Some(true),
        device_profile,
    };

    let path = format!("/Items/{}/PlaybackInfo", item_id);
    let response = client.post_json(&path, &request).await?;
    let playback_info = response.json::<PlaybackInfoResponse>().await?;
    Ok(playback_info)
}

/// Create a default device profile for direct play of common formats.
///
/// This profile supports:
/// - Containers: mp4, mkv, webm, avi, mov
/// - Video codecs: h264, hevc, vp8, vp9, av1
/// - Audio codecs: aac, ac3, eac3, mp3, flac, opus, vorbis
/// - Subtitles: srt, ass, vtt (external delivery)
///
/// Max streaming bitrate is set to 120 Mbps (sufficient for 4K content).
pub fn create_default_device_profile() -> DeviceProfile {
    DeviceProfile {
        name: Some("Crabfin".to_string()),
        max_streaming_bitrate: Some(120_000_000), // 120 Mbps
        max_static_bitrate: Some(120_000_000),
        direct_play_profiles: vec![
            DirectPlayProfile {
                container: Some("mp4,mkv,webm,avi,mov".to_string()),
                audio_codec: Some("aac,ac3,eac3,mp3,flac,opus,vorbis".to_string()),
                video_codec: Some("h264,hevc,vp8,vp9,av1".to_string()),
                type_: DlnaProfileType::Video,
            },
            DirectPlayProfile {
                container: Some("mp3,flac,aac,ogg,wav".to_string()),
                audio_codec: Some("mp3,flac,aac,vorbis,opus".to_string()),
                video_codec: None,
                type_: DlnaProfileType::Audio,
            },
        ],
        transcoding_profiles: vec![], // No transcoding profiles for now (direct play only)
        subtitle_profiles: vec![
            SubtitleProfile {
                format: "srt".to_string(),
                method: SubtitleDeliveryMethod::External,
            },
            SubtitleProfile {
                format: "ass".to_string(),
                method: SubtitleDeliveryMethod::External,
            },
            SubtitleProfile {
                format: "ssa".to_string(),
                method: SubtitleDeliveryMethod::External,
            },
            SubtitleProfile {
                format: "vtt".to_string(),
                method: SubtitleDeliveryMethod::External,
            },
        ],
    }
}

// ============================================================================
// Playback Reporting (Phase 16)
// ============================================================================

/// Report to the server that playback has started.
///
/// This should be called when the video player begins playing an item.
/// The server uses this to show "Now Playing" on the dashboard.
pub async fn report_playback_started(
    client: &AuthenticatedClient,
    info: &PlaybackStartInfo,
) -> Result<()> {
    client.post_json("/Sessions/Playing", info).await?;
    Ok(())
}

/// Report playback progress to the server (heartbeat).
///
/// This should be called periodically (typically every 10 seconds) while
/// playing to update the server on current position, pause state, etc.
/// The server uses this for "Continue Watching" synchronization.
pub async fn report_playback_progress(
    client: &AuthenticatedClient,
    info: &PlaybackProgressInfo,
) -> Result<()> {
    client.post_json("/Sessions/Playing/Progress", info).await?;
    Ok(())
}

/// Report to the server that playback has stopped.
///
/// This should be called when the video player stops playing (user stops,
/// video ends, or user navigates away). The server uses this to update
/// the user's watch progress.
pub async fn report_playback_stopped(
    client: &AuthenticatedClient,
    info: &PlaybackStopInfo,
) -> Result<()> {
    client.post_json("/Sessions/Playing/Stopped", info).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_device_profile() {
        let profile = create_default_device_profile();

        assert_eq!(profile.name, Some("Crabfin".to_string()));
        assert_eq!(profile.max_streaming_bitrate, Some(120_000_000));

        // Should have video and audio direct play profiles
        assert_eq!(profile.direct_play_profiles.len(), 2);
        assert_eq!(profile.direct_play_profiles[0].type_, DlnaProfileType::Video);
        assert_eq!(profile.direct_play_profiles[1].type_, DlnaProfileType::Audio);

        // Should support common containers
        let video_containers = profile.direct_play_profiles[0].container.as_ref().unwrap();
        assert!(video_containers.contains("mp4"));
        assert!(video_containers.contains("mkv"));

        // Should have subtitle profiles
        assert!(!profile.subtitle_profiles.is_empty());
        assert_eq!(profile.subtitle_profiles[0].format, "srt");
    }

    #[test]
    fn test_default_profile_serializes() {
        let profile = create_default_device_profile();
        let json = serde_json::to_string(&profile).unwrap();

        assert!(json.contains("\"Name\":\"Crabfin\""));
        assert!(json.contains("\"MaxStreamingBitrate\":120000000"));
        assert!(json.contains("\"DirectPlayProfiles\""));
    }

    #[test]
    fn test_ticks_from_seconds() {
        assert_eq!(ticks_from_seconds(1.0), 10_000_000);
        assert_eq!(ticks_from_seconds(0.5), 5_000_000);
        assert_eq!(ticks_from_seconds(60.0), 600_000_000);
        assert_eq!(ticks_from_seconds(0.0), 0);
    }

    #[test]
    fn test_seconds_from_ticks() {
        assert_eq!(seconds_from_ticks(10_000_000), 1.0);
        assert_eq!(seconds_from_ticks(5_000_000), 0.5);
        assert_eq!(seconds_from_ticks(600_000_000), 60.0);
        assert_eq!(seconds_from_ticks(0), 0.0);
    }

    /// Integration test for full playback flow.
    /// Requires local Jellyfin server at localhost:8096.
    #[tokio::test]
    #[ignore = "Requires local Jellyfin server with media content"]
    async fn test_full_playback_flow() {
        use crate::client::ClientBuilder;
        use crate::library::{get_items, ItemsQuery};
        use crate::user::authenticate_by_name;

        // 1. Connect and authenticate
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();

        let auth_result = authenticate_by_name(&client, "hitarashi", "9015@Media")
            .await
            .expect("Failed to authenticate");

        let auth_client =
            crate::client::AuthenticatedClient::new(client, auth_result);

        // 2. Get a playable item (movie or episode)
        let query = ItemsQuery::default()
            .with_recursive(true)
            .with_limit(1);

        let items = get_items(&auth_client, &query)
            .await
            .expect("Failed to get items");

        assert!(!items.items.is_empty(), "No items found in library");
        let item = &items.items[0];
        println!("Testing playback for: {} ({})", item.name.as_deref().unwrap_or("Unknown"), item.type_);

        // 3. Get playback info
        let device_profile = create_default_device_profile();
        let playback_info = get_playback_info(&auth_client, &item.id, device_profile)
            .await
            .expect("Failed to get playback info");

        // 4. Verify we got media sources
        assert!(!playback_info.media_sources.is_empty(), "No media sources returned");
        println!("Got {} media source(s)", playback_info.media_sources.len());

        // 5. Verify we got a play session ID
        assert!(playback_info.play_session_id.is_some(), "No play session ID returned");
        println!("Play session ID: {}", playback_info.play_session_id.as_ref().unwrap());

        // 6. Get stream URL
        let media_source = &playback_info.media_sources[0];
        println!(
            "Media source: {} (container: {:?}, direct_play: {:?})",
            media_source.id,
            media_source.container,
            media_source.supports_direct_play
        );

        let stream_url = auth_client.stream_url(&item.id, media_source);
        assert!(stream_url.is_some(), "Failed to construct stream URL");

        let url = stream_url.unwrap();
        println!("Stream URL: {}", url);

        // 7. Verify URL is reachable (HEAD request)
        let http_client = reqwest::Client::new();
        let head_response = http_client
            .head(&url)
            .send()
            .await
            .expect("Failed to send HEAD request");

        let status = head_response.status();
        println!("HEAD response status: {}", status);

        // Accept 200 OK or 206 Partial Content
        assert!(
            status.is_success() || status.as_u16() == 206,
            "Stream URL not reachable: {}",
            status
        );

        // Verify content type is video
        if let Some(content_type) = head_response.headers().get("content-type") {
            println!("Content-Type: {:?}", content_type);
        }

        println!("✓ Full playback flow test passed!");
    }

    /// Integration test for playback reporting (Phase 16).
    /// Requires local Jellyfin server at localhost:8096.
    #[tokio::test]
    #[ignore = "Requires local Jellyfin server with media content"]
    async fn test_playback_reporting_flow() {
        use crate::client::ClientBuilder;
        use crate::library::{get_items, ItemsQuery};
        use crate::models::{PlayMethod, RepeatMode};
        use crate::user::authenticate_by_name;

        // 1. Connect and authenticate
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();

        let auth_result = authenticate_by_name(&client, "hitarashi", "9015@Media")
            .await
            .expect("Failed to authenticate");

        let auth_client = crate::client::AuthenticatedClient::new(client, auth_result);

        // 2. Get a playable item
        let query = ItemsQuery::default().with_recursive(true).with_limit(1);

        let items = get_items(&auth_client, &query)
            .await
            .expect("Failed to get items");

        assert!(!items.items.is_empty(), "No items found in library");
        let item = &items.items[0];
        println!(
            "Testing reporting for: {} ({})",
            item.name.as_deref().unwrap_or("Unknown"),
            item.type_
        );

        // 3. Get playback info to get session ID
        let device_profile = create_default_device_profile();
        let playback_info = get_playback_info(&auth_client, &item.id, device_profile)
            .await
            .expect("Failed to get playback info");

        let media_source = &playback_info.media_sources[0];
        let play_session_id = playback_info.play_session_id.clone();

        // 4. Report playback started
        let start_info = PlaybackStartInfo {
            item_id: item.id.clone(),
            media_source_id: Some(media_source.id.clone()),
            play_session_id: play_session_id.clone(),
            play_method: PlayMethod::DirectPlay,
            can_seek: true,
            position_ticks: 0,
            audio_stream_index: None,
            subtitle_stream_index: None,
        };

        report_playback_started(&auth_client, &start_info)
            .await
            .expect("Failed to report playback started");
        println!("✓ Reported playback started");

        // 5. Report progress (simulate 5 seconds in)
        let progress_info = PlaybackProgressInfo {
            item_id: item.id.clone(),
            media_source_id: Some(media_source.id.clone()),
            play_session_id: play_session_id.clone(),
            position_ticks: ticks_from_seconds(5.0),
            is_paused: false,
            is_muted: false,
            volume_level: Some(100),
            play_method: PlayMethod::DirectPlay,
            repeat_mode: RepeatMode::RepeatNone,
            can_seek: true,
            audio_stream_index: None,
            subtitle_stream_index: None,
        };

        report_playback_progress(&auth_client, &progress_info)
            .await
            .expect("Failed to report playback progress");
        println!("✓ Reported playback progress (5 seconds)");

        // 6. Report playback stopped
        let stop_info = PlaybackStopInfo {
            item_id: item.id.clone(),
            media_source_id: Some(media_source.id.clone()),
            play_session_id: play_session_id.clone(),
            position_ticks: ticks_from_seconds(10.0),
        };

        report_playback_stopped(&auth_client, &stop_info)
            .await
            .expect("Failed to report playback stopped");
        println!("✓ Reported playback stopped (10 seconds)");

        println!("✓ Full playback reporting test passed!");
    }
}
