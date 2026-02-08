load "slint.ring"

cLog = ""
nPass = 0
nFail = 0
nModelId = 0

oApp = new SlintApp {
    loadUI("08_types.slint")
    setCallback("run-string-test", :testString)
    setCallback("run-number-test", :testNumber)
    setCallback("run-bool-test", :testBool)
    setCallback("run-struct-test", :testStruct)
    setCallback("run-model-test", :testModel)
    setCallback("run-color-test", :testColor)
    setCallback("run-enum-test", :testEnum)
    setCallback("run-image-test", :testImage)
    setCallback("run-roundtrip-test", :testRoundtrip)
    setCallback("run-all-tests", :testAll)
    show()
    run()
}

func testAll
    cLog = ""
    nPass = 0
    nFail = 0
    testString()
    testNumber()
    testBool()
    testStruct()
    testModel()
    testColor()
    testEnum()
    testImage()
    testRoundtrip()
    log("─────────────────────────")
    log("Results: " + nPass + " passed, " + nFail + " failed")

func testString
    log("── String ──")

    oApp.set("str-val", "hello ring")
    assert("set/get string", oApp.getProperty("str-val"), "hello ring")

    oApp.set("str-val", "")
    assert("empty string", oApp.getProperty("str-val"), "")

    oApp.set("str-val", "special: é ñ ü 日本語")
    assert("unicode string", oApp.getProperty("str-val"), "special: é ñ ü 日本語")

    cResult = oApp.invoke("greet", ["World"])
    assert("invoke returns string", cResult, "Hello, World!")

func testNumber
    log("── Number ──")

    oApp.set("int-val", 42)
    assert("set/get int", oApp.getProperty("int-val"), 42)

    oApp.set("int-val", -100)
    assert("negative int", oApp.getProperty("int-val"), -100)

    oApp.set("int-val", 0)
    assert("zero", oApp.getProperty("int-val"), 0)

    oApp.set("float-val", 3.14)
    nFloat = oApp.getProperty("float-val")
    assertApprox("set/get float", nFloat, 3.14, 0.01)

    nSum = oApp.invoke("add-numbers", [7, 3])
    assert("invoke add(7,3)", nSum, 10)

func testBool
    log("── Bool ──")

    oApp.set("bool-val", "true")
    assert("set true via string", oApp.getProperty("bool-val"), 1)

    oApp.set("bool-val", "false")
    assert("set false via string", oApp.getProperty("bool-val"), 0)

    oApp.setBool("bool-val", true)
    assert("setBool true", oApp.getProperty("bool-val"), 1)

    oApp.setBool("bool-val", false)
    assert("setBool false", oApp.getProperty("bool-val"), 0)

    nResult = oApp.invoke("echo-bool", ["true"])
    assert("invoke echo-bool(true)", nResult, 1)

    nResult = oApp.invoke("echo-bool", ["false"])
    assert("invoke echo-bool(false)", nResult, 0)

func testStruct
    log("── Struct ──")

    nModelId = oApp.modelCreate("people")

    oApp.modelPush(nModelId, [:name = "Alice", :age = 30, :active = "true"])
    aItem = oApp.modelGet(nModelId, 0)

    assert("struct field string", aItem[:name], "Alice")
    assert("struct field number", aItem[:age], 30)
    assert("struct field bool", aItem[:active], 1)

    oApp.modelSet(nModelId, 0, [:name = "Alice", :age = 31, :active = "false"])
    aItem = oApp.modelGet(nModelId, 0)
    assert("struct update number", aItem[:age], 31)
    assert("struct update bool", aItem[:active], 0)

    oApp.modelDestroy(nModelId)

