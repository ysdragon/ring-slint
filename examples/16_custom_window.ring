load "slint.ring"

oApp = new SlintApp {
    loadUI("16_custom_window.slint")
    setCallback("close-window", :onClose)
    setCallback("opacity-changed", :onOpacityChanged)
    setCallback("start-drag", :onStartDrag)
    show()
    run()
}

func onClose
    ? "Closing translucent window"
    oApp.quit()

func onOpacityChanged
    nOpacity = oApp.callbackArg(1)
    oApp.set("window-opacity", nOpacity)
    ? "Opacity: " + (nOpacity * 100) + "%"

func onStartDrag
    oApp.windowDrag()
