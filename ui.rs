use gtk4::{glib::GString,Application,Notebook ,Grid,Label,DropDown,ApplicationWindow,Button,Entry,prelude::*};
pub fn ui() -> glib::ExitCode{
    let app = Application::builder()
        .application_id("RustyAuto")
        .build();
    app.connect_activate(|app| {
        let main = ApplicationWindow::builder().application(app).default_width(200).default_height(200).title("RuStyAuto").build();
        let book = Notebook::new();
        let button = Button::with_label("start");
        button.connect_clicked(|button|{
            let s:Option<GString> = Some(GString::from("stop"));
            if button.label() == s {
                button.set_label("start")
            }else {
                button.set_label("stop")
            }
        });
        let about = Label::new(Some("About"));
        let page1 = Label::builder().label("AutoClicker").build();
        let page2 = Label::builder().label("About").build();
        let clicks = Entry::builder().placeholder_text("10").build();
        let list = ["hour","min","sec"];
        let clicks_per = DropDown::from_strings(&list);
        let perlabel = Label::new(Some("Clicks Per"));
        let grid = Grid::builder().margin_start(6).margin_end(6).margin_top(6).margin_bottom(6).halign(gtk4::Align::Fill).valign(gtk4::Align::Fill).row_spacing(6).column_spacing(6).build();
        book.append_page(&grid, Some(&page1));
        book.append_page(&about, Some(&page2));
        grid.attach(&clicks_per, 2, 0, 1, 1);
        grid.attach(&perlabel, 1, 0, 1, 1);
        grid.attach(&clicks, 0, 0,1,1);
        grid.attach(&button,1,1,1,1);

        main.set_child(Some(&book));
        main.show()
    });

    app.run()
}