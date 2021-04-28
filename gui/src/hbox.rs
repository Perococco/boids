use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider, WidgetSpecific};
use crate::gui::{Gui};
use raylib::core::drawing::RaylibDrawHandle;
use crate::size::{Size};
use raylib::math::Vector2;
use std::cell::Cell;
use crate::fill::Fill;
use raylib::prelude::*;

pub struct HBoxPar {
    widget_data: WidgetData,
    spacing: Cell<f32>,
}

impl HBoxPar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new(), spacing: Cell::new(10.0) }
    }

    pub fn set_spacing(&self, gui: &Gui, spacing: f32) -> &HBoxPar {
        if spacing.eq(&self.spacing.get()) {
            return self;
        }
        self.spacing.set(spacing);
        self.widget_data.invalidate_preferred_size(gui);
        self
    }

    pub fn get_spacing(&self) -> f32 {
        self.spacing.get()
    }
}

impl WidgetDataProvider for HBoxPar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}

impl WidgetSpecific for HBoxPar {

    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return Size::empty();
        }

        let tree_index = tree_index.unwrap();

        let mut nb_children = 0;
        let mut max_height: f32 = 0.0;
        let mut summed_width: f32 = 0.0;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let child_computed_size = child.get_computed_size(gui);
                nb_children += 1;
                println!("comp {:?} {}",child_index,child_computed_size.width());
                max_height = max_height.max(child_computed_size.height());
                summed_width += child_computed_size.width();
            }
        }
        let spacing = self.spacing.get();
        summed_width += spacing * ((nb_children - 1).max(0) as f32);

        println!("sum+spac {}",summed_width);

        let computed = Size::new( summed_width, max_height).with_padding(&self.widget_data().model.padding.get());



        let mut preferred = self.widget_data.model.preferred_size.get();
        preferred.replace_empty_dimensions_and_max(&computed);

        return preferred.clone();
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let mut summed_fixed_width:f32 = 0.0;
        let mut summed_weight:u32 = 0;
        let mut nb_children = 0;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let fill = child.widget_data().fill_width();
                nb_children+=1;
                match fill {
                    Fill::Disabled => {
                        summed_fixed_width += child.get_computed_size(gui).width();
                    }
                    Fill::Enabled { weight} => {
                        summed_weight += weight;
                    }
                }
            }
        }

        println!("sum+fix {}",summed_fixed_width);


        let padding = self.widget_data.model.padding.get();
        let width = available_size.width() - padding.h_padding();
        let height= available_size.height() - padding.v_padding();

        let space_taken_by_spacing = self.spacing.get() * ((nb_children - 1).max(0) as f32);

        let fill_width = (width - space_taken_by_spacing - summed_fixed_width)/(summed_weight.max(1) as f32);

        if width<0.0 || height<=0.0 {
            return
        }

        let mut size = Size::new(0.0,height);
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let fill = child.widget_data().fill_width();
                match fill {
                    Fill::Disabled => {
                        let child_width = child.get_computed_size(gui).width();
                        size.set_width(child_width);
                        child.update_content_size(gui,&size);
                    }
                    Fill::Enabled { weight } => {
                        size.set_width(fill_width * weight as f32);
                        child.update_content_size(gui, &size)
                    }
                }
            }
        }

    }

    fn compute_child_positions(&self, gui: &Gui) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let content_size = {
            let content_layout = self.widget_data().geometry.content_layout.get();
            Size::new(content_layout.width, content_layout.height)
        };

        let spacing = self.spacing.get();

        let mut position = Vector2::new(0.0,0.0);
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                {
                    let borrow_widget_size = w.widget_data().geometry.widget_size.borrow();
                    position.y = (content_size.height() - borrow_widget_size.size().height())*0.5;
                    w.widget_data().set_widget_target(&position);
                    w.update_child_positions(gui);
                }
                let borrowed_widget_layout = w.widget_data().geometry.widget_layout.get();

                position.x += borrowed_widget_layout.width + spacing;
            }
        }
    }
}

impl RenderableWidget for HBoxPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        self.widget_data.render_background_and_border(d, &offset);

        let mut content_layout = self.widget_data.geometry.content_layout.get();
        let mut target = offset.clone();
        target.x += content_layout.x;
        target.y += content_layout.y;



        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.render(gui,d,&target);
            }
        }
        // {
        //     content_layout.x += offset.x;
        //     content_layout.y += offset.y;
        //     d.draw_rectangle_lines_ex(content_layout,1,Color::GREEN);
        // }


    }
}