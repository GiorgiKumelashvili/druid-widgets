use druid::widget::Label;
use druid::{
    AppLauncher, Data, Env, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx, Widget, WidgetExt,
    WindowDesc,
};

#[derive(Clone, Data, Default)]
struct AppState {}

// A wrapper widget to handle the macOS title bar setup.
struct MacosTitlebarSetup<W: Widget<AppState>> {
    inner: W,
}

impl<W: Widget<AppState>> MacosTitlebarSetup<W> {
    fn new(inner: W) -> Self {
        MacosTitlebarSetup { inner }
    }
}

impl<W: Widget<AppState>> Widget<AppState> for MacosTitlebarSetup<W> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        // --- NEW CODE START ---
        // Check if the event is a window size change event.
        if let druid::Event::WindowSize(new_size) = event {
            // Print the new size to the console.
            println!("Window size changed to: {:?}", new_size);
        }
        // --- NEW CODE END ---

        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSEvent, NSWindow, NSWindowStyleMask};
                use cocoa::base::{id, nil};
                use druid::{HasRawWindowHandle, RawWindowHandle};

                let window = ctx.window();
                window.show_titlebar(false);

                let raw_handle = window.raw_window_handle();
                if let RawWindowHandle::AppKit(appkit_handle) = raw_handle {
                    // Your key insight: ns_view is valid, but ns_window is not.
                    // So, we get the window from the view.
                    if !appkit_handle.ns_view.is_null() {
                        unsafe {
                            let ns_view = appkit_handle.ns_view as id;

                            // This is the crucial step: ask the view for its parent window.
                            let ns_window: id = ns_view.window();

                            // Safety check: make sure we got a valid window pointer back.
                            if ns_window != nil {
                                ns_window.setTitlebarAppearsTransparent_(cocoa::base::YES);

                                let mut style_mask = ns_window.styleMask();
                                style_mask
                                    .insert(NSWindowStyleMask::NSFullSizeContentViewWindowMask);
                                ns_window.setStyleMask_(style_mask);
                            }
                        }
                    }
                }
            }
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

fn build_ui() -> impl Widget<AppState> {
    Label::new("Hello world! The title bar is transparent.")
}

fn main() {
    let main_widget = MacosTitlebarSetup::new(build_ui());

    let window = WindowDesc::new(main_widget)
        .window_size((600.0, 400.0))
        .title("");

    AppLauncher::with_window(window)
        .launch(AppState::default())
        .expect("launch failed");
}
