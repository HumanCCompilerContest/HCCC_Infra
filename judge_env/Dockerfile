# Dockerfile for judge server
FROM archlinux

RUN pacman-key --init
RUN pacman -Sy --noconfirm archlinux-keyring
RUN pacman -Syyu --noconfirm base-devel git 
RUN pacman -S --noconfirm neovim vim emacs gdb 



WORKDIR /root
