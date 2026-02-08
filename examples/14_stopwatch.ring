load "slint.ring"

nTimerId = 0
nMilliseconds = 0
nLapsModelId = 0

oApp = new SlintApp {
    loadUI("14_stopwatch.slint")
    nLapsModelId = modelCreate("laps")
    setCallback("start-stop", :onStartStop)
    setCallback("lap", :onLap)
    setCallback("reset", :onReset)
    show()
    run()
}

func onStartStop
    if nTimerId = 0
        nTimerId = oApp.timerStart(10, :onTick)
        oApp.setBool("running", true)
    else
        oApp.timerStop(nTimerId)
        nTimerId = 0
        oApp.setBool("running", false)
    ok

func onLap
    if nTimerId != 0
        nLapNum = oApp.modelCount(nLapsModelId) + 1
        oApp.modelPush(nLapsModelId, [:num = nLapNum, :time = formatTime(nMilliseconds)])
    ok

func onReset
    if nTimerId != 0
        oApp.timerStop(nTimerId)
        nTimerId = 0
    ok
    nMilliseconds = 0
    oApp.setBool("running", false)
    oApp.set("display", "00:00.00")
    oApp.modelClear(nLapsModelId)

func onTick
    nMilliseconds += 10
    oApp.set("display", formatTime(nMilliseconds))

func formatTime nMs
    nTotalSecs = floor(nMs / 1000)
    nMins = floor(nTotalSecs / 60)
    nSecs = nTotalSecs % 60
    nCentis = floor((nMs % 1000) / 10)
    return padZero(nMins) + ":" + padZero(nSecs) + "." + padZero(nCentis)

func padZero n
    if n < 10
        return "0" + n
    ok
    return "" + n
