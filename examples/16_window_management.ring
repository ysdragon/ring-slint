load "slint.ring"

oApp = new SlintApp {
    loadUI("16_window_management.slint")

    setCallback("toggle-minimized", :onToggleMinimized)
    setCallback("toggle-maximized", :onToggleMaximized)
    setCallback("toggle-fullscreen", :onToggleFullscreen)
    setCallback("set-position", :onSetPosition)
    setCallback("get-position", :onGetPosition)
    setCallback("set-size", :onSetSize)
    setCallback("get-size", :onGetSize)
    setCallback("open-file", :onOpenFile)
    setCallback("save-file", :onSaveFile)
    setCallback("show-message", :onShowMessage)
    setCallback("show-confirm", :onShowConfirm)
    setCallback("get-scale-factor", :onGetScaleFactor)
    setCallback("check-visible", :onCheckVisible)
    setCallback("request-redraw", :onRequestRedraw)
    setCallback("start-drag", :onStartDrag)
    setCallback("set-icon", :onSetIcon)
    setCallback("show-pointers", :onShowPointers)

    show()
    run()
}

func onToggleMinimized
    if oApp.windowIsMinimized()
        oApp.windowSetMinimized(0)
        oApp.set("status-text", "Window restored from minimized")
    else
        oApp.windowSetMinimized(1)
        oApp.set("status-text", "Window minimized")
    ok

func onToggleMaximized
    if oApp.windowIsMaximized()
        oApp.windowSetMaximized(0)
        oApp.set("status-text", "Window restored from maximized")
    else
        oApp.windowSetMaximized(1)
        oApp.set("status-text", "Window maximized")
    ok

func onToggleFullscreen
    if oApp.windowIsFullscreen()
        oApp.windowSetFullscreen(0)
        oApp.set("status-text", "Window exited fullscreen")
    else
        oApp.windowSetFullscreen(1)
        oApp.set("status-text", "Window entered fullscreen")
    ok

func onSetPosition
    nX = oApp.getProperty("pos-x")
    nY = oApp.getProperty("pos-y")
    oApp.windowSetPosition(nX, nY)
    oApp.set("status-text", "Position set to: " + nX + ", " + nY)

func onGetPosition
    aPos = oApp.windowGetPosition()
    oApp.set("pos-x", aPos[1])
    oApp.set("pos-y", aPos[2])
    oApp.set("status-text", "Current position: " + aPos[1] + ", " + aPos[2])

func onSetSize
    nWidth = oApp.getProperty("size-width")
    nHeight = oApp.getProperty("size-height")
    oApp.windowSetSize(nWidth, nHeight)
    oApp.set("status-text", "Size set to: " + nWidth + " x " + nHeight)

func onGetSize
    aSize = oApp.windowGetSize()
    oApp.set("size-width", aSize[1])
    oApp.set("size-height", aSize[2])
    oApp.set("status-text", "Current size: " + aSize[1] + " x " + aSize[2])

func onOpenFile
    aFilters = [
        ["Ring Files", "ring"],
        ["Slint Files", "slint"],
        ["All Files", "*"]
    ]
    cFile = oApp.fileOpenWithFilters("Select a File", aFilters)
    if cFile != NULL and len(cFile) > 0
        oApp.set("status-text", "Selected: " + cFile)
    else
        oApp.set("status-text", "No file selected")
    ok

func onSaveFile
    aFilters = [
        ["Ring Files", "ring"],
        ["Text Files", "txt"]
    ]
    cFile = oApp.fileSaveWithFilters("Save File As", "untitled.ring", aFilters)
    if cFile != NULL and len(cFile) > 0
        oApp.set("status-text", "Save to: " + cFile)
    else
        oApp.set("status-text", "Save canceled")
    ok

func onShowMessage
    oApp.msgbox("Test Message", "This is a test message from SlintApp!")
    oApp.set("status-text", "Message box shown")

func onShowConfirm
    nResult = oApp.confirm("Confirm", "Do you want to continue?")
    if nResult = 1
        oApp.set("status-text", "User confirmed (OK)")
    else
        oApp.set("status-text", "User canceled")
    ok

func onGetScaleFactor
    nScale = oApp.windowScaleFactor()
    oApp.set("status-text", "Scale factor: " + nScale)

func onCheckVisible
    nVisible = oApp.windowIsVisible()
    if nVisible = 1
        oApp.set("status-text", "Window is visible")
    else
        oApp.set("status-text", "Window is not visible")
    ok

func onRequestRedraw
    oApp.windowRequestRedraw()
    oApp.set("status-text", "Redraw requested")

func onStartDrag
    oApp.windowDrag()
    oApp.set("status-text", "Window drag initiated")

func onSetIcon
    # windowSetIcon - set custom window icon
    # Note: Requires an actual icon file to work
    try 
        cIconPath = currentdir() + "/icon.png"
        oApp.windowSetIcon(cIconPath)
        oApp.set("status-text", "Window icon set (if icon.png exists)")
    catch
        oApp.set("status-text", "Failed to set icon (icon.png not found)")
    done

func onShowPointers
    # window() and definition() getters
    pWindow = oApp.window()
    pDefinition = oApp.definition()
    if pWindow != NULL
        cWin = "valid"
    else
        cWin = "null"
    ok
    if pDefinition != NULL
        cDef = "valid"
    else
        cDef = "null"
    ok
    oApp.set("status-text", "Window: " + cWin + " | Definition: " + cDef)
