install:
	cargo build --release
	cp ./target/release/tmux-cli ~/.local/bin/

clean-install:
	rm ~/.local/bin/tmux-cli
	make install

