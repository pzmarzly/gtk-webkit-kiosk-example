# Deploying on Raspberry Pi 0W

## Preparation

2018-11-13-raspbian-stretch-lite.img

On `/boot`, create empty `ssh` file and `wpa_supplicant.conf` file with:

```text
country=US
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1

network={
 ssid="WIFI_SSID"
 scan_ssid=1
 psk="WIFI_PASSWORD"
}
```

SSH with `pi:raspberry`, change `passwd`.

## Installation

```bash
sudo bash
apt update
apt upgrade -y
apt install -y git rsync gawk busybox bindfs i3 libgtk-3-dev x11-xserver-utils unclutter libssl-dev libsoup2.4-dev
exit
cd
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
git clone https://github.com/pzmarzly/kiosk-miarka --depth 1
cd kiosk-miarka
cargo build
cd deploy
cp config ~/.config/i3/config
cp xinitrc ~/.xinitrc
sudo bash
dphys-swapfile swapoff
dphys-swapfile uninstall
update-rc.d dphys-swapfile disable
systemctl disable dphys-swapfile
git clone https://github.com/josepsanzcamp/root-ro.git --depth 1
rsync -va root-ro/etc/initramfs-tools/* /etc/initramfs-tools/
mkinitramfs -o /boot/initrd.gz
echo initramfs initrd.gz >> /boot/config.txt
exit
```

TODO: X -s 0 -dpms

## References

https://github.com/josepsanzcamp/root-ro

https://www.losant.com/blog/getting-started-with-the-raspberry-pi-zero-w-without-a-monitor

https://github.com/pzmarzly/xephyr-now/blob/102fdf5c53b32f5946a20669c52df99c510ef0ea/i3config

https://faq.i3wm.org/question/2885/disable-titlebar-completely.1.html

https://blogs.wcode.org/2013/09/howto-boot-your-raspberry-pi-into-a-fullscreen-browser-kiosk/
