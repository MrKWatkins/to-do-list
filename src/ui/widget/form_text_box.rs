use druid::{Widget, WidgetPod, Env, Data, EventCtx, PaintCtx, Size, BoxConstraints, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Event};
use crate::ui::widget::FormField;
use druid::widget::TextBox;
use crate::ui::{TEXT_COLOUR, TEXT_DISABLED_COLOUR};

pub struct FormTextBox {
    text_box: WidgetPod<String, TextBox<String>>
}

impl FormTextBox {
    pub fn new() -> FormTextBox {
        FormTextBox {
            text_box: WidgetPod::new(TextBox::new())
        }
    }
}

impl Widget<FormField<String>> for FormTextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut FormField<String>, env: &Env) {
        self.text_box.event(ctx, event, &mut data.value, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &FormField<String>, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.set_colours(data);
        }
        self.text_box.lifecycle(ctx, event, &data.value, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &FormField<String>, data: &FormField<String>, env: &Env) {
        self.text_box.update(ctx, &data.value, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &FormField<String>, env: &Env) -> Size {
        return self.text_box.layout(ctx, bc, &data.value, env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &FormField<String>, env: &Env) {
        self.text_box.paint(ctx, &data.value, env);
    }
}

impl FormTextBox {
    fn set_colours(&mut self, data: &FormField<String>)
    {
        let label = self.text_box.widget_mut();
        if data.is_enabled {
            label.set_text_color(TEXT_COLOUR);
        }
        else {
            label.set_text_color(TEXT_DISABLED_COLOUR);
        }
    }
}