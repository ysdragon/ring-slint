aPackageInfo = [
	:name = "Ring Slint",
	:description = "Ring bindings for Slint. Build beautiful, native applications with Ring and Slint.",
	:folder = "ring-slint",
	:developer = "ysdragon",
	:email = "youssefelkholey@gmail.com",
	:license = "MIT",
	:version = "1.0.0",
	:ringversion = "1.25",
	:versions = 	[
		[
			:version = "1.0.0",
			:branch = "master"
		]
	],
	:libs = 	[
		[
			:name = "",
			:version = "",
			:providerusername = ""
		]
	],
	:files = 	[
		"lib.ring",
		"main.ring",
		"README.md",
		"examples/01_hello.ring",
		"examples/01_hello.slint",
		"examples/02_counter.ring",
		"examples/02_counter.slint",
		"examples/03_properties.ring",
		"examples/03_properties.slint",
		"examples/04_loadstring.ring",
		"examples/05_callbacks.ring",
		"examples/05_callbacks.slint",
		"examples/06_invoke.ring",
		"examples/06_invoke.slint",
		"examples/07_globals.ring",
		"examples/07_globals.slint",
		"examples/08_types.ring",
		"examples/08_types.slint",
		"examples/09_timer.ring",
		"examples/09_timer.slint",
		"examples/10_temperature.ring",
		"examples/10_temperature.slint",
		"examples/11_fonts.ring",
		"examples/11_fonts.slint",
		"examples/12_images.ring",
		"examples/12_images.slint",
		"examples/images/photo1.jpg",
		"examples/images/photo2.jpg",
		"examples/images/photo3.jpg",
		"examples/images/photo4.jpg",
		"examples/images/photo5.jpg",
		"examples/13_todo.ring",
		"examples/13_todo.slint",
		"examples/14_stopwatch.ring",
		"examples/14_stopwatch.slint",
		"examples/15_calculator.ring",
		"examples/15_calculator.slint",
		"examples/16_custom_window.ring",
		"examples/16_custom_window.slint",
		"examples/17_file_dialogs.ring",
		"examples/17_file_dialogs.slint",
		"examples/18_window_management.ring",
		"examples/18_window_management.slint",
		"examples/19_desktop_features.ring",
		"examples/19_desktop_features.slint",
		"examples/20_notes.ring",
		"examples/20_notes.slint",
		"src/slint.ring",
		"src/rust_src/Cargo.toml",
		"src/rust_src/.gitignore",
		"src/rust_src/src/android.rs",
		"src/rust_src/src/lib.rs",
		"src/rust_src/src/slint/tray.rs",
		"src/rust_src/src/slint/notification.rs",
		"src/rust_src/src/slint/hotkey.rs",
		"src/rust_src/src/slint/component.rs",
		"src/rust_src/src/slint/interpreter.rs",
		"src/rust_src/src/slint/callback.rs",
		"src/rust_src/src/slint/model.rs",
		"src/rust_src/src/slint/mod.rs",
		"src/rust_src/src/slint/clipboard.rs",
		"src/rust_src/src/slint/dialogs.rs",
		"src/rust_src/src/slint/value.rs",
		"src/rust_src/src/slint/timer.rs",
		"src/utils/uninstall.ring",
		"src/utils/color.ring",
		"src/utils/install.ring",
		"LICENSE"
	],
	:ringfolderfiles = 	[

	],
	:windowsfiles = 	[
		"lib/windows/amd64/ring_slint.dll",
		"lib/windows/i386/ring_slint.dll",
		"lib/windows/arm64/ring_slint.dll"
	],
	:linuxfiles = 	[
		"lib/linux/amd64/libring_slint.so",
		"lib/linux/arm64/libring_slint.so"
	],
	:ubuntufiles = 	[

	],
	:fedorafiles = 	[

	],
	:macosfiles = 	[
		"lib/macos/amd64/libring_slint.dylib",
		"lib/macos/arm64/libring_slint.dylib"
	],
	:freebsdfiles = 	[
		"lib/freebsd/amd64/libring_slint.so"
	],
	:windowsringfolderfiles = 	[

	],
	:linuxringfolderfiles = 	[

	],
	:ubunturingfolderfiles = 	[

	],
	:fedoraringfolderfiles = 	[

	],
	:freebsdringfolderfiles = 	[

	],
	:macosringfolderfiles = 	[

	],
	:run = "ring main.ring",
	:windowsrun = "",
	:linuxrun = "",
	:macosrun = "",
	:ubunturun = "",
	:fedorarun = "",
	:setup = "ring src/utils/install.ring",
	:windowssetup = "",
	:linuxsetup = "",
	:macossetup = "",
	:ubuntusetup = "",
	:fedorasetup = "",
	:remove = "ring src/utils/uninstall.ring",
	:windowsremove = "",
	:linuxremove = "",
	:macosremove = "",
	:ubunturemove = "",
	:fedoraremove = "",
    :remotefolder = "ring-slint",
    :branch = "master",
    :providerusername = "ysdragon",
    :providerwebsite = "github.com"
]