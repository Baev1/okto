use super::launches::{LaunchData, LaunchInfo};

impl From<LaunchInfo> for LaunchData {
    fn from(mut info: LaunchInfo) -> LaunchData {
        if let Some(urls) = info.vid_urls.as_mut() {
            urls.sort_by_key(|u| u.priority);
            urls.dedup_by_key(|u| u.title.clone());
        };

        LaunchData {
            id: 0,
            ll_id: info.id,
            launch_name: info.name,
            status: info.status.id,
            payload: info
                .mission
                .clone()
                .map(|m| m.name)
                .unwrap_or(String::from("payload unknown")),
            vid_urls: info.vid_urls.unwrap_or_default(),
            vehicle: info.rocket.configuration.full_name,
            location: info.pad.name,
            net: info.net,
            launch_window: info.window_end - info.window_start,
            rocket_img: info.image,
            mission_type: info
                .mission
                .clone()
                .map(|m| m.mission_type)
                .unwrap_or(String::from("mission type unknown")),
            mission_description: info
                .mission
                .clone()
                .map(|m| m.description)
                .unwrap_or(String::from("mission description unknown")),
            lsp: info.launch_service_provider.name,
        }
    }
}
