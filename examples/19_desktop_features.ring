load "slint.ring"

nHotkeyId = 0
nTimerId = 0

oApp = new SlintApp {
    loadUI("19_desktop_features.slint")

    setCallback("send-notification", :onSendNotification)
    setCallback("send-notification-timeout", :onSendNotificationTimeout)
    setCallback("send-notification-icon", :onSendNotificationIcon)
    setCallback("send-notification-full", :onSendNotificationFull)
    setCallback("copy-to-clipboard", :onCopyToClipboard)
    setCallback("paste-from-clipboard", :onPasteFromClipboard)
    setCallback("clear-clipboard", :onClearClipboard)
    setCallback("toggle-always-on-top", :onToggleAlwaysOnTop)
    setCallback("register-hotkey", :onRegisterHotkey)
    setCallback("unregister-hotkey", :onUnregisterHotkey)
    setCallback("unregister-all-hotkeys", :onUnregisterAllHotkeys)
    setCallback("create-tray", :onCreateTray)
    setCallback("create-tray-icon", :onCreateTrayIcon)
    setCallback("set-tray-icon", :onSetTrayIcon)
    setCallback("set-tray-tooltip", :onSetTrayTooltip)
    setCallback("destroy-tray", :onDestroyTray)

    nTimerId = timerStart(100, :onPollEvents)

    show()
    run()
}

func onSendNotification
    # notify() - basic notification
    oApp.notify("Ring Slint", "Hello from Ring! This is a desktop notification.")
    oApp.setString("status-text", "Basic notification sent!")

func onSendNotificationTimeout
    # notifyWithTimeout() - notification with custom timeout (5 seconds = 5000ms)
    oApp.notifyWithTimeout("Timed Notification", "This will disappear in 5 seconds", 5000)
    oApp.setString("status-text", "Notification with 5s timeout sent!")

func onSendNotificationIcon
    # notifyWithIcon() - notification with custom icon
    cIconPath = currentdir() + "/images/ring.png"
    oApp.notifyWithIcon("Icon Notification", "This notification has a custom icon", cIconPath)
    oApp.setString("status-text", "Notification with icon sent!")

func onSendNotificationFull
    # notifyFull() - notification with icon and timeout (10 seconds = 10000ms)
    cIconPath = currentdir() + "/images/ring.png"
    oApp.notifyFull("Full Notification", "Custom icon + 10 second timeout", cIconPath, 10000)
    oApp.setString("status-text", "Full notification sent!")

func onCopyToClipboard
    cText = oApp.getProperty("clipboard-text")
    if cText != NULL and len(cText) > 0
        oApp.clipboardSet(cText)
        oApp.setString("status-text", "Copied to clipboard: " + cText)
    else
        oApp.setString("status-text", "Nothing to copy")
    ok

func onPasteFromClipboard
    cText = oApp.clipboardGet()
    if cText != NULL and len(cText) > 0
        oApp.setString("clipboard-text", cText)
        oApp.setString("status-text", "Pasted from clipboard")
    else
        oApp.setString("status-text", "Clipboard is empty or has no text")
    ok

func onClearClipboard
    oApp.clipboardClear()
    oApp.setString("clipboard-text", "")
    oApp.setString("status-text", "Clipboard cleared!")

func onToggleAlwaysOnTop
    bOnTop = oApp.getProperty("is-on-top")
    oApp.windowSetAlwaysOnTop(bOnTop)
    if bOnTop
        oApp.setString("status-text", "Window is now always on top")
    else
        oApp.setString("status-text", "Window is now normal")
    ok

func onRegisterHotkey
    if nHotkeyId = 0
        nHotkeyId = oApp.hotkeyRegister("ctrl+shift", "KeyH", :onHotkeyPressed)
        oApp.setString("status-text", "Hotkey registered: Ctrl+Shift+H (ID: " + nHotkeyId + ")")
    else
        oApp.setString("status-text", "Hotkey already registered")
    ok

func onUnregisterHotkey
    if nHotkeyId != 0
        oApp.hotkeyUnregister(nHotkeyId)
        oApp.setString("status-text", "Hotkey unregistered")
        nHotkeyId = 0
    else
        oApp.setString("status-text", "No hotkey to unregister")
    ok

func onUnregisterAllHotkeys
    # hotkeyUnregisterAll() - unregister all hotkeys
    oApp.hotkeyUnregisterAll()
    nHotkeyId = 0
    oApp.setString("status-text", "All hotkeys unregistered!")

func onHotkeyPressed
    oApp.notify("Hotkey", "Ctrl+Shift+H was pressed!")
    oApp.setString("status-text", "Hotkey pressed!")

func onCreateTray
    # trayCreate() - create tray with tooltip only
    oApp.trayCreate("Ring Slint App")
    oApp.trayAddItem("Show Window", :onTrayShow)
    oApp.trayAddItem("Send Notification", :onTraySendNotification)
    oApp.trayAddSeparator()
    oApp.trayAddItem("Quit", :onTrayQuit)
    oApp.setBool("tray-active", true)
    oApp.setString("status-text", "System tray created")

func onCreateTrayIcon
    # trayCreateWithIcon() - create tray with tooltip and icon
    cIconPath = currentdir() + "/images/ring.png"
    oApp.trayCreateWithIcon("Ring Slint App", cIconPath)
    oApp.trayAddItem("Show Window", :onTrayShow)
    oApp.trayAddItem("Send Notification", :onTraySendNotification)
    oApp.trayAddSeparator()
    oApp.trayAddItem("Quit", :onTrayQuit)
    oApp.setBool("tray-active", true)
    oApp.setString("status-text", "System tray created with icon")

func onSetTrayIcon
    if !oApp.getProperty("tray-active")
        oApp.setString("status-text", "Create a tray first")
        return
    ok
    cIconPath = currentdir() + "/images/ring.png"
    oApp.traySetIcon(cIconPath)
    oApp.setString("status-text", "Tray icon updated")

func onSetTrayTooltip
    if !oApp.getProperty("tray-active")
        oApp.setString("status-text", "Create a tray first")
        return
    ok
    oApp.traySetTooltip("Ring Slint - Updated Tooltip!")
    oApp.setString("status-text", "Tray tooltip updated")

func onDestroyTray
    if !oApp.getProperty("tray-active")
        oApp.setString("status-text", "No tray to destroy")
        return
    ok
    oApp.trayDestroy()
    oApp.setBool("tray-active", false)
    oApp.setString("status-text", "System tray destroyed")

func onTrayShow
    oApp.show()
    oApp.setString("status-text", "Window shown from tray")

func onTraySendNotification
    oApp.notify("Tray Action", "Notification from system tray!")

func onTrayQuit
    oApp.quit()

func onPollEvents
    oApp.hotkeyPoll()
    oApp.trayPoll()
