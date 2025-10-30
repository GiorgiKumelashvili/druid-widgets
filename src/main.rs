mod widgets;
use druid::widget::{Button, Container, Either, Flex, Label, SizedBox, ZStack};
use druid::{
    AppLauncher, Color, Data, EventCtx, Lens, PlatformError, Point, Size, Widget, WidgetExt,
    WindowDesc,
};

use crate::widgets::advanced_button::AdvancedButton;
use crate::widgets::advanced_dropdown::{DropDownState, main_widget_dropdown};
use crate::widgets::advanced_select::{SelectState, main_widget_select};

#[derive(Debug, Clone, Data, Lens)]
pub struct InitialState {
    counter: u32,
    dropdown_state: DropDownState,
    select_state: SelectState,
    show_popup: bool,
}

fn ui_builder() -> impl Widget<InitialState> {
    let label = Label::new(|data: &u32, _: &_| format!("current value is {}", data))
        .lens(InitialState::counter);

    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut u32, _env| *data += 1)
        .lens(InitialState::counter);

    let button2 = Button::new("change screen pos").on_click(|_ctx, _data, _env| {
        _ctx.window().set_position(Point::new(200., 200.));
    });

    let dropdown = main_widget_dropdown().lens(InitialState::dropdown_state);

    let select = main_widget_select().lens(InitialState::select_state);

    // Button to toggle the popup
    let popup_button = Button::new("Show Popup").on_click(|_ctx, data: &mut InitialState, _env| {
        data.show_popup = !data.show_popup;
    });
    // //
    // // Background widget
    // let background: SizedBox<InitialState> = Label::new("I am the background")
    //     .with_text_color(Color::WHITE)
    //     .background(Color::rgb8(0x33, 0x66, 0x99))
    //     .expand_width()
    //     .fix_height(30.0);

    // // Overlay widget (absolute layer on top)

    // let overlay = Label::new("I'm on top!")
    //     .with_text_alignment(druid::text::TextAlignment::Center)
    //     .with_text_color(Color::RED)
    //     // .padding(30.0)
    //     .background(Color::rgba8(255, 255, 0, 128))
    //     .align_vertical(UnitPoint::CENTER)
    //     .fix_size(150.0, 50.0);
    // // .padding((150.0, 80.0, 0.0, 0.0)); // shifts it down/right outside background;

    // // Stack them
    // let zstack = ZStack::new(background)
    //     .with_child(
    //         overlay,
    //         Vec2::new(1.0, 1.0),
    //         Vec2::ZERO,
    //         UnitPoint::new(300., 0.),
    //         Vec2::ZERO,
    //     )
    //     .expand_width()
    //     .expand_height() // <-- makes ZStack take full width
    //     .background(Color::GRAY); // optional: visualize the ZStack area;
    // // let zstack = ZStack::new(background).with_child(
    // //     overlay,
    // //     Vec2::new(0., 0.),
    // //     Vec2::new(0., 0.),
    // //     UnitPoint::CENTER,
    // //     Vec2::new(0., 0.),
    // // );

    let content = Flex::column()
        .with_child(label)
        .with_default_spacer()
        .with_child(AdvancedButton::new("Advanced Button"))
        .with_default_spacer()
        .with_child(button)
        .with_default_spacer()
        .with_child(button2)
        .with_default_spacer()
        .with_child(dropdown)
        .with_default_spacer()
        .with_child(select)
        .with_default_spacer()
        .with_child(popup_button)
        .with_child(
            Label::new(|data: &bool, _: &_| format!("current value is {}", data))
                .lens(InitialState::show_popup),
        )
        // .with_child(Label::new(format!("Popup {:?}", InitialState::show_popup)))
        // .with_default_spacer()
        // .with_child(zstack)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .main_axis_alignment(druid::widget::MainAxisAlignment::Start)
        .padding(20.0)
        .background(Color::TRANSPARENT);
    // .background(Color::rgb8(0x11, 0x22, 0x33));

    // The popup widget
    let popup = Container::new(
        Flex::column()
            .with_child(Label::new("This is a popup!").with_text_size(24.0))
            .with_spacer(20.0)
            .with_child(
                Button::new("Close").on_click(|_ctx, data: &mut InitialState, _env| {
                    data.show_popup = false;
                }),
            )
            .main_axis_alignment(druid::widget::MainAxisAlignment::Center),
    )
    .background(Color::GREEN)
    // .background(Color::rgb8(0x80, 0x80, 0x80))
    .border(Color::BLACK, 2.0)
    .rounded(5.0)
    .fix_size(300.0, 200.0)
    .center();

    let popup2 = Container::new(
        Flex::column()
            .with_child(Label::new("This is a popup!").with_text_size(24.0))
            .with_spacer(20.0)
            .with_child(
                Button::new("Close").on_click(|_ctx, data: &mut InitialState, _env| {
                    data.show_popup = false;
                }),
            )
            .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
            .background(Color::MAROON),
    )
    .background(Color::GREEN)
    // .background(Color::rgb8(0x80, 0x80, 0x80))
    .border(Color::BLACK, 2.0)
    .rounded(5.0)
    .center()
    .fix_width(200.)
    .fix_height(200.);

    let conditional_popup = Either::new(
        |data: &InitialState, _env| data.show_popup,
        // Button::new("hi"),
        popup2,
        SizedBox::empty(),
    );

    ZStack::new(content)
        .with_centered_child(conditional_popup)
        .background(Color::RED)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .set_position(Point::new(300.0, 300.0))
        .show_titlebar(true)
        .title("Druid Gio Widgets")
        .window_size(Size::new(1200.0, 700.0));

    let initial_state = InitialState {
        counter: 0,
        dropdown_state: DropDownState {
            place: "California".to_owned(),
        },
        select_state: SelectState {
            place: "California".to_owned(),
        },
        show_popup: false,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
}
