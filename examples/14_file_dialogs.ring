load "slint.ring"

oApp = new SlintApp {
    loadUI("14_file_dialogs.slint")
    setCallback("open-file", :onOpenFile)
    setCallback("open-files", :onOpenFiles)
    setCallback("save-file", :onSaveFile)
    setCallback("open-folder", :onOpenFolder)
    setCallback("open-folders", :onOpenFolders)
    setCallback("show-msgbox", :onShowMsgbox)
    setCallback("show-confirm", :onShowConfirm)
    setCallback("show-yesno", :onShowYesNo)
    show()
    run()
}

func onOpenFile
    aFilters = [
        ["Ring Files", "ring"],
        ["Slint Files", "slint"],
        ["All Files", "*"]
    ]
    cFile = oApp.fileOpenWithFilters("Select a File", aFilters)
    if cFile != NULL and len(cFile) > 0
        oApp.set("result-text", "Selected file:" + nl + cFile)
    else
        oApp.set("result-text", "No file selected")
    ok

func onOpenFiles
    aFilters = [
        ["Images", "png", "jpg", "jpeg", "gif"],
        ["Documents", "txt", "pdf", "doc"],
        ["All Files", "*"]
    ]
    aFiles = oApp.fileOpenMultipleWithFilters("Select Multiple Files", aFilters)
    if len(aFiles) > 0
        cResult = "Selected " + len(aFiles) + " file(s):" + nl
        for cFile in aFiles
            cResult += "  " + cFile + nl
        next
        oApp.set("result-text", cResult)
    else
        oApp.set("result-text", "No files selected")
    ok

func onSaveFile
    aFilters = [
        ["Ring Files", "ring"],
        ["Text Files", "txt"]
    ]
    cFile = oApp.fileSaveWithFilters("Save File As", "untitled.ring", aFilters)
    if cFile != NULL and len(cFile) > 0
        oApp.set("result-text", "Save to:" + nl + cFile)
    else
        oApp.set("result-text", "Save canceled")
    ok

func onOpenFolder
    cFolder = oApp.folderOpen("Select a Folder")
    if cFolder != NULL and len(cFolder) > 0
        oApp.set("result-text", "Selected folder:" + nl + cFolder)
    else
        oApp.set("result-text", "No folder selected")
    ok

func onOpenFolders
    aFolders = oApp.folderOpenMultiple("Select Multiple Folders")
    if len(aFolders) > 0
        cResult = "Selected " + len(aFolders) + " folder(s):" + nl
        for cFolder in aFolders
            cResult += "  " + cFolder + nl
        next
        oApp.set("result-text", cResult)
    else
        oApp.set("result-text", "No folders selected")
    ok

func onShowMsgbox
    oApp.msgbox("Information", "This is an informational message!")
    oApp.set("result-text", "Info message shown")

func onShowConfirm
    nResult = oApp.confirm("Confirm Action", "Do you want to proceed with this operation?")
    if nResult = 1
        oApp.set("result-text", "User clicked OK")
    else
        oApp.set("result-text", "User clicked Cancel")
    ok

func onShowYesNo
    nResult = oApp.yesno("Question", "Do you like Ring programming language?")
    if nResult = 1
        oApp.set("result-text", "User clicked Yes!")
    else
        oApp.set("result-text", "User clicked No")
    ok
