load "slint.ring"

oApp = new SlintApp {
    loadUI("10_temperature.slint")
    setCallback("celsius-changed", :onCelsiusChanged)
    setCallback("fahrenheit-changed", :onFahrenheitChanged)
    setCallback("kelvin-changed", :onKelvinChanged)
    show()
    run()
}

func onCelsiusChanged
    nCelsius = oApp.callbackArg(1)
    nFahrenheit = (nCelsius * 9 / 5) + 32
    nKelvin = nCelsius + 273.15
    oApp.set("fahrenheit", round(nFahrenheit, 2))
    oApp.set("kelvin", round(nKelvin, 2))
    updateBar(nCelsius)

func onFahrenheitChanged
    nFahrenheit = oApp.callbackArg(1)
    nCelsius = (nFahrenheit - 32) * 5 / 9
    nKelvin = nCelsius + 273.15
    oApp.set("celsius", round(nCelsius, 2))
    oApp.set("kelvin", round(nKelvin, 2))
    updateBar(nCelsius)

func onKelvinChanged
    nKelvin = oApp.callbackArg(1)
    nCelsius = nKelvin - 273.15
    nFahrenheit = (nCelsius * 9 / 5) + 32
    oApp.set("celsius", round(nCelsius, 2))
    oApp.set("fahrenheit", round(nFahrenheit, 2))
    updateBar(nCelsius)

func updateBar nCelsius
    nPercent = (nCelsius + 50) / 150 * 100
    if nPercent < 0 nPercent = 0 ok
    if nPercent > 100 nPercent = 100 ok
    oApp.set("temp-percent", nPercent)
    if nCelsius < 0
        oApp.set("temp-label", "Freezing! 🥶")
    but nCelsius < 15
        oApp.set("temp-label", "Cold 🧊")
    but nCelsius < 25
        oApp.set("temp-label", "Comfortable 😊")
    but nCelsius < 35
        oApp.set("temp-label", "Warm ☀️")
    else
        oApp.set("temp-label", "Hot! 🔥")
    ok

func round nValue, nDecimals
    nMult = pow(10, nDecimals)
    return floor(nValue * nMult + 0.5) / nMult
