use jellyfin::client::{ClientBuilder, AuthenticatedClient};
use jellyfin::user::authenticate_by_name;
use jellyfin::library::{get_views, get_items, ItemsQuery};
use jellyfin::models::CollectionType;

const TEST_SERVER: &str = "http://localhost:8096";
const TEST_USER: &str = "hitarashi";
const TEST_PASS: &str = "9015@Media";

async fn get_authenticated_client() -> AuthenticatedClient {
    let uuid = uuid::Uuid::new_v4().to_string();
    let client = ClientBuilder::new(TEST_SERVER)
        .unwrap()
        .device_name(format!("TestDevice-{}", uuid))
        .device_id(format!("TestDeviceID-{}", uuid))
        .build()
        .unwrap();

    let auth_result = authenticate_by_name(&client, TEST_USER, TEST_PASS)
        .await
        .expect("Failed to authenticate test user");

    AuthenticatedClient::new(client, auth_result)
}

#[tokio::test]
#[ignore = "Requires local Jellyfin server"]
async fn test_get_views_returns_libraries() {
    let client = get_authenticated_client().await;
    let views = get_views(&client).await.expect("Failed to get views");

    assert!(views.total_record_count.unwrap() > 0);
    assert!(!views.items.is_empty());

    let has_known_type = views.items.iter().any(|item| {
        matches!(
            item.collection_type,
            Some(CollectionType::Movies) | Some(CollectionType::TvShows) | Some(CollectionType::Music)
        )
    });
    
    assert!(has_known_type, "Should have at least one standard library type (Movies, TV, Music)");
}

#[tokio::test]
#[ignore = "Requires local Jellyfin server"]
async fn test_get_items_by_parent() {
    let client = get_authenticated_client().await;
    let views = get_views(&client).await.expect("Failed to get views");
    
    if let Some(first_view) = views.items.first() {
        let query = ItemsQuery {
            parent_id: Some(first_view.id.clone()),
            limit: Some(5),
            fields: Some(vec!["ParentId".to_string()]),
            ..Default::default()
        };
        
        let items = get_items(&client, &query).await.expect("Failed to get items by parent");
        assert!(items.total_record_count.is_some());
        
        if items.total_record_count.unwrap() > 0 {
            assert!(!items.items.is_empty());
        }
    }
}

#[tokio::test]
#[ignore = "Requires local Jellyfin server"]
async fn test_get_items_recursive_movies() {
    let client = get_authenticated_client().await;
    
    let query = ItemsQuery {
        include_item_types: Some(vec!["Movie".to_string()]),
        recursive: Some(true),
        sort_by: Some(vec!["SortName".to_string()]),
        limit: Some(5),
        ..Default::default()
    };

    let items = get_items(&client, &query).await.expect("Failed to get movies");
    
    if let Some(count) = items.total_record_count {
        if count > 0 {
            assert!(!items.items.is_empty());
            assert_eq!(items.items[0].type_, "Movie");
        }
    }
}

#[tokio::test]
#[ignore = "Requires local Jellyfin server"]
async fn test_get_items_pagination() {
    let client = get_authenticated_client().await;
    
    let query1 = ItemsQuery {
        recursive: Some(true),
        include_item_types: Some(vec!["Movie".to_string()]),
        limit: Some(2),
        ..Default::default()
    };
    
    let result1 = get_items(&client, &query1).await.unwrap();
    
    if result1.items.len() == 2 {
        let _first_id = &result1.items[0].id;
        
        let query2 = ItemsQuery {
            recursive: Some(true),
            include_item_types: Some(vec!["Movie".to_string()]),
            limit: Some(1),
            start_index: Some(1),
            ..Default::default()
        };
        
        let result2 = get_items(&client, &query2).await.unwrap();
        assert_eq!(result2.items.len(), 1);
        
        let query_sorted = ItemsQuery {
            recursive: Some(true),
            include_item_types: Some(vec!["Movie".to_string()]),
            sort_by: Some(vec!["DateCreated".to_string()]),
            limit: Some(2),
            ..Default::default()
        };
        
        let sorted_res = get_items(&client, &query_sorted).await.unwrap();
        
        if sorted_res.items.len() == 2 {
            let item2_id = &sorted_res.items[1].id;
            
            let query_offset = ItemsQuery {
                recursive: Some(true),
                include_item_types: Some(vec!["Movie".to_string()]),
                sort_by: Some(vec!["DateCreated".to_string()]),
                limit: Some(1),
                start_index: Some(1),
                ..Default::default()
            };
            
            let offset_res = get_items(&client, &query_offset).await.unwrap();
            assert_eq!(offset_res.items[0].id, *item2_id);
        }
    }
}
