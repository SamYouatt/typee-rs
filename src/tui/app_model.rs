use super::app_page::AppPage;

pub struct AppModel {
    // The app should stop at next loop
    pub app_done: bool,

    pub app_state: AppPage,
}
