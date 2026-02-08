load "slint.ring"

cDisplay = "0"
cPending = ""
nOperand = 0
bNewNumber = true

oApp = new SlintApp {
    loadUI("15_calculator.slint")
    setCallback("digit", :onDigit)
    setCallback("operator", :onOperator)
    setCallback("equals", :onEquals)
    setCallback("clear", :onClear)
    setCallback("decimal", :onDecimal)
    setCallback("negate", :onNegate)
    setCallback("percent", :onPercent)
    show()
    run()
}

func onDigit
    cDigit = oApp.callbackArg(1)
    if bNewNumber
        cDisplay = cDigit
        bNewNumber = false
    else
        if cDisplay = "0"
            cDisplay = cDigit
        else
            cDisplay += cDigit
        ok
    ok
    updateDisplay()

func onOperator
    cOp = oApp.callbackArg(1)
    if cPending != ""
        calculate()
    ok
    nOperand = number(cDisplay)
    cPending = cOp
    bNewNumber = true

func onEquals
    if cPending != ""
        calculate()
        cPending = ""
    ok

func onClear
    cDisplay = "0"
    cPending = ""
    nOperand = 0
    bNewNumber = true
    updateDisplay()

func onDecimal
    if bNewNumber
        cDisplay = "0."
        bNewNumber = false
    else
        if not substr(cDisplay, ".")
            cDisplay += "."
        ok
    ok
    updateDisplay()

func onNegate
    nVal = number(cDisplay)
    cDisplay = "" + (-nVal)
    updateDisplay()

func onPercent
    nVal = number(cDisplay)
    cDisplay = "" + (nVal / 100)
    updateDisplay()

func calculate
    nCurrent = number(cDisplay)
    switch cPending
        on "+"
            cDisplay = "" + (nOperand + nCurrent)
        on "-"
            cDisplay = "" + (nOperand - nCurrent)
        on "*"
            cDisplay = "" + (nOperand * nCurrent)
        on "/"
            if nCurrent != 0
                cDisplay = "" + (nOperand / nCurrent)
            else
                cDisplay = "Error"
            ok
    off
    bNewNumber = true
    updateDisplay()

func updateDisplay
    oApp.set("display", cDisplay)
