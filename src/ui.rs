use druid::{
    widget::{ControllerHost, CrossAxisAlignment, Flex, Label},
    Widget, WidgetExt,
};

use crate::data::{TimeController, TimeState};

pub fn build_root_widget() -> impl Widget<TimeState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(16.0)
        .lens(TimeState::time)
        .padding(5.0);

    let display = ControllerHost::new(display, TimeController);

    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .cross_axis_alignment(CrossAxisAlignment::Start)
}
