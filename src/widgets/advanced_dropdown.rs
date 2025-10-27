use druid::widget::{Button, Click, ControllerHost, Flex, RadioGroup, WidgetExt};
use druid::{Data, EventCtx, Lens, UnitPoint, Widget};

use crate::widgets::dropdown::{DROPDOWN_SHOW, Dropdown};

#[derive(Debug, Data, Clone, Lens, Default)]
pub struct DropDownState {
    pub place: String,
}

pub fn main_widget_dropdown() -> impl Widget<DropDownState> {
    let dropdown_button: ControllerHost<Button<String>, Click<String>> =
        Button::new("Select place")
            .on_click(|ctx: &mut EventCtx, _, _| ctx.submit_notification(DROPDOWN_SHOW));

    let places: Vec<(&'static str, String)> = vec!["England", "San Tropez", "Antarctica"]
        .into_iter()
        .map(|item| (item, item.to_owned()))
        .collect();

    Dropdown::new(dropdown_button, move |_, _| {
        Flex::column().with_child(RadioGroup::row(places.clone()).align_vertical(UnitPoint::CENTER))
    })
    .align_left()
    .lens(DropDownState::place)
}
