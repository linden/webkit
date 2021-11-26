MacOS:
	cd window/MacOS && swiftc index.swift && mv index ../../bin/webkitd
	cd ../..

build_examples:
	cd client/rust && \
	cargo build --examples && \
	mv target/debug/examples/window ../../bin/window && \
	mv target/debug/examples/two_windows ../../bin/two_windows
	
	cd ../..

run_example_window:
	cd bin && ./window
	
run_example_two_windows:
	cd bin && ./two_windows
	
window: MacOS build_examples run_example_window

windows: MacOS build_examples run_example_two_windows