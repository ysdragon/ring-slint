load "slint.ring"

nModelId = 0
nSelectedIndex = -1
cSaveFile = "notes.dat"
aAllNotes = []
aFilterMap = []
cSearchText = ""

oApp = new SlintApp {
    loadUI("20_notes.slint")
    nModelId = modelCreate("notes")
    setCallback("add-note", :onAddNote)
    setCallback("delete-note", :onDeleteNote)
    setCallback("select-note", :onSelectNote)
    setCallback("update-title", :onUpdateTitle)
    setCallback("update-content", :onUpdateContent)
    setCallback("search-changed", :onSearchChanged)
    loadNotes()
    show()
    run()
}

func onAddNote
    cDate = date()
    add(aAllNotes, [
        :title = NULL,
        :content = NULL,
        :preview = NULL,
        :date = cDate
    ])
    cSearchText = ""
    oApp.set("search-text", "")
    rebuildModel()
    selectNote(len(aFilterMap) - 1)
    saveNotes()

func onDeleteNote
    nIndex = oApp.callbackArg(1)
    if nIndex >= 0 and nIndex < len(aFilterMap)
        del(aAllNotes, aFilterMap[nIndex + 1])
        rebuildModel()
        nNewCount = len(aFilterMap)
        if nNewCount = 0
            nSelectedIndex = -1
            oApp.set("selected-index", -1)
            oApp.set("current-title", "")
            oApp.set("current-content", "")
        else
            if nIndex >= nNewCount
                selectNote(nNewCount - 1)
            else
                selectNote(nIndex)
            ok
        ok
        saveNotes()
    ok

func onSelectNote
    selectNote(oApp.callbackArg(1))

func selectNote nDisplayIndex
    if nDisplayIndex >= 0 and nDisplayIndex < len(aFilterMap)
        nSelectedIndex = nDisplayIndex
        oApp.set("selected-index", nSelectedIndex)
        aNote = aAllNotes[aFilterMap[nDisplayIndex + 1]]
        oApp.set("current-title", aNote[:title])
        oApp.set("current-content", aNote[:content])
    ok

func onUpdateTitle
    cTitle = oApp.callbackArg(1)
    if nSelectedIndex >= 0 and nSelectedIndex < len(aFilterMap)
        nRealIndex = aFilterMap[nSelectedIndex + 1]
        aAllNotes[nRealIndex][:title] = cTitle
        aAllNotes[nRealIndex][:preview] = makePreview(aAllNotes[nRealIndex][:content])
        oApp.modelSet(nModelId, nSelectedIndex, [
            :title = aAllNotes[nRealIndex][:title],
            :content = aAllNotes[nRealIndex][:content],
            :preview = aAllNotes[nRealIndex][:preview],
            :date = aAllNotes[nRealIndex][:date]
        ])
        saveNotes()
    ok

func onUpdateContent
    cContent = oApp.callbackArg(1)
    if nSelectedIndex >= 0 and nSelectedIndex < len(aFilterMap)
        nRealIndex = aFilterMap[nSelectedIndex + 1]
        aAllNotes[nRealIndex][:content] = cContent
        aAllNotes[nRealIndex][:preview] = makePreview(cContent)
        oApp.modelSet(nModelId, nSelectedIndex, [
            :title = aAllNotes[nRealIndex][:title],
            :content = aAllNotes[nRealIndex][:content],
            :preview = aAllNotes[nRealIndex][:preview],
            :date = aAllNotes[nRealIndex][:date]
        ])
        saveNotes()
    ok

func onSearchChanged
    cSearchText = oApp.callbackArg(1)
    rebuildModel()

func rebuildModel
    oApp.modelClear(nModelId)
    aFilterMap = []
    cQuery = lower(cSearchText)
    for i = 1 to len(aAllNotes)
        aNote = aAllNotes[i]
        if len(cQuery) = 0 or
           substr(lower(aNote[:title]), cQuery) > 0 or
           substr(lower(aNote[:content]), cQuery) > 0
            oApp.modelPush(nModelId, [
                :title = aNote[:title],
                :content = aNote[:content],
                :preview = aNote[:preview],
                :date = aNote[:date]
            ])
            add(aFilterMap, i)
        ok
    next
    updateCount()
    nSelectedIndex = -1
    oApp.set("selected-index", -1)
    oApp.set("current-title", "")
    oApp.set("current-content", "")

func makePreview cText
    cClean = substr(cText, char(10), " ")
    cClean = substr(cClean, char(13), "")
    if len(cClean) > 60
        return left(cClean, 60) + "..."
    ok
    return cClean

func updateCount
    oApp.set("note-count", len(aAllNotes))

# --- File persistence ---

func saveNotes
    cData = ""
    for i = 1 to len(aAllNotes)
        aNote = aAllNotes[i]
        cContent = substr(aNote[:content], char(10), "<<<NL>>>")
        cData += aNote[:title] + nl +
                 aNote[:date] + nl +
                 cContent + nl +
                 "---" + nl
    next
    write(cSaveFile, cData)

func loadNotes
    if not fexists(cSaveFile) return ok
    cData = read(cSaveFile)
    if len(trim(cData)) = 0 return ok
    aLines = str2list(cData)
    nLine = 1
    while nLine + 2 <= len(aLines)
        cTitle = aLines[nLine]
        cDate = aLines[nLine + 1]
        cContent = aLines[nLine + 2]
        cContent = substr(cContent, "<<<NL>>>", char(10))
        add(aAllNotes, [
            :title = cTitle,
            :content = cContent,
            :preview = makePreview(cContent),
            :date = cDate
        ])
        nLine += 4
    end
    rebuildModel()
    if len(aFilterMap) > 0
        selectNote(0)
    ok
