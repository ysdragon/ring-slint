load "slint.ring"

nModelId = 0

oApp = new SlintApp {
    loadUI("09_todo.slint")
    nModelId = modelCreate("todos")
    setCallback("add-todo", :onAddTodo)
    setCallback("toggle-todo", :onToggleTodo)
    setCallback("delete-todo", :onDeleteTodo)
    setCallback("clear-completed", :onClearCompleted)
    show()
    run()
}

func onAddTodo
    cText = oApp.callbackArg(1)
    if len(trim(cText)) > 0
        oApp.modelPush(nModelId, [:text = cText, :completed = "false"])
        oApp.set("new-todo-text", "")
        updateCount()
    ok

func onToggleTodo
    nIndex = oApp.callbackArg(1)
    aItem = oApp.modelGet(nModelId, nIndex)
    if aItem[:completed] = 1
        oApp.modelSet(nModelId, nIndex, [:text = aItem[:text], :completed = "false"])
    else
        oApp.modelSet(nModelId, nIndex, [:text = aItem[:text], :completed = "true"])
    ok

func onDeleteTodo
    nIndex = oApp.callbackArg(1)
    oApp.modelRemove(nModelId, nIndex)
    updateCount()

func onClearCompleted
    nCount = oApp.modelCount(nModelId)
    i = nCount - 1
    while i >= 0
        aItem = oApp.modelGet(nModelId, i)
        if aItem[:completed]
            oApp.modelRemove(nModelId, i)
        ok
        i -= 1
    end
    updateCount()

func updateCount
    nCount = oApp.modelCount(nModelId)
    oApp.set("todo-count", nCount)
