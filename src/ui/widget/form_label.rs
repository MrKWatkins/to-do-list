use druid::{Widget, WidgetPod, Env, Data, EventCtx, PaintCtx, Size, BoxConstraints, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Event};
use crate::ui::widget::FormField;
use druid::widget::Label;
use crate::ui::{TEXT_COLOUR, TEXT_DISABLED_COLOUR};

pub struct FormLabel<T> {
    label: WidgetPod<FormField<T>, Label<FormField<T>>>
}

impl<T: Data> FormLabel<T> {
    pub fn new() -> FormLabel<T> {
        FormLabel {
            label: WidgetPod::new(Label::dynamic(|field: &FormField<T>, _env| field.label.to_string()))
        }
    }
}

impl<T: Data> Widget<FormField<T>> for FormLabel<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut FormField<T>, env: &Env) {
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &FormField<T>, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.set_colours(data);
        }
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &FormField<T>, data: &FormField<T>, env: &Env) {
        self.label.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &FormField<T>, env: &Env) -> Size {
        return self.label.layout(ctx, bc, data, env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &FormField<T>, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}

impl<T: Data> FormLabel<T> {
    fn set_colours(&mut self, data: &FormField<T>)
    {
        let label = self.label.widget_mut();
        if data.is_enabled {
            label.set_text_color(TEXT_COLOUR);
        }
        else {
            label.set_text_color(TEXT_DISABLED_COLOUR);
        }
    }
}