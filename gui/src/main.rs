use lib_gui::SimApp;

fn main() -> iced::Result {
    iced::application(SimApp::default, SimApp::update, SimApp::view)
        .title("Simulation")
        .subscription(SimApp::subscription)
        .theme(SimApp::theme)
        .run()
}
