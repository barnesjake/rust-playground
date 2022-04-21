use druid::widget::{Label, Flex, Padding, Align, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top middle"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom middle")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom right")), 1.0)
                    .with_flex_child(Button::new("add one").on_click(|_ctx, data, _env| *data += 1).padding(5.0), 1.0),
                1.0)
                
    )
}

fn build_ui2() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left2"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left2")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top middle2"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom middle2")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right2"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom right2")), 1.0),
                    1.0)
    )
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
    
}

// fn main() -> Result<(), PlatformError> {
//     let main_window = WindowDesc::new(ui_builder);
//     let data = 0_u32;
//     AppLauncher::with_window(main_window)
//         .use_simple_logger()
//         .launch(data)
// }

// fn ui_builder() -> impl Widget<u32> {
//     // The label text will be computed dynamically based on the current locale and count
//     let text =
//         LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
//     let label = Label::new(text).padding(5.0).center();
//     let button = Button::new("add one")
//         .on_click(|_ctx, data, _env| *data += 1)
//         .padding(5.0);

//     Flex::column().with_child(label).with_child(button)
// }