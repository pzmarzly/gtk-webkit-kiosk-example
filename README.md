# Fullscreen program displaying news @ my school

Abandoned, never finished (works but not fast enough for smooth playback).

Meant to be run on Raspberry Pi 3 or equivalent. Scraps website for news and shows them, slowly scrolling.

Uses GTK+ and WebKit2GTK.

Latest working version is at [842a8d3d20732e48e187478d4158f0b88ecfb54b](https://github.com/pzmarzly/kiosk-miarka/tree/842a8d3d20732e48e187478d4158f0b88ecfb54b), it uses Javascript to autoscroll inside WebView. I tried rewriting the program to cache image as GtkImages, but run into problems when making screenshot of pages. See an issue I opened at https://github.com/gtk-rs/cairo/issues/226 and all newer commits.

See [Deployment guide](deploy/README.md).
