FROM archlinux

RUN pacman-key --init
RUN pacman -Sy --noconfirm archlinux-keyring
RUN pacman -Syyu --noconfirm base-devel git 
RUN pacman -S --noconfirm neovim vim emacs gdb 

RUN git clone https://github.com/longld/peda.git ~/peda
RUN echo "source ~/peda/peda.py" >> $HOME/.gdbinit

RUN echo "alias asm2bin=\"f(){ \gcc \\\$1 -O0 -g -o tmp && ./tmp; }; f\"" >> $HOME/.bashrc
RUN echo "alias gcc=\"echo gcc command is disabled. use asm2bin command.\"" >> $HOME/.bashrc

WORKDIR /root
