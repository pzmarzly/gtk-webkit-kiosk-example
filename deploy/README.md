# Deploying

Raspberry Pi 0W is too slow.

Tritium 512MB from libre.computer works.

## Preparation

Armbian_5.65_Tritium-h3_Ubuntu_bionic_next_4.14.78.7z [torrent](magnet:?xt=urn:btih:2a26966098a7a4f8040ed716acdc6f90c5d7da4a&dn=Armbian_5.65_Tritium-h3_Ubuntu_bionic_next_4.14.78.7z&tr=udp%3a%2f%2f62.138.0.158%3a6969%2fannounce&tr=udp%3a%2f%2f185.225.17.100%3a1337%2fannounce&tr=udp%3a%2f%2f51.15.4.13%3a1337%2fannounce&tr=udp%3a%2f%2f151.80.120.114%3a2710%2fannounce&tr=udp%3a%2f%2f208.83.20.20%3a6969%2fannounce&tr=udp%3a%2f%2f184.105.151.164%3a6969%2fannounce&tr=udp%3a%2f%2f51.15.76.199%3a6969%2fannounce&tr=udp%3a%2f%2f191.96.249.23%3a6969%2fannounce&tr=udp%3a%2f%2f5.79.83.194%3a6969%2fannounce&tr=udp%3a%2f%2f128.1.203.23%3a8080%2fannounce&tr=udp%3a%2f%2f51.15.40.114%3a80%2fannounce&tr=udp%3a%2f%2f89.234.156.205%3a451%2fannounce&tr=udp%3a%2f%2f5.206.28.90%3a6969%2fannounce&tr=udp%3a%2f%2f95.211.168.204%3a2710%2fannounce&tr=udp%3a%2f%2f51.38.184.185%3a6969%2fannounce&tr=udp%3a%2f%2f176.31.106.35%3a80%2fannounce&tr=udp%3a%2f%2f188.246.227.212%3a80%2fannounce&tr=udp%3a%2f%2f51.15.103.67%3a1337%2fannounce&tr=udp%3a%2f%2f51.15.215.89%3a6969%2fannounce&tr=udp%3a%2f%2f154.16.245.176%3a6969%2fannounce)

Unplug the display - voltage on HDMI drops during the installation

## Installation

```bash
sudo bash
apt update
apt upgrade -y
apt install -y git i3 libgtk-3-dev x11-xserver-utils unclutter libssl-dev libsoup2.4-dev libwebkit2gtk-4.0-dev xorg
#For Raspbian - left for future reference:
# nano /etc/dphys-swapfile # CONF_SWAPSIZE=700
# /etc/init.d/dphys-swapfile stop
# /etc/init.d/dphys-swapfile start
#For Armbian:
fallocate /var/swap.file --length 700MB
chmod 0600 /var/swap.file
mkswap /var/swap.file
swapon /var/swap.file
exit
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
git clone https://github.com/pzmarzly/kiosk-miarka --depth 1
cd kiosk-miarka
cargo build # debug build takes about an hour
cd deploy
mkdir ~/.config
mkdir ~/.config/i3
cp config ~/.config/i3/config
cp xinitrc ~/.xinitrc
sudo bash
#For Raspbian - left for future reference:
# dphys-swapfile swapoff
# dphys-swapfile uninstall
# update-rc.d dphys-swapfile disable
# systemctl disable dphys-swapfile
# git clone https://github.com/josepsanzcamp/root-ro.git --depth 1
# rsync -va root-ro/etc/initramfs-tools/* /etc/initramfs-tools/
# mkinitramfs -o /boot/initrd.gz
# echo initramfs initrd.gz >> /boot/config.txt
#For Armbian:
swapoff /var/swap.file
rm /var/swap.file
exit
```

TODO: X -s 0 -dpms

## References

https://github.com/josepsanzcamp/root-ro

https://www.losant.com/blog/getting-started-with-the-raspberry-pi-zero-w-without-a-monitor

https://github.com/pzmarzly/xephyr-now/blob/102fdf5c53b32f5946a20669c52df99c510ef0ea/i3config

https://faq.i3wm.org/question/2885/disable-titlebar-completely.1.html

https://blogs.wcode.org/2013/09/howto-boot-your-raspberry-pi-into-a-fullscreen-browser-kiosk/
