load "slint.ring"

oApp = new SlintApp {
    loadUI("05_invoke.slint")
    setCallback("calculate", :onCalculate)
    show()
    run()
}

func onCalculate
    nA = oApp.callbackArg(1)
    nB = oApp.callbackArg(2)
    nSum = oApp.invoke("add", [nA, nB])
    nProduct = oApp.invoke("multiply", [nA, nB])
    ? "A=" + nA + ", B=" + nB
    ? "Sum: " + nSum
    ? "Product: " + nProduct
    oApp.set("result", "Sum: " + nSum + ", Product: " + nProduct)
