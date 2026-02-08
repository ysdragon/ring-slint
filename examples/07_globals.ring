load "slint.ring"

oApp = new SlintApp {
    loadUI("07_globals.slint")
    globalSet("AppState", "user-name", "Ring User")
    globalSet("AppState", "counter", 0)
    globalSetCallback("AppState", "increment", :onIncrement)
    globalSetCallback("AppState", "save", :onSave)
    show()
    run()
}

func onIncrement
    nCount = oApp.globalGet("AppState", "counter")
    nCount++
    oApp.globalSet("AppState", "counter", nCount)
    ? "Counter: " + nCount

func onSave
    cName = oApp.globalGet("AppState", "user-name")
    nCount = oApp.globalGet("AppState", "counter")
    ? "Saving: " + cName + " with count " + nCount
    nResult = oApp.globalInvoke("AppState", "multiply", [7, 6])
    ? "Global invoke multiply(7, 6) = " + nResult
    cGreeting = oApp.globalInvoke("AppState", "format-greeting", [cName])
    ? "Global invoke format-greeting: " + cGreeting
