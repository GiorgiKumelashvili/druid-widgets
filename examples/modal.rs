use druid::widget::{Button, Flex, Label};
use druid::{
    AppLauncher, Data, Lens, LocalizedString, PlatformError, Point, Widget, WidgetExt,
    WindowConfig, WindowDesc, WindowLevel, commands,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    count: u32,
}

// best example of modal because it is not considered as separated window or process, unlike jetbrains
// but with added benefit that you could take position based on screen since it can go outbounds of application
// so if app is clipped on either side screen then modal will never be clipped
fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_root_widget())
        .title(LocalizedString::new("modal Example"))
        .window_size((300.0, 200.0));

    AppLauncher::with_window(main_window).launch(AppState { count: 0 })
}

fn build_root_widget() -> impl Widget<AppState> {
    let button = Button::new("Open modal").on_click(|ctx, data: &mut AppState, env| {
        let parent = ctx.window().clone();
        let position = ctx.to_window(Point::new(100.0, 100.0));

        let modal_window = WindowConfig::default()
            // let modal_window = WindowDesc::new(modal_widget())
            .set_position(position)
            .window_size((150.0, 100.0))
            .show_titlebar(false)
            .resizable(false)
            .set_level(WindowLevel::Modal(parent));

        ctx.new_sub_window(modal_window, modal_widget(), data.clone(), env.clone());
    });

    Flex::column()
        .with_child(Label::new("Main window"))
        .with_spacer(20.0)
        .with_child(button)
        .center()
}

fn modal_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("This is a modal"))
        .with_spacer(10.0)
        .with_child(Button::new("Close").on_click(|ctx, _data, _env| {
            ctx.submit_command(commands::CLOSE_WINDOW);
        }))
        .padding(10.0)
}
