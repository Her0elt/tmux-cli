install:
	cargo build --release
	cp ./target/release/main ~/.local/bin/tmux-cli

clean-install:
	rm ~/.local/bin/tmux-cli
	make install

