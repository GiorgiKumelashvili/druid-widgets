use druid::widget::{Label, LabelText};
use druid::{
    Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget, theme,
};
use tracing::trace;

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct AdvancedButton<T> {
    label: Label<T>,
    label_size: Size,
}

impl<T: Data> AdvancedButton<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> AdvancedButton<T> {
        AdvancedButton {
            label: Label::new(text),
            label_size: Size::ZERO,
        }
    }
}

// custom button
//TODO figure out how to do cursor based on cursor.rs file
// cool guide - https://www.pauljmiller.com/posts/druid-widget-tutorial.html

impl<T: Data> Widget<T> for AdvancedButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                    trace!("Button {:?} released", ctx.widget_id());
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Button");
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        trace!("Computed button size: {}", button_size);
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // let is_active = ctx.is_active() && !ctx.is_disabled();
        // let is_hot = ctx.is_hot();
        let size = ctx.size();

        let stroke_width = if ctx.is_active() {
            4.0
        } else {
            env.get(theme::BUTTON_BORDER_WIDTH)
        };

        // let stroke_width = ctx.is_active() ? 2.0 : env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        // let bg_gradient = if ctx.is_disabled() {
        //     LinearGradient::new(
        //         UnitPoint::TOP,
        //         UnitPoint::BOTTOM,
        //         (
        //             env.get(theme::DISABLED_BUTTON_LIGHT),
        //             env.get(theme::DISABLED_BUTTON_DARK),
        //         ),
        //     )
        // } else if is_active {
        //     LinearGradient::new(
        //         UnitPoint::TOP,
        //         UnitPoint::BOTTOM,
        //         (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
        //     )
        // } else {
        //     LinearGradient::new(
        //         UnitPoint::TOP,
        //         UnitPoint::BOTTOM,
        //         (env.get(theme::BUTTON_LIGHT), env.get(theme::BUTTON_DARK)),
        //     )
        // };

        let main_border_color = if ctx.is_focused() || ctx.has_focus() || ctx.is_active() {
            // #3474f0
            &Color::rgb8(52, 116, 240)
        } else {
            &Color::rgb8(81, 83, 85)
        };

        ctx.stroke(rounded_rect, main_border_color, stroke_width);

        ctx.fill(rounded_rect, &Color::rgb8(43, 45, 48));

        ctx.with_save(|ctx| {
            let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
