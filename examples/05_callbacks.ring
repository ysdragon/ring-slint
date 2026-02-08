load "slint.ring"

oApp = new SlintApp {
    loadUI("05_callbacks.slint")
    setCallback("button-clicked", :onButtonClicked)
    setCallback("slider-changed", :onSliderChanged)
    show()
    run()
}

func onButtonClicked
    nArgs = oApp.callbackArgsCount()
    ? "Button clicked! Args count: " + nArgs
    if nArgs > 0
        cButtonId = oApp.callbackArg(1)
        ? "Button ID: " + cButtonId
        oApp.set("last-clicked", "Last clicked: " + cButtonId)
    ok

func onSliderChanged
    nArgs = oApp.callbackArgsCount()
    if nArgs > 0
        nValue = oApp.callbackArg(1)
        ? "Slider value: " + nValue
        oApp.set("slider-display", "Value: " + floor(nValue))
    ok
