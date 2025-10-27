use druid::widget::{Button, Click, ControllerHost, WidgetExt};
use druid::{Data, EventCtx, Lens, Widget};

use crate::widgets::select::{SELECT_SHOW, Select};

#[derive(Debug, Data, Clone, Lens, Default)]
pub struct SelectState {
    pub place: String,
}

pub fn main_widget_select() -> impl Widget<SelectState> {
    let select_button: ControllerHost<Button<String>, Click<String>> =
        Button::new("Select place from select")
            .on_click(|ctx: &mut EventCtx, _, _| ctx.submit_notification(SELECT_SHOW));

    // let places: Vec<(&'static str, String)> = vec!["England", "San Tropez", "Antarctica"]
    //     .into_iter()
    //     .map(|item| (item, item.to_owned()))
    //     .collect();

    Select::new(select_button)
        // Select::new(dropdown_button, move |_, _| {
        //     Flex::column().with_child(RadioGroup::row(places.clone()).align_vertical(UnitPoint::CENTER))
        // })
        .align_left()
        .lens(SelectState::place)
}
