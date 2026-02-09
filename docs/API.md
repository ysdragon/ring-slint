# Ring Slint API Reference

Complete API documentation for the `SlintApp` class.

## Table of Contents

- [Core Methods](#core-methods)
- [Property Methods](#property-methods)
- [Callback Methods](#callback-methods)
- [Global Singleton Methods](#global-singleton-methods)
- [Timer Methods](#timer-methods)
- [Model Methods](#model-methods)
- [Style Methods](#style-methods)
- [Window Management Methods](#window-management-methods)
- [Component Introspection Methods](#component-introspection-methods)
- [File Dialog Methods](#file-dialog-methods) *(Desktop only)*
- [Message Dialog Methods](#message-dialog-methods) *(Desktop only)*
- [Notification Methods](#notification-methods) *(Desktop only)*
- [Clipboard Methods](#clipboard-methods) *(Desktop only)*
- [Hotkey Methods](#hotkey-methods) *(Desktop only)*
- [System Tray Methods](#system-tray-methods) *(Desktop only)*

---

## Core Methods

### `loadUI(cFile)`

Loads and compiles a Slint definition from a file.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cFile` | String | Path to the `.slint` file |

**Returns:** Self (for method chaining)

**Example:**
```ring
oApp = new SlintApp {
    loadUI("main.slint")
}
```

---

### `loadUIString(cSource, cPath)`

Loads and compiles a Slint definition from a source string.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cSource` | String | Slint markup source code |
| `cPath` | String | Virtual path for resolving imports |

**Returns:** Self

**Example:**
```ring
cUI = '
export component App inherits Window {
    Text { text: "Hello"; }
}
'
oApp = new SlintApp {
    loadUIString(cUI, "virtual://app.slint")
}
```

---

### `create()`

Creates a window instance from the loaded component definition. Call after `loadUI`/`loadUIString` if you need to recreate the window.

**Returns:** Self

---

### `show()`

Shows the window.

**Returns:** Self

---

### `hide()`

Hides the window without destroying it.

**Returns:** Self

---

### `run()`

Runs the application event loop. This is a blocking call that returns when the window is closed or `quit()` is called.

**Example:**
```ring
oApp = new SlintApp {
    loadUI("app.slint")
    show()
    run()  // Blocks here until window closes
}
```

---

### `quit()`

Quits the application and exits the event loop.

---

### `window()`

Gets the native window pointer.

**Returns:** Window pointer

---

### `definition()`

Gets the component definition pointer.

**Returns:** Component definition pointer

---

## Property Methods

### `set(cProp, value)`

Sets a property value on the Slint component.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `value` | Any | Value to set (string, number, or list) |

**Returns:** Self

**Example:**
```ring
oApp.set("counter", 42)
oApp.set("title", "My App")
oApp.set("items", ["Apple", "Banana", "Cherry"])
```

---

### `setBool(cProp, bValue)`

Sets a boolean property value.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `bValue` | Boolean | `true`/`false` or `1`/`0` |

**Returns:** Self

---

### `setString(cProp, cValue)`

Sets a string property value. Always treats the value as a plain string with no type inference.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `cValue` | String | String value |

**Returns:** Self

**Example:**
```ring
oApp.setString("status-text", "File: icon.png not found")
oApp.setString("label", "#not-a-color")
```

---

### `setNumber(cProp, nValue)`

Sets a numeric property value.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `nValue` | Number | Integer or float value |

**Returns:** Self

**Example:**
```ring
oApp.setNumber("counter", 42)
oApp.setNumber("opacity", 0.5)
```

---

### `setColor(cProp, cHex)`

Sets a color/brush property from a hex color string.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `cHex` | String | Hex color (`#RRGGBB` or `#RRGGBBAA`) |

**Returns:** Self

**Example:**
```ring
oApp.setColor("background", "#FF0000")
oApp.setColor("overlay", "#00000080")
```

---

### `setEnum(cProp, cValue)`

Sets an enum property value.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |
| `cValue` | String | Enum in `EnumName.variant` format |

**Returns:** Self

**Example:**
```ring
oApp.setEnum("alignment", "TextHorizontalAlignment.center")
```

---

### `getProperty(cProp)`

Gets the current value of a property.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name |

**Returns:** Property value, or `NULL` if window not initialized

**Example:**
```ring
nCount = oApp.getProperty("counter")
? "Current count: " + nCount
```

---

## Callback Methods

### `setCallback(cCallback, cRingFunc)`

Registers a Ring function as a callback for a Slint callback.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cCallback` | String | Callback name defined in Slint |
| `cRingFunc` | String | Ring function name (use `:funcName` syntax) |

**Returns:** Self

**Example:**
```ring
oApp.setCallback("button-clicked", :onButtonClick)

func onButtonClick
    ? "Button was clicked!"
```

---

### `invoke(cCallback, aArgs)`

Invokes a Slint function/callback programmatically.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cCallback` | String | Function name to invoke |
| `aArgs` | List | Optional arguments |

**Returns:** Return value from Slint function, or `NULL`

---

### `callbackArg(nIndex)`

Gets an argument passed to the current callback. Use inside a callback function.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nIndex` | Number | 1-based index (first argument is 1) |

**Returns:** Argument value

**Example:**
```ring
func onValueChanged
    newValue = oApp.callbackArg(1)
    ? "New value: " + newValue
```

---

### `callbackArgsCount()`

Gets the number of arguments passed to the current callback.

**Returns:** Number of arguments

---

## Global Singleton Methods

### `globalGet(cGlobal, cProp)`

Gets a property value from a Slint global singleton.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cGlobal` | String | Global name (e.g., "AppState") |
| `cProp` | String | Property name |

**Returns:** Property value

---

### `globalSet(cGlobal, cProp, value)`

Sets a property value on a Slint global singleton.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cGlobal` | String | Global name |
| `cProp` | String | Property name |
| `value` | Any | Value to set |

**Returns:** Self

---

### `globalSetCallback(cGlobal, cCallback, cRingFunc)`

Registers a callback on a Slint global singleton.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cGlobal` | String | Global name |
| `cCallback` | String | Callback name |
| `cRingFunc` | String | Ring function name |

**Returns:** Self

---

### `globalInvoke(cGlobal, cCallback, aArgs)`

Invokes a function on a Slint global singleton.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cGlobal` | String | Global name |
| `cCallback` | String | Function name |
| `aArgs` | List | Optional arguments |

**Returns:** Return value

---

## Timer Methods

### `timerStart(nInterval, cCallback)`

Starts a repeating timer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nInterval` | Number | Interval in milliseconds |
| `cCallback` | String | Ring function to call |

**Returns:** Timer ID

**Example:**
```ring
nTimerId = oApp.timerStart(1000, :onTick)

func onTick
    ? "Tick!"
```

---

### `timerStartOnce(nInterval, cCallback)`

Starts a one-shot timer that fires only once.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nInterval` | Number | Delay in milliseconds |
| `cCallback` | String | Ring function to call |

**Returns:** Timer ID

---

### `timerStop(nTimerId)`

Stops a running timer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nTimerId` | Number | Timer ID |

**Returns:** Result of operation

---

### `timerRunning(nTimerId)`

Checks if a timer is currently running.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nTimerId` | Number | Timer ID |

**Returns:** `1` if running, `0` if stopped

---

### `timerRestart(nTimerId)`

Restarts a stopped or running timer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nTimerId` | Number | Timer ID |

**Returns:** Result of operation

---

### `timerSetInterval(nTimerId, nInterval)`

Changes the interval of an existing timer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nTimerId` | Number | Timer ID |
| `nInterval` | Number | New interval in milliseconds |

**Returns:** Result of operation

---

## Model Methods

Models are used to populate Slint repeaters/lists with dynamic data.

### `modelCreate(cProp)`

Creates a new model and binds it to a property.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cProp` | String | Property name to bind |

**Returns:** Model ID, or `-1` on failure

**Example:**
```ring
nModelId = oApp.modelCreate("items")
oApp.modelPush(nModelId, ["text": "Item 1"])
oApp.modelPush(nModelId, ["text": "Item 2"])
```

---

### `modelPush(nModelId, value)`

Appends an item to the end of a model.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |
| `value` | Any | Item value (typically a list/struct) |

**Returns:** Result of operation

---

### `modelRemove(nModelId, nIndex)`

Removes an item from a model by index.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |
| `nIndex` | Number | Zero-based index |

**Returns:** Result of operation

---

### `modelSet(nModelId, nIndex, value)`

Updates an item in a model at a specific index.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |
| `nIndex` | Number | Zero-based index |
| `value` | Any | New value |

**Returns:** Result of operation

---

### `modelGet(nModelId, nIndex)`

Gets an item from a model by index.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |
| `nIndex` | Number | Zero-based index |

**Returns:** Item value

---

### `modelCount(nModelId)`

Gets the number of items in a model.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |

**Returns:** Number of items

---

### `modelClear(nModelId)`

Removes all items from a model.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |

**Returns:** Result of operation

---

### `modelInsert(nModelId, nIndex, value)`

Inserts an item at a specific index.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |
| `nIndex` | Number | Zero-based index |
| `value` | Any | Item value |

**Returns:** Result of operation

---

### `modelDestroy(nModelId)`

Destroys a model and releases its resources.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nModelId` | Number | Model ID |

**Returns:** Result of operation

---

## Style Methods

### `setStyle(cStyle)`

Sets the widget style for subsequent compilations. Must be called **before** `loadUI`/`loadUIString`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cStyle` | String | Style name: `"fluent"`, `"material"`, `"native"`, `"cupertino"`, or `""` for default |

**Returns:** Self

**Example:**
```ring
oApp = new SlintApp {
    setStyle("material")
    loadUI("app.slint")
}
```

---

### `getStyle()`

Gets the current widget style setting.

**Returns:** Current style name, or `NULL` if using default

---

### `addLibraryPath(cName, cPath)`

Adds a library path for `@library` imports in Slint files. Must be called **before** `loadUI`/`loadUIString`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cName` | String | Library name (used in `@name` imports) |
| `cPath` | String | Filesystem path to the library |

**Returns:** Self

---

### `removeLibraryPath(cName)`

Removes a previously added library path.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cName` | String | Library name |

**Returns:** Self

---

### `clearLibraryPaths()`

Clears all custom library paths.

**Returns:** Self

---

## Window Management Methods

### `windowSetPosition(nX, nY)`

Moves the window to a specific screen position.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nX` | Number | X coordinate in physical pixels |
| `nY` | Number | Y coordinate in physical pixels |

**Returns:** Self

---

### `windowGetPosition()`

Gets the current window position.

**Returns:** List `[x, y]` in physical pixels

---

### `windowSetSize(nWidth, nHeight)`

Sets the window size.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nWidth` | Number | Width in physical pixels |
| `nHeight` | Number | Height in physical pixels |

**Returns:** Self

---

### `windowGetSize()`

Gets the current window size.

**Returns:** List `[width, height]` in physical pixels

---

### `windowSetMinimized(bMinimized)`

Minimizes or restores the window.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bMinimized` | Boolean | `true` to minimize |

**Returns:** Self

---

### `windowIsMinimized()`

**Returns:** `1` if minimized, `0` otherwise

---

### `windowSetMaximized(bMaximized)`

Maximizes or restores the window.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bMaximized` | Boolean | `true` to maximize |

**Returns:** Self

---

### `windowIsMaximized()`

**Returns:** `1` if maximized, `0` otherwise

---

### `windowSetFullscreen(bFullscreen)`

Enables or disables fullscreen mode.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bFullscreen` | Boolean | `true` for fullscreen |

**Returns:** Self

---

### `windowIsFullscreen()`

**Returns:** `1` if fullscreen, `0` otherwise

---

### `windowScaleFactor()`

Gets the window's scale factor (for HiDPI displays).

**Returns:** Scale factor (e.g., `1.0`, `1.5`, `2.0`)

---

### `windowIsVisible()`

**Returns:** `1` if visible, `0` if hidden

---

### `windowRequestRedraw()`

Requests a redraw of the window contents.

**Returns:** Self

---

### `windowDrag()`

Initiates window dragging for custom title bars. Call from a mouse-down event handler.

**Returns:** Self

> **Note:** Not supported on Android.

---

### `windowSetAlwaysOnTop(bOnTop)`

Sets whether the window should stay above all other windows.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bOnTop` | Boolean | `true` to keep on top |

**Returns:** Self

> **Note:** Desktop only.

---

### `windowSetIcon(cIconPath)`

Sets the window icon from an image file.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cIconPath` | String | Path to icon file (PNG recommended) |

**Returns:** Self

> **Note:** Desktop only.

---

## Component Introspection Methods

### `definitionName()`

Gets the name of the loaded component.

**Returns:** Component name string

---

### `definitionProperties()`

Gets a list of all properties defined in the component.

**Returns:** List of `[name, type]` pairs

---

### `definitionCallbacks()`

Gets a list of all callback names defined in the component.

**Returns:** List of callback name strings

---

### `definitionFunctions()`

Gets a list of all public function names defined in the component.

**Returns:** List of function name strings

---

### `definitionGlobals()`

Gets a list of all global singleton names defined in the component.

**Returns:** List of global name strings

---

## File Dialog Methods

> **Note:** Desktop only. Not available on Android.

### `fileOpen(cTitle)`

Opens a file selection dialog.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cTitle` | String | Dialog title |

**Returns:** Selected file path, or empty string if cancelled

---

### `fileOpenWithFilters(cTitle, aFilters)`

Opens a file selection dialog with file type filters.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cTitle` | String | Dialog title |
| `aFilters` | List | Filter list: `[["Description", "ext1", "ext2"], ...]` |

**Returns:** Selected file path

**Example:**
```ring
cFile = oApp.fileOpenWithFilters("Open File", [
    ["Text Files", "txt", "md"],
    ["Ring Files", "ring"]
])
```

---

### `fileOpenMultiple(cTitle)`

Opens a dialog for selecting multiple files.

**Returns:** List of selected file paths

---

### `fileOpenMultipleWithFilters(cTitle, aFilters)`

Opens a multiple file dialog with filters.

**Returns:** List of selected file paths

---

### `fileSave(cTitle)`

Opens a file save dialog.

**Returns:** Selected save path

---

### `fileSaveWithName(cTitle, cDefaultName)`

Opens a file save dialog with a default filename.

**Returns:** Selected save path

---

### `fileSaveWithFilters(cTitle, cDefaultName, aFilters)`

Opens a file save dialog with default name and filters.

**Returns:** Selected save path

---

### `folderOpen(cTitle)`

Opens a folder selection dialog.

**Returns:** Selected folder path

---

### `folderOpenMultiple(cTitle)`

Opens a dialog for selecting multiple folders.

**Returns:** List of selected folder paths

---

## Message Dialog Methods

> **Note:** Desktop only. Not available on Android.

### `msgbox(cTitle, cMessage)`

Shows an information message box.

**Returns:** Self

---

### `msgboxWarning(cTitle, cMessage)`

Shows a warning message box.

**Returns:** Self

---

### `msgboxError(cTitle, cMessage)`

Shows an error message box.

**Returns:** Self

---

### `confirm(cTitle, cMessage)`

Shows a confirmation dialog with OK/Cancel buttons.

**Returns:** `1` if OK clicked, `0` if cancelled

---

### `yesno(cTitle, cMessage)`

Shows a Yes/No dialog.

**Returns:** `1` if Yes clicked, `0` if No clicked

---

## Notification Methods

> **Note:** Desktop only. Not available on Android.

### `notify(cSummary, cBody)`

Shows a desktop notification.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cSummary` | String | Notification title |
| `cBody` | String | Notification body text |

**Returns:** Self

---

### `notifyWithTimeout(cSummary, cBody, nTimeout)`

Shows a notification with a custom timeout.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nTimeout` | Number | Timeout in milliseconds |

**Returns:** Self

---

### `notifyWithIcon(cSummary, cBody, cIcon)`

Shows a notification with a custom icon.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cIcon` | String | Path to icon file |

**Returns:** Self

---

### `notifyFull(cSummary, cBody, cIcon, nTimeout)`

Shows a notification with icon and timeout.

**Returns:** Self

---

## Clipboard Methods

> **Note:** Desktop only. Not available on Android.

### `clipboardGet()`

Gets the current clipboard text content.

**Returns:** Clipboard text string

---

### `clipboardSet(cText)`

Sets the clipboard text content.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cText` | String | Text to copy |

**Returns:** Self

---

### `clipboardClear()`

Clears the clipboard contents.

**Returns:** Self

---

## Hotkey Methods

> **Note:** Desktop only. Not available on Android.

### `hotkeyRegister(cModifiers, cKey, cCallback)`

Registers a global hotkey that works even when the app is not focused.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cModifiers` | String | Modifiers: `"ctrl"`, `"alt"`, `"shift"`, `"super"`. Combine with `+`: `"ctrl+shift"` |
| `cKey` | String | Key code: `"KeyA"`, `"KeyB"`, `"F1"`, `"Space"`, `"Enter"`, etc. |
| `cCallback` | String | Ring function to call |

**Returns:** Hotkey ID

**Example:**
```ring
nHotkey = oApp.hotkeyRegister("ctrl+shift", "KeyS", :onSaveHotkey)

func onSaveHotkey
    ? "Save hotkey pressed!"
```

---

### `hotkeyUnregister(nId)`

Unregisters a hotkey.

| Parameter | Type | Description |
|-----------|------|-------------|
| `nId` | Number | Hotkey ID |

**Returns:** Self

---

### `hotkeyUnregisterAll()`

Unregisters all hotkeys.

**Returns:** Self

---

### `hotkeyPoll()`

Polls for hotkey events. Call periodically (e.g., in a timer) to check for hotkey presses.

**Returns:** Self

---

## System Tray Methods

> **Note:** Desktop only. Not available on Android.
> 
> **Platform implementations:**
> - **Linux/BSD:** Uses ksni (D-Bus/SNI protocol)
> - **Windows/macOS:** Uses tray-icon (native APIs)

### `trayCreate(cTooltip)`

Creates a system tray icon with a tooltip.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cTooltip` | String | Tooltip text |

**Returns:** Self

---

### `trayCreateWithIcon(cTooltip, cIconPath)`

Creates a system tray icon with tooltip and custom icon.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cTooltip` | String | Tooltip text |
| `cIconPath` | String | Path to icon file |

**Returns:** Self

---

### `traySetIcon(cIconPath)`

Sets the tray icon image.

**Returns:** Self

---

### `traySetTooltip(cTooltip)`

Sets the tray icon tooltip.

**Returns:** Self

---

### `trayAddItem(cLabel, cCallback)`

Adds a menu item to the tray context menu.

| Parameter | Type | Description |
|-----------|------|-------------|
| `cLabel` | String | Menu item label |
| `cCallback` | String | Ring function to call |

**Returns:** Menu item ID

---

### `trayAddSeparator()`

Adds a separator line to the tray menu.

**Returns:** Self

---

### `trayDestroy()`

Destroys the tray icon and menu.

**Returns:** Self

---

### `trayPoll()`

Polls for tray menu events. Must be called periodically to process menu item clicks.

**Returns:** Self

**Example:**
```ring
oApp.trayCreate("My App")
oApp.trayAddItem("Show", :onShow)
oApp.trayAddSeparator()
oApp.trayAddItem("Quit", :onQuit)

// Poll in timer
oApp.timerStart(100, :pollTray)

func pollTray
    oApp.trayPoll()
```
