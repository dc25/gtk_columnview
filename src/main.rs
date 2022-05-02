use gtk::glib;
use glib::{closure, Object};
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, ColumnView, ColumnViewColumn, Label, PolicyType, SignalListItemFactory, ScrolledWindow, SingleSelection, StringList, StringObject, Widget};


fn build_ui(app: &Application) {

    let model: StringList = (0..=100000)
        .into_iter()
        .map(|n| n.to_string())
        .collect();

    let selection_model = SingleSelection::new(Some(&model));
    let column_view = ColumnView::new(Some(&selection_model));

    // removed the activated (double clicked) item.
    column_view.connect_activate(move |col_view, index| { 
        let selmodel = col_view.model().ok_or(()).expect("Expected Some(model)"); 
        let singleselmodel = selmodel.downcast::<SingleSelection>().expect("Must be a SingleSlection");
        let listmodel = singleselmodel.model().ok_or(()).expect("Expected Some(listmodel");
        let stringlist = listmodel.downcast::<gtk::StringList>().expect("Must be a StringList");
        stringlist.remove(index);
    });

    let factory1 = SignalListItemFactory::new();
    factory1.connect_setup(move |_, list_item| {
        let label = Label::builder()
            .build();
        list_item.set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let column_view_column = ColumnViewColumn::new(Some("column1"), Some(&factory1));
    column_view.append_column(&column_view_column);

    let factory2 = SignalListItemFactory::new();
    factory2.connect_setup(move |_, list_item| {
        let label = Label::builder()
            .build();
        list_item.set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .chain_closure::<String>(closure!(|_: Option<Object>, s: String| { format!("Wow! {} HEY!!", s) }))
            .bind(&label, "label", Widget::NONE);
    });

    let column_view_column2 = ColumnViewColumn::new(Some("column2"), Some(&factory2));
    column_view.append_column(&column_view_column2);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&column_view)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    window.present();
}

fn main() {
    let app = Application::builder().build();
    app.connect_activate(build_ui);
    app.run();
}

