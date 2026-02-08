load "slint.ring"

nTimerId = 0
nOnceTimerId = 0
nSeconds = 0

oApp = new SlintApp {
    loadUI("09_timer.slint")

    setCallback("start-timer", :onStartTimer)
    setCallback("stop-timer", :onStopTimer)
    setCallback("reset-timer", :onResetTimer)
    setCallback("check-running", :onCheckRunning)
    setCallback("restart-timer", :onRestartTimer)
    setCallback("change-interval", :onChangeInterval)
    
    nOnceTimerId = timerStartOnce(2000, :onOnceTimer)
    
    show()
    run()
}

func onStartTimer
    if nTimerId = 0
        nInterval = oApp.getProperty("interval-ms")
        nTimerId = oApp.timerStart(nInterval, :onTick)
        oApp.setBool("running", true)
        oApp.set("status", "Running")
        ? "Timer started with ID: " + nTimerId + " at " + nInterval + "ms interval"
    ok

func onStopTimer
    if nTimerId != 0
        oApp.timerStop(nTimerId)
        nTimerId = 0
        oApp.setBool("running", false)
        oApp.set("status", "Paused")
        ? "Timer stopped"
    ok

func onResetTimer
    if nTimerId != 0
        oApp.timerStop(nTimerId)
        nTimerId = 0
    ok
    nSeconds = 0
    oApp.set("seconds", 0)
    oApp.setBool("running", false)
    oApp.set("status", "Reset")
    ? "Timer reset"

func onTick
    nSeconds++
    oApp.set("seconds", nSeconds)
    ? "Tick: " + nSeconds

func onCheckRunning
    if nTimerId != 0
        bRunning = oApp.timerRunning(nTimerId)
        if bRunning
            oApp.set("status", "Timer IS running")
            ? "Timer " + nTimerId + " is running"
        else
            oApp.set("status", "Timer exists but NOT running")
            ? "Timer " + nTimerId + " exists but is not running"
        ok
    else
        oApp.set("status", "No timer created")
        ? "No timer created yet"
    ok

func onRestartTimer
    if nTimerId != 0
        oApp.timerRestart(nTimerId)
        oApp.setBool("running", true)
        oApp.set("status", "Restarted")
    else
        nSeconds = 0
        oApp.set("seconds", 0)
        nInterval = oApp.getProperty("interval-ms")
        nTimerId = oApp.timerStart(nInterval, :onTick)
        oApp.setBool("running", true)
        oApp.set("status", "Started")
    ok

func onChangeInterval
    nInterval = oApp.getProperty("interval-ms")
    if nTimerId != 0
        oApp.timerSetInterval(nTimerId, nInterval)
        oApp.set("status", "Interval changed to " + nInterval + "ms")
        ? "Timer " + nTimerId + " interval changed to " + nInterval + "ms"
    else
        ? "No timer to modify - will use new interval when started"
        oApp.set("status", "Will use " + nInterval + "ms when started")
    ok

func onOnceTimer
    oApp.set("status", "One-shot timer fired!")
