import Foundation
import Network
import AppKit
import WebKit

struct Request: Decodable {
    let event: String
    let body: String?
	let key: String
}

let app = NSApplication.shared

class AppDelegate: NSObject, NSApplicationDelegate, NSWindowDelegate {
	var key = ""
	
	let window = NSWindow.init(contentRect: NSRect(x: 0, y: 0, width: 750, height: 600), styleMask: [
		NSWindow.StyleMask.titled,
		NSWindow.StyleMask.closable,
		NSWindow.StyleMask.resizable,
		NSWindow.StyleMask.miniaturizable
	], backing: NSWindow.BackingStoreType.buffered, defer: false)
	
	var webview: Optional<WKWebView> = Optional.none

	func receive(connection: NWConnection) {
		connection.receive(minimumIncompleteLength: 1, maximumLength: 65536) { (message, _, done, error) in
			if let message = message, !message.isEmpty {
				let decoder = JSONDecoder()
			
				guard let request = try? decoder.decode(Request.self, from: message) else {
					print("request: invalid json")
				
					connection.cancel()
					return
				} 
				
				if (request.key != self.key) {
					print("invalid key for window")
					
					connection.cancel()
					return
				}
				
				print("request: ", request)
				
				switch request.event {
				case "hide":
					app.hide(nil)
					
				case "show":
					app.unhide(nil)
					
			       	self.window.makeKeyAndOrderFront(nil)
					self.window.orderFrontRegardless()
					
				case "set_title":
					self.window.title = request.body!
					
				case "set_body":
				   	self.webview!.loadHTMLString(request.body!, baseURL: URL(string: "about:blank"))
					
				case "close":
					app.terminate(self)
					return
				
				default: 
					print("unknown command", request.event)
					
				}
			}
		
			if done || error != nil {
				connection.cancel()
				return
			}
		
			self.receive(connection: connection)
		}
	}
	
	func direct(connection: NWConnection) {
		print("new client:", connection.endpoint)
	
		connection.start(queue: .main)
	
		receive(connection: connection)
	
	    connection.send(content: "Hello World".data(using: .utf8), completion: .contentProcessed({ plain in
			if let error = plain {
				print("error:", error)
			}
	    }))
	}

	func resize(width: Int, height: Int) {
		var frame = self.window.frame
		
		frame.size = NSSize(width: width, height: height)
		
		self.window.setFrame(frame, display: true)
	}

    func applicationDidFinishLaunching(_ notification: Notification) {
		if (CommandLine.arguments.count < 2) {
			print("port argument required, example: ./webkitd 1234 access_key\r\n")
			
			app.terminate(self)
			return
		}
	
		guard let plainPort = UInt16(CommandLine.arguments[1]) else {
			print("port", CommandLine.arguments[1], "is invalid")
			
			app.terminate(self)
			return
		}
		
		if (CommandLine.arguments.count < 3) {
			print("access key argument required, example: ./webkitd 1234 access_key\r\n")
			
			app.terminate(self)
			return
		}
		
		if (CommandLine.arguments.count == 5) {
			guard let height = Int(CommandLine.arguments[3]) else {
				print("height is invalid")
				
				app.terminate(self)
				return
			}
			
			guard let width = Int(CommandLine.arguments[4]) else {
				print("width is invalid")
				
				app.terminate(self)
				return
			}
			
			self.resize(width: width, height: height)
		}
		
		key = CommandLine.arguments[2]
		
       	window.makeKeyAndOrderFront(nil)
		window.orderFrontRegardless()
		window.center()
		
		app.hide(nil)

		window.delegate = self
		window.title = "Loading..."

	   	webview = WKWebView(frame: window.contentView!.frame)
	   	let request = URLRequest(url: URL(string: "about:blank")!)

	   	window.contentView?.addSubview(webview!)
	   	webview!.load(request)
		
		DispatchQueue(label: "background").async {
			let port = NWEndpoint.Port(rawValue: plainPort)!
			let listener = try! NWListener(using: .tcp, on: port)
			
			listener.newConnectionHandler = self.direct(connection:)
			listener.start(queue: .main)
			
			RunLoop.current.run()
		}
    }

	func windowWillClose(_ notification: Notification) {
		app.terminate(self)
	}
	
	func windowDidResize(_ notification: Notification) {
		self.webview!.frame.size.height = self.window.frame.size.height - 28.5
		self.webview!.frame.size.width = self.window.frame.size.width
	}
}

let delegate = AppDelegate()

app.delegate = delegate

app.run()