extern crate gtk;
extern crate reqwest;
extern crate webkit2gtk;

mod miarka;

use gtk::prelude::*;
use gtk::{Label, Orientation, Window, WindowType};
use miarka::ZastepstwaIKomunikaty;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{sleep, spawn};
use std::time::Duration;
use webkit2gtk::{WebContext, WebView, WebViewExt};

fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    window.fullscreen();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let vbox = gtk::Box::new(Orientation::Vertical, 0);

    let label_error = Label::new("");
    vbox.add(&label_error);

    let hbox = gtk::Box::new(Orientation::Horizontal, 0);
    hbox.set_vexpand(true);

    let webview_komunikaty = make_webview();
    webview_komunikaty.set_hexpand(true);
    hbox.add(&webview_komunikaty);
    let webview_zastepstwa = make_webview();
    webview_zastepstwa.set_size_request(440, -1);
    hbox.add(&webview_zastepstwa);

    vbox.add(&hbox);

    window.add(&vbox);
    window.show_all();

    let rx = spawn_fetch_thread();
    loop {
        while let Ok(ev) = rx.try_recv() {
            match ev {
                Ok(page) => {
                    label_error.set_visible(false);
                    webview_komunikaty.load_html(&page.komunikaty(), None);
                    webview_zastepstwa.load_html(&page.zastepstwa(), None);
                }
                Err(err) => {
                    label_error.set_visible(true);
                    label_error.set_markup(&format!(
                        "<span foreground=\"red\" font-weight=\"bold\">Nie udało się wczytać {}: {}</span>",
                        miarka::FETCH_URI, err
                    ));
                }
            }
        }
        gtk::main_iteration_do(false);
        sleep(Duration::from_millis(4));
    }
}

fn spawn_fetch_thread() -> Receiver<Result<ZastepstwaIKomunikaty, String>> {
    let (tx, rx) = channel();
    spawn(move || {
        let mut old_page = ZastepstwaIKomunikaty::empty();
        loop {
            match ZastepstwaIKomunikaty::fetch() {
                Ok(page) => {
                    if old_page != page {
                        old_page = page;
                        tx.send(Ok(old_page.clone())).unwrap();
                    }
                }
                Err(err) => tx.send(Err(err.to_string())).unwrap(),
            }
            sleep(Duration::from_secs(15 * 60));
        }
    });
    rx
}

fn make_webview() -> WebView {
    let context = WebContext::get_default().unwrap();
    let webview = WebView::new_with_context(&context);
    webview
}
