mod domain;
mod infrastructure;
mod ui;

use ui::app::DatabaseManagerApp;
use ui::messages::Message;

fn main() -> iced::Result {
    iced::run(
        "Rojava SaveSlot",
        DatabaseManagerApp::update,
        DatabaseManagerApp::view,
    )
}
