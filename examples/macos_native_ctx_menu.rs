use druid::widget::{Button, Container, Flex};
use druid::{
    AppLauncher, Color, Data, Lens, PlatformError, Point, Size, Widget, WidgetExt, WindowDesc,
};

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl}; // ✅ these macros must be imported!

#[derive(Debug, Clone, Data, Lens)]
pub struct InitialState {}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
unsafe fn show_native_menu_at_mouse() {
    unsafe {
        register_menu_handler_class();

        let menu: id = msg_send![class!(NSMenu), alloc];
        let menu: id = msg_send![menu, initWithTitle: NSString::alloc(nil).init_str("")];
        // let menu: id = msg_send![menu, initWithTitle: NSString::alloc(nil).init_str("")];

        // Create our handler object
        let handler_class = class!(RustMenuHandler);
        let handler: id = msg_send![handler_class, alloc];
        let handler: id = msg_send![handler, init];

        // Create item 1
        let item1_title = NSString::alloc(nil).init_str("Say hi");
        let item1: id = msg_send![class!(NSMenuItem), alloc];
        let item1: id = msg_send![item1,
            initWithTitle: item1_title
            action: sel!(menuItemClicked:)
            keyEquivalent: NSString::alloc(nil).init_str("")
        ];
        let _: () = msg_send![item1, setTarget: handler];
        let _: () = msg_send![menu, addItem: item1];

        // Create item 2
        let item2_title = NSString::alloc(nil).init_str("Say bye");
        let item2: id = msg_send![class!(NSMenuItem), alloc];
        let item2: id = msg_send![item2,
            initWithTitle: item2_title
            action: sel!(menuItemClicked:)
            keyEquivalent: NSString::alloc(nil).init_str("")
        ];
        let _: () = msg_send![item2, setTarget: handler];
        let _: () = msg_send![menu, addItem: item2];

        // Mouse location (screen coordinates)
        let mouse_loc: cocoa::foundation::NSPoint = msg_send![class!(NSEvent), mouseLocation];

        // Show menu
        let _: () = msg_send![menu,
            popUpMenuPositioningItem: nil
            atLocation: mouse_loc
            inView: nil
        ];
    }
}

/// Registers a small Objective-C class called `RustMenuHandler`
/// with one method: `menuItemClicked:`
#[cfg(target_os = "macos")]
// #[allow(unexpected_cfgs)]
unsafe fn register_menu_handler_class() {
    unsafe {
        use objc::{
            declare::ClassDecl,
            runtime::{Object, Sel},
        };

        static mut REGISTERED: bool = false;

        if REGISTERED {
            return;
        }

        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RustMenuHandler", superclass).unwrap();

        // Add the `menuItemClicked:` method
        extern "C" fn menu_item_clicked(_this: &Object, _sel: Sel, item: id) {
            let title: id = unsafe { msg_send![item, title] };
            let cstr: *const std::os::raw::c_char = unsafe { msg_send![title, UTF8String] };
            let rust_str = unsafe { std::ffi::CStr::from_ptr(cstr) }.to_string_lossy();

            println!("👉 Menu item clicked: {}", rust_str);
        }

        decl.add_method(
            sel!(menuItemClicked:),
            menu_item_clicked as extern "C" fn(&Object, Sel, id),
        );
        decl.register();
        REGISTERED = true;
    }
}

fn ui_builder() -> impl Widget<InitialState> {
    // Button to toggle the popup
    let menu_button =
        Button::new("Menu Button").on_click(|_ctx, _data: &mut InitialState, _env| {
            // call our macOS menu show function
            #[cfg(target_os = "macos")]
            unsafe {
                show_native_menu_at_mouse();
            }
        });

    // Button to toggle the popup
    let menu_button2 =
        Button::new("Menu Button").on_click(|_ctx, _data: &mut InitialState, _env| {
            // call our macOS menu show function
            #[cfg(target_os = "macos")]
            unsafe {
                show_native_menu_at_mouse();
            }
        });

    Flex::column()
        .with_child(menu_button)
        .with_default_spacer()
        .with_spacer(400.0)
        .with_default_spacer()
        .with_child(menu_button2)
        .with_default_spacer()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .main_axis_alignment(druid::widget::MainAxisAlignment::Start)
        .padding(20.0)
        .background(Color::TRANSPARENT)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .set_position(Point::new(300.0, 300.0))
        .show_titlebar(true)
        .title("Macos Native Ctx Menu")
        .window_size(Size::new(1200.0, 700.0));

    let initial_state = InitialState {};

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
}