func testModel
    log("── Model ──")

    nModelId = oApp.modelCreate("people")

    oApp.modelPush(nModelId, [:name = "Bob", :age = 25, :active = "true"])
    oApp.modelPush(nModelId, [:name = "Carol", :age = 35, :active = "false"])
    oApp.modelPush(nModelId, [:name = "Dave", :age = 28, :active = "true"])

    assert("model count", oApp.modelCount(nModelId), 3)

    aFirst = oApp.modelGet(nModelId, 0)
    assert("model[0].name", aFirst[:name], "Bob")

    aLast = oApp.modelGet(nModelId, 2)
    assert("model[2].name", aLast[:name], "Dave")

    oApp.modelRemove(nModelId, 1)
    assert("count after remove", oApp.modelCount(nModelId), 2)

    aAfter = oApp.modelGet(nModelId, 1)
    assert("model[1] after remove", aAfter[:name], "Dave")

    oApp.modelInsert(nModelId, 0, [:name = "Eve", :age = 22, :active = "true"])
    assert("count after insert", oApp.modelCount(nModelId), 3)

    aInserted = oApp.modelGet(nModelId, 0)
    assert("inserted at 0", aInserted[:name], "Eve")

    oApp.modelClear(nModelId)
    assert("count after clear", oApp.modelCount(nModelId), 0)

    oApp.modelDestroy(nModelId)

func testColor
    log("── Color ──")

    oApp.set("color-val", "#ff0000")
    assert("set red via hex6", oApp.getProperty("color-val"), "#ff0000")

    oApp.set("color-val", "#00ff00")
    assert("set green via hex6", oApp.getProperty("color-val"), "#00ff00")

    oApp.set("color-val", "#0000ff80")
    assert("set blue+alpha via hex8", oApp.getProperty("color-val"), "#0000ff80")

    oApp.set("color-val", "#ffffff")
    assert("set white", oApp.getProperty("color-val"), "#ffffff")

    oApp.set("color-val", "#000000")
    assert("set black", oApp.getProperty("color-val"), "#000000")

func testEnum
    log("── Enum ──")

    oApp.set("align-val", "TextHorizontalAlignment.left")
    assert("set enum left", oApp.getProperty("align-val"), "TextHorizontalAlignment.left")

    oApp.set("align-val", "TextHorizontalAlignment.center")
    assert("set enum center", oApp.getProperty("align-val"), "TextHorizontalAlignment.center")

    oApp.set("align-val", "TextHorizontalAlignment.right")
    assert("set enum right", oApp.getProperty("align-val"), "TextHorizontalAlignment.right")

func testImage
    log("── Image ──")

    cPath = oApp.getProperty("image-val")
    assert("default image is empty", cPath, "")

func testRoundtrip
    log("── Roundtrip ──")

    oApp.set("str-val", "roundtrip")
    assert("string roundtrip", oApp.getProperty("str-val"), "roundtrip")

    oApp.set("int-val", 999)
    assert("int roundtrip", oApp.getProperty("int-val"), 999)

    oApp.set("float-val", 2.718)
    assertApprox("float roundtrip", oApp.getProperty("float-val"), 2.718, 0.01)

    oApp.set("bool-val", "true")
    assert("bool roundtrip", oApp.getProperty("bool-val"), 1)

    nModelId = oApp.modelCreate("people")
    oApp.modelPush(nModelId, [:name = "Test", :age = 99, :active = "false"])
    aItem = oApp.modelGet(nModelId, 0)
    assert("struct roundtrip name", aItem[:name], "Test")
    assert("struct roundtrip age", aItem[:age], 99)
    assert("struct roundtrip bool", aItem[:active], 0)

    nSum = oApp.invoke("add-numbers", [123, 456])
    assert("invoke roundtrip", nSum, 579)

    oApp.set("color-val", "#abcdef")
    assert("color roundtrip", oApp.getProperty("color-val"), "#abcdef")

    oApp.set("align-val", "TextHorizontalAlignment.right")
    assert("enum roundtrip", oApp.getProperty("align-val"), "TextHorizontalAlignment.right")

    oApp.modelDestroy(nModelId)

func assert cName, actual, expected
    if actual = expected
        log("  ✓ " + cName)
        nPass++
    else
        log("  ✗ " + cName + " → got: " + actual + ", want: " + expected)
        nFail++
    ok

func assertApprox cName, actual, expected, tolerance
    if fabs(actual - expected) <= tolerance
        log("  ✓ " + cName)
        nPass++
    else
        log("  ✗ " + cName + " → got: " + actual + ", want: ~" + expected)
        nFail++
    ok

func log cMsg
    ? cMsg
    cLog += cMsg + nl
    oApp.set("log-text", cLog)
