example:
	cd window/MacOS && swiftc index.swift && mv index ../../bin/webkitd
	cd ../..
	cd client/rust && cargo build --examples && pwd && mv target/debug/examples/window ../../bin/window
	cd ../..

	cd bin && ./window