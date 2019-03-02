mod miarka;

use crate::miarka::ZastepstwaIKomunikaty;
use cairo::Surface;
use gtk::prelude::*;
use gtk::{Box, Builder, Image, Label, Orientation, Window, WindowType};
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{sleep, spawn};
use std::time::Duration;
use webkit2gtk::{LoadEvent, SnapshotOptions, SnapshotRegion, WebContext, WebView, WebViewExt};

fn main() {
    gtk::init().unwrap();

    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("window").unwrap();
    //TODO: window.fullscreen();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let label_error: Label = builder.get_object("error").unwrap();
    let box1: Box = builder.get_object("box1").unwrap();
    let box2: Box = builder.get_object("box2").unwrap();
    let img1: Image = builder.get_object("img1").unwrap();
    let img2: Image = builder.get_object("img2").unwrap();

    window.show_all();

    let webview_komunikaty = make_webview();
    webview_komunikaty.set_hexpand(true);
    box1.add(&webview_komunikaty);
    let webview_zastepstwa = make_webview();
    webview_zastepstwa.set_size_request(440, -1);
    box2.add(&webview_zastepstwa);

    let page_fetched_rx = spawn_fetch_thread();
    let (page_screenshot1_tx, page_screenshot1_rx) = channel();
    let (page_screenshot2_tx, page_screenshot2_rx) = channel();
    loop {
        while let Ok(ev) = page_fetched_rx.try_recv() {
            match ev {
                Ok(page) => {
                    label_error.set_visible(false);

                    let pstx1 = page_screenshot1_tx.clone();
                    let pstx2 = page_screenshot2_tx.clone();
                    let wvk = webview_komunikaty.clone();
                    let wvz = webview_zastepstwa.clone();
                    let img1 = img1.clone();
                    let img2 = img2.clone();

                    render(wvk, img1, page.komunikaty(), pstx1);
                    render(wvz, img2, page.zastepstwa(), pstx2);
                }
                Err(err) => {
                    label_error.set_visible(true);
                    label_error.set_markup(&format!(
                        "Nie udało się wczytać {}: {}",
                        miarka::FETCH_URI,
                        err
                    ));
                }
            }
        }
        while let Ok(ptr) = page_screenshot1_rx.try_recv() {
            img1.set_from_surface(Some(ptr));
            webview_zastepstwa.set_visible(false);
            img1.set_visible(true);
        }
        while let Ok(ptr) = page_screenshot2_rx.try_recv() {
            img2.set_from_surface(Some(ptr));
            webview_komunikaty.set_visible(false);
            img2.set_visible(true);
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
    webview.set_vexpand(true);
    webview
}

fn render(wv: WebView, img: Image, content: String, pstx: Sender<Surface>) {
    img.set_visible(false);
    wv.set_visible(true);
    wv.load_html(&content, None);
    let pstx = pstx.clone();
    wv.connect_load_changed(move |wv, ev| {
        if ev == LoadEvent::Finished {
            let pstx = pstx.clone();
            let img = img.clone();
            wv.get_snapshot(
                SnapshotRegion::FullDocument,
                SnapshotOptions::NONE,
                None,
                move |res| {
                    let pstx = pstx.clone();
                    if let Ok(surface) = res {
                        img.set_from_surface(Some(&surface));
                        // let ptr = AtomicPtr::new(&mut surface);
                        // pstx.send(surface).unwrap();
                        // std::thread::sleep(Duration::from_secs(15));
                    }
                },
            );
        }
    });
}
