use std::cell::RefCell;
use std::rc::Rc;



use crate::label::LabelPar;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};
use crate::widget_operation::{Size, RenderableWidget, LayoutableWidget};
use raylib::core::drawing::RaylibDrawHandle;
use crate::pane::PanePar;

pub enum  Widget {
    Label(LabelPar),
    Pane(PanePar)
}

impl SizeableWidget for Widget {
    fn compute_content_size(&self, available_space:&Size) -> Size {
        match &self {
            Widget::Label(p) => p.compute_content_size(available_space),
            Widget::Pane(p) => p.compute_content_size(available_space),
        }
    }
}

impl WidgetDataProvider for Widget {

    fn widget_data(&self) -> &WidgetData {
        match &self {
            Widget::Label(p) => p.widget_data(),
            Widget::Pane(p) => p.widget_data(),
        }
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        match self {
            Widget::Label(p) => p.widget_data_mut(),
            Widget::Pane(p) => p.widget_data_mut(),
        }
    }
}

impl RenderableWidget for Widget {
    fn render(&self, d: &mut RaylibDrawHandle) {
        match self {
            Widget::Label(p) => p.render(d),
            Widget::Pane(p) => p.render(d)
        }
    }
}