# This file is part of the Ring Slint library.

/*
 * Ring Slint Library
 * 
 * Provides bindings for the Slint GUI toolkit in the Ring programming language.
 * Slint is a declarative GUI toolkit for building native user interfaces.
 * 
 * This library supports:
 *   - Loading Slint files (.slint) or inline Slint markup
 *   - Property binding and manipulation
 *   - Callback registration between Slint and Ring
 *   - Timer management
 *   - Model-based data binding for lists/repeaters
 *   - Window management (position, size, fullscreen, etc.)
 *   - File dialogs, message boxes, and notifications
 *   - Clipboard operations
 *   - Global hotkey registration
 *   - System tray integration
 */

/**
 * Class SlintApp: Main class for creating and managing Slint GUI applications.
 * 
 * Provides methods for loading UI definitions, manipulating properties,
 * handling callbacks, managing windows, and interacting with system features
 * like file dialogs, notifications, clipboard, hotkeys, and system tray.
 * 
 * Example usage:
 *   load "slint.ring"
 *   
 *   oApp = new SlintApp {
 *       loadUI("main.slint")
 *       set("greeting", "Hello, World!")
 *       setCallback("button-clicked", :onButtonClick)
 *       show()
 *       run()
 *   }
 *   
 *   func onButtonClick
 *       oApp.set("greeting", "Button was clicked!")
 */
class SlintApp

    self.pComponent = NULL
    self.pWindow = NULL

    /**
     * Loads and compiles a Slint definition from a file, then creates a window instance.
     * The first component found in the file will be used.
     * On Android, paths are resolved relative to the assets directory.
     * @param cFile Path to the .slint file.
     * @return Self for method chaining.
     * @raises Error if compilation fails or no component is found.
     */
    func loadUI cFile
        pComponent = slint_load(cFile)
        pWindow = slint_create(pComponent)
        return self

    /**
     * Loads and compiles a Slint definition from source code string, then creates a window instance.
     * The first component found in the source will be used.
     * @param cSource The Slint markup source code.
     * @param cPath Virtual path for resolving relative imports (e.g., "virtual/path.slint").
     * @return Self for method chaining.
     * @raises Error if compilation fails or no component is found.
     */
    func loadUIString cSource, cPath
        pComponent = slint_loadstring(cSource, cPath)
        pWindow = slint_create(pComponent)
        return self

    /**
     * Creates a window instance from the loaded component definition.
     * Call this after loadUI/loadUIString if you need to recreate the window.
     * @return Self for method chaining.
     */
    func create
        if pComponent != NULL
            pWindow = slint_create(pComponent)
        ok
        return self

    /**
     * Sets a property value on the Slint component.
     * @param cProp Property name (e.g., "title", "counter", "items").
     * @param value The value to set (string, number, or list).
     * @return Self for method chaining.
     */
    func set cProp, value
        if pWindow != NULL
            slint_set(pWindow, cProp, value)
        ok
        return self

    /**
     * Sets a boolean property value on the Slint component.
     * @param cProp Property name.
     * @param bValue Boolean value (true/false or 1/0).
     * @return Self for method chaining.
     */
    func setBool cProp, bValue
        if pWindow != NULL
            slint_set_bool(pWindow, cProp, bValue)
        ok
        return self

    /**
     * Sets an image property from a file path.
     * Supports PNG, JPEG, BMP, and other formats.
     * @param cProp Property name.
     * @param cPath Path to the image file.
     * @return Self for method chaining.
     */
    func setImage cProp, cPath
        if pWindow != NULL
            slint_set_image(pWindow, cProp, cPath)
        ok
        return self

    /**
     * Gets the current value of a property.
     * @param cProp Property name.
     * @return The property value, or NULL if window not initialized.
     */
    func getProperty cProp
        if pWindow != NULL
            return slint_get(pWindow, cProp)
        ok
        return NULL

    /**
     * Registers a Ring function as a callback for a Slint callback.
     * @param cCallback Name of the callback defined in Slint (e.g., "clicked").
     * @param cRingFunc Name of the Ring function to call.
     * @return Self for method chaining.
     */
    func setCallback cCallback, cRingFunc
        if pWindow != NULL
            slint_on(pWindow, cCallback, cRingFunc)
        ok
        return self

    /**
     * Invokes a Slint function/callback programmatically.
     * @param cCallback Name of the function to invoke.
     * @param aArgs Optional list of arguments to pass.
     * @return The return value from the Slint function, or NULL.
     */
    func invoke cCallback, aArgs
        if pWindow != NULL
            if isList(aArgs)
                return slint_invoke(pWindow, cCallback, aArgs)
            ok
            return slint_invoke(pWindow, cCallback)
        ok
        return NULL

    /**
     * Gets an argument passed to the current callback.
     * Use inside a callback function to retrieve arguments from Slint.
     * Arguments are stored thread-locally and are only valid during callback execution.
     * @param nIndex 1-based index of the argument (first argument is 1).
     * @return The argument value, or 0 if index is out of bounds.
     */
    func callbackArg nIndex
        return slint_callback_arg(nIndex)

    /**
     * Gets the number of arguments passed to the current callback.
     * @return Number of arguments.
     */
    func callbackArgsCount
        return slint_callback_args_count()

    /**
     * Gets a property value from a Slint global singleton.
     * @param cGlobal Name of the global (e.g., "AppState").
     * @param cProp Property name within the global.
     * @return The property value, or NULL.
     */
    func globalGet cGlobal, cProp
        if pWindow != NULL
            return slint_global_get(pWindow, cGlobal, cProp)
        ok
        return NULL

    /**
     * Sets a property value on a Slint global singleton.
     * @param cGlobal Name of the global.
     * @param cProp Property name.
     * @param value The value to set.
     * @return Self for method chaining.
     */
    func globalSet cGlobal, cProp, value
        if pWindow != NULL
            slint_global_set(pWindow, cGlobal, cProp, value)
        ok
        return self

    /**
     * Registers a callback on a Slint global singleton.
     * @param cGlobal Name of the global.
     * @param cCallback Callback name.
     * @param cRingFunc Ring function to call.
     * @return Self for method chaining.
     */
    func globalSetCallback cGlobal, cCallback, cRingFunc
        if pWindow != NULL
            slint_global_on(pWindow, cGlobal, cCallback, cRingFunc)
        ok
        return self

    /**
     * Invokes a function on a Slint global singleton.
     * @param cGlobal Name of the global.
     * @param cCallback Function name.
     * @param aArgs Optional arguments.
     * @return The return value, or NULL.
     */
    func globalInvoke cGlobal, cCallback, aArgs
        if pWindow != NULL
            if isList(aArgs)
                return slint_global_invoke(pWindow, cGlobal, cCallback, aArgs)
            ok
            return slint_global_invoke(pWindow, cGlobal, cCallback)
        ok
        return NULL

    /**
     * Shows the window.
     * @return Self for method chaining.
     */
    func show
        if pWindow != NULL
            slint_show(pWindow)
        ok
        return self

    /**
     * Hides the window without destroying it.
     * @return Self for method chaining.
     */
    func hide
        if pWindow != NULL
            slint_hide(pWindow)
        ok
        return self

    /**
     * Runs the application event loop (blocking call).
     * This will block until the window is closed or quit() is called.
     */
    func run
        if pWindow != NULL
            slint_run(pWindow)
        ok

    /**
     * Quits the application and exits the event loop.
     */
    func quit
        slint_quit()

    /**
     * Gets the native window pointer.
     * @return The window pointer.
     */
    func window
        return pWindow

    /**
     * Gets the component definition pointer.
     * @return The component definition pointer.
     */
    func definition
        return pComponent

    /*
     * ========================================
     * Timer Functions
     * ========================================
     */

    /**
     * Starts a repeating timer.
     * @param nInterval Interval in milliseconds.
     * @param cCallback Ring function to call on each tick.
     * @return Timer ID for later reference.
     */
    func timerStart nInterval, cCallback
        return slint_timer_start(nInterval, cCallback)

    /**
     * Starts a one-shot timer that fires only once.
     * @param nInterval Delay in milliseconds.
     * @param cCallback Ring function to call when timer fires.
     * @return Timer ID.
     */
    func timerStartOnce nInterval, cCallback
        return slint_timer_start(nInterval, cCallback, 0)

    /**
     * Stops a running timer.
     * @param nTimerId Timer ID returned from timerStart.
     * @return Result of the operation.
     */
    func timerStop nTimerId
        return slint_timer_stop(nTimerId)

    /**
     * Checks if a timer is currently running.
     * @param nTimerId Timer ID.
     * @return 1 if running, 0 if stopped.
     */
    func timerRunning nTimerId
        return slint_timer_running(nTimerId)

    /**
     * Restarts a stopped or running timer.
     * @param nTimerId Timer ID.
     * @return Result of the operation.
     */
    func timerRestart nTimerId
        return slint_timer_restart(nTimerId)

    /**
     * Changes the interval of an existing timer.
     * @param nTimerId Timer ID.
     * @param nInterval New interval in milliseconds.
     * @return Result of the operation.
     */
    func timerSetInterval nTimerId, nInterval
        return slint_timer_set_interval(nTimerId, nInterval)

    /*
     * ========================================
     * Model Functions (for lists/repeaters)
     * ========================================
     */

    /**
     * Creates a new model and binds it to a property.
     * Models are used to populate Slint repeaters/lists with dynamic data.
     * @param cProp Property name to bind the model to.
     * @return Model ID, or -1 on failure.
     */
    func modelCreate cProp
        if pWindow != NULL
            return slint_model_create(pWindow, cProp)
        ok
        return -1

    /**
     * Appends an item to the end of a model.
     * @param nModelId Model ID.
     * @param value Item value (typically a list/struct).
     * @return Result of the operation.
     */
    func modelPush nModelId, value
        return slint_model_push(nModelId, value)

    /**
     * Removes an item from a model by index.
     * @param nModelId Model ID.
     * @param nIndex Zero-based index of item to remove.
     * @return Result of the operation.
     */
    func modelRemove nModelId, nIndex
        return slint_model_remove(nModelId, nIndex)

    /**
     * Updates an item in a model at a specific index.
     * @param nModelId Model ID.
     * @param nIndex Zero-based index.
     * @param value New value for the item.
     * @return Result of the operation.
     */
    func modelSet nModelId, nIndex, value
        return slint_model_set(nModelId, nIndex, value)

    /**
     * Gets the number of items in a model.
     * @param nModelId Model ID.
     * @return Number of items.
     */
    func modelCount nModelId
        return slint_model_count(nModelId)

    /**
     * Removes all items from a model.
     * @param nModelId Model ID.
     * @return Result of the operation.
     */
    func modelClear nModelId
        return slint_model_clear(nModelId)

    /**
     * Inserts an item at a specific index in a model.
     * @param nModelId Model ID.
     * @param nIndex Zero-based index for insertion.
     * @param value Item value.
     * @return Result of the operation.
     */
    func modelInsert nModelId, nIndex, value
        return slint_model_insert(nModelId, nIndex, value)

    /**
     * Destroys a model and releases its resources.
     * @param nModelId Model ID.
     * @return Result of the operation.
     */
    func modelDestroy nModelId
        return slint_model_destroy(nModelId)

    /**
     * Gets an item from a model by index.
     * @param nModelId Model ID.
     * @param nIndex Zero-based index.
     * @return The item value.
     */
    func modelGet nModelId, nIndex
        return slint_model_get(nModelId, nIndex)

    /*
     * ========================================
     * Style Functions
     * ========================================
     */

    /**
     * Sets the widget style for subsequent compilations.
     * Must be called BEFORE loadUI/loadUIString to take effect.
     * Pass empty string to reset to default style.
     * @param cStyle Style name (e.g., "fluent", "material", "native", "cupertino").
     * @return Self for method chaining.
     */
    func setStyle cStyle
        slint_set_style(cStyle)
        return self

    /**
     * Gets the current widget style setting.
     * @return Current style name, or NULL if using default.
     */
    func getStyle
        return slint_get_style()

    /**
     * Adds a library path for @library imports in Slint files (e.g., @material).
     * Must be called BEFORE loadUI/loadUIString to take effect.
     * @param cName Library name (used in @name imports).
     * @param cPath Filesystem path to the library directory.
     * @return Self for method chaining.
     */
    func addLibraryPath cName, cPath
        slint_add_library_path(cName, cPath)
        return self

    /**
     * Removes a previously added library path.
     * @param cName Library name.
     * @return Self for method chaining.
     */
    func removeLibraryPath cName
        slint_remove_library_path(cName)
        return self

    /**
     * Clears all custom library paths.
     * @return Self for method chaining.
     */
    func clearLibraryPaths
        slint_clear_library_paths()
        return self

    /*
     * ========================================
     * Window Management Functions
     * ========================================
     */

    /**
     * Minimizes or restores the window.
     * @param bMinimized True to minimize, false to restore.
     * @return Self for method chaining.
     */
    func windowSetMinimized bMinimized
        if pWindow != NULL
            slint_window_set_minimized(pWindow, bMinimized)
        ok
        return self

    /**
     * Checks if the window is minimized.
     * @return 1 if minimized, 0 otherwise.
     */
    func windowIsMinimized
        if pWindow != NULL
            return slint_window_is_minimized(pWindow)
        ok
        return 0

    /**
     * Maximizes or restores the window.
     * @param bMaximized True to maximize, false to restore.
     * @return Self for method chaining.
     */
    func windowSetMaximized bMaximized
        if pWindow != NULL
            slint_window_set_maximized(pWindow, bMaximized)
        ok
        return self

    /**
     * Checks if the window is maximized.
     * @return 1 if maximized, 0 otherwise.
     */
    func windowIsMaximized
        if pWindow != NULL
            return slint_window_is_maximized(pWindow)
        ok
        return 0

    /**
     * Enables or disables fullscreen mode.
     * @param bFullscreen True for fullscreen, false for windowed.
     * @return Self for method chaining.
     */
    func windowSetFullscreen bFullscreen
        if pWindow != NULL
            slint_window_set_fullscreen(pWindow, bFullscreen)
        ok
        return self

    /**
     * Checks if the window is in fullscreen mode.
     * @return 1 if fullscreen, 0 otherwise.
     */
    func windowIsFullscreen
        if pWindow != NULL
            return slint_window_is_fullscreen(pWindow)
        ok
        return 0

    /**
     * Moves the window to a specific screen position using physical (pixel) coordinates.
     * @param nX X coordinate in physical pixels.
     * @param nY Y coordinate in physical pixels.
     * @return Self for method chaining.
     */
    func windowSetPosition nX, nY
        if pWindow != NULL
            slint_window_set_position(pWindow, nX, nY)
        ok
        return self

    /**
     * Gets the current window position in physical (pixel) coordinates.
     * @return List [x, y] with window coordinates in physical pixels.
     */
    func windowGetPosition
        if pWindow != NULL
            return slint_window_get_position(pWindow)
        ok
        return [0, 0]

    /**
     * Sets the window size using physical (pixel) dimensions.
     * @param nWidth Width in physical pixels.
     * @param nHeight Height in physical pixels.
     * @return Self for method chaining.
     */
    func windowSetSize nWidth, nHeight
        if pWindow != NULL
            slint_window_set_size(pWindow, nWidth, nHeight)
        ok
        return self

    /**
     * Gets the current window size in physical (pixel) dimensions.
     * @return List [width, height] in physical pixels.
     */
    func windowGetSize
        if pWindow != NULL
            return slint_window_get_size(pWindow)
        ok
        return [0, 0]

    /**
     * Gets the window's scale factor (for HiDPI displays).
     * @return Scale factor (e.g., 1.0, 1.5, 2.0).
     */
    func windowScaleFactor
        if pWindow != NULL
            return slint_window_scale_factor(pWindow)
        ok
        return 1.0

    /**
     * Checks if the window is currently visible.
     * @return 1 if visible, 0 if hidden.
     */
    func windowIsVisible
        if pWindow != NULL
            return slint_window_is_visible(pWindow)
        ok
        return 0

    /**
     * Requests a redraw of the window contents.
     * @return Self for method chaining.
     */
    func windowRequestRedraw
        if pWindow != NULL
            slint_window_request_redraw(pWindow)
        ok
        return self

    /**
     * Initiates window dragging (for custom title bars).
     * Call this from a mouse-down event handler on your custom title bar area.
     * Uses winit's drag_window() internally. On Wayland, automatically resets
     * pointer state since the compositor steals the pointer during drag.
     * Desktop only (not available on Android).
     * @return Self for method chaining.
     */
    func windowDrag
        if pWindow != NULL
            slint_window_drag(pWindow)
        ok
        return self

    /**
     * Sets whether the window should stay above all other windows.
     * Uses winit's WindowLevel::AlwaysOnTop internally.
     * Desktop only (not supported on Android).
     * @param bOnTop True to keep on top, false for normal behavior.
     * @return Self for method chaining.
     */
    func windowSetAlwaysOnTop bOnTop
        if pWindow != NULL
            slint_window_set_always_on_top(pWindow, bOnTop)
        ok
        return self

    /**
     * Sets the window icon from an image file.
     * Supports PNG, JPEG, and other formats via the image crate.
     * The image is converted to RGBA8 format internally.
     * Desktop only (not supported on Android).
     * @param cIconPath Path to the icon file (PNG recommended).
     * @return Self for method chaining.
     */
    func windowSetIcon cIconPath
        if pWindow != NULL
            slint_window_set_icon(pWindow, cIconPath)
        ok
        return self

    /*
     * ========================================
     * Component Definition Introspection
     * ========================================
     */

    /**
     * Gets the name of the loaded component.
     * @return Component name string.
     */
    func definitionName
        if pComponent != NULL
            return slint_definition_name(pComponent)
        ok
        return ""

    /**
     * Gets a list of all properties defined in the component.
     * Each property is returned as [name, type] where type is a string
     * representation of the Slint value type (e.g., "String", "Int", "Bool").
     * @return List of [name, type] pairs for each property.
     */
    func definitionProperties
        if pComponent != NULL
            return slint_definition_properties(pComponent)
        ok
        return []

    /**
     * Gets a list of all callback names defined in the component.
     * @return List of callback name strings.
     */
    func definitionCallbacks
        if pComponent != NULL
            return slint_definition_callbacks(pComponent)
        ok
        return []

    /**
     * Gets a list of all public function names defined in the component.
     * @return List of function name strings.
     */
    func definitionFunctions
        if pComponent != NULL
            return slint_definition_functions(pComponent)
        ok
        return []

    /**
     * Gets a list of all global singleton names defined in the component.
     * @return List of global name strings.
     */
    func definitionGlobals
        if pComponent != NULL
            return slint_definition_globals(pComponent)
        ok
        return []

    /*
     * ========================================
     * File Dialog Functions (Desktop only)
     * ========================================
     */

    /**
     * Opens a file selection dialog.
     * @param cTitle Dialog title.
     * @return Selected file path, or empty string if cancelled.
     */
    func fileOpen cTitle
        return slint_file_open(cTitle)

    /**
     * Opens a file selection dialog with file type filters.
     * Uses rfd (Rust File Dialog) library internally.
     * Desktop only (not available on Android).
     * @param cTitle Dialog title.
     * @param aFilters List of filter lists: [["Description", "ext1", "ext2"], ...].
     *                 Example: [["Text Files", "txt"], ["Ring Files", "ring"]]
     * @return Selected file path, or empty if cancelled.
     */
    func fileOpenWithFilters cTitle, aFilters
        return slint_file_open(cTitle, aFilters)

    /**
     * Opens a dialog for selecting multiple files.
     * @param cTitle Dialog title.
     * @return List of selected file paths.
     */
    func fileOpenMultiple cTitle
        return slint_file_open_multiple(cTitle)

    /**
     * Opens a multiple file dialog with filters.
     * @param cTitle Dialog title.
     * @param aFilters List of filter strings.
     * @return List of selected file paths.
     */
    func fileOpenMultipleWithFilters cTitle, aFilters
        return slint_file_open_multiple(cTitle, aFilters)

    /**
     * Opens a file save dialog.
     * @param cTitle Dialog title.
     * @return Selected save path.
     */
    func fileSave cTitle
        return slint_file_save(cTitle)

    /**
     * Opens a file save dialog with a default filename.
     * @param cTitle Dialog title.
     * @param cDefaultName Default filename.
     * @return Selected save path.
     */
    func fileSaveWithName cTitle, cDefaultName
        return slint_file_save(cTitle, cDefaultName)

    /**
     * Opens a file save dialog with default name and filters.
     * @param cTitle Dialog title.
     * @param cDefaultName Default filename.
     * @param aFilters List of filter strings.
     * @return Selected save path.
     */
    func fileSaveWithFilters cTitle, cDefaultName, aFilters
        return slint_file_save(cTitle, cDefaultName, aFilters)

    /**
     * Opens a folder selection dialog.
     * @param cTitle Dialog title.
     * @return Selected folder path.
     */
    func folderOpen cTitle
        return slint_folder_open(cTitle)

    /**
     * Opens a dialog for selecting multiple folders.
     * @param cTitle Dialog title.
     * @return List of selected folder paths.
     */
    func folderOpenMultiple cTitle
        return slint_folder_open_multiple(cTitle)

    /*
     * ========================================
     * Message Dialog Functions (Desktop only)
     * ========================================
     */

    /**
     * Shows an information message box.
     * @param cTitle Dialog title.
     * @param cMessage Message text.
     * @return Self for method chaining.
     */
    func msgbox cTitle, cMessage
        slint_msgbox(cTitle, cMessage)
        return self

    /**
     * Shows a warning message box.
     * @param cTitle Dialog title.
     * @param cMessage Message text.
     * @return Self for method chaining.
     */
    func msgboxWarning cTitle, cMessage
        slint_msgbox(cTitle, cMessage, "warning")
        return self

    /**
     * Shows an error message box.
     * @param cTitle Dialog title.
     * @param cMessage Message text.
     * @return Self for method chaining.
     */
    func msgboxError cTitle, cMessage
        slint_msgbox(cTitle, cMessage, "error")
        return self

    /**
     * Shows a confirmation dialog with OK/Cancel buttons.
     * @param cTitle Dialog title.
     * @param cMessage Message text.
     * @return 1 if OK clicked, 0 if cancelled.
     */
    func confirm cTitle, cMessage
        return slint_confirm(cTitle, cMessage)

    /**
     * Shows a Yes/No dialog.
     * @param cTitle Dialog title.
     * @param cMessage Message text.
     * @return 1 if Yes clicked, 0 if No clicked.
     */
    func yesno cTitle, cMessage
        return slint_yesno(cTitle, cMessage)

    /*
     * ========================================
     * Notification Functions (Desktop only)
     * ========================================
     */

    /**
     * Shows a desktop notification.
     * @param cSummary Notification title/summary.
     * @param cBody Notification body text.
     * @return Self for method chaining.
     */
    func notify cSummary, cBody
        slint_notify(cSummary, cBody)
        return self

    /**
     * Shows a notification with a custom timeout.
     * @param cSummary Notification title.
     * @param cBody Notification body.
     * @param nTimeout Timeout in milliseconds.
     * @return Self for method chaining.
     */
    func notifyWithTimeout cSummary, cBody, nTimeout
        slint_notify_with_timeout(cSummary, cBody, nTimeout)
        return self

    /**
     * Shows a notification with a custom icon.
     * @param cSummary Notification title.
     * @param cBody Notification body.
     * @param cIcon Path to icon file.
     * @return Self for method chaining.
     */
    func notifyWithIcon cSummary, cBody, cIcon
        slint_notify_with_icon(cSummary, cBody, cIcon)
        return self

    /**
     * Shows a notification with icon and timeout.
     * @param cSummary Notification title.
     * @param cBody Notification body.
     * @param cIcon Path to icon file.
     * @param nTimeout Timeout in milliseconds.
     * @return Self for method chaining.
     */
    func notifyFull cSummary, cBody, cIcon, nTimeout
        slint_notify_full(cSummary, cBody, cIcon, nTimeout)
        return self

    /*
     * ========================================
     * Clipboard Functions
     * ========================================
     */

    /**
     * Gets the current clipboard text content.
     * @return Clipboard text string.
     */
    func clipboardGet
        return slint_clipboard_get()

    /**
     * Sets the clipboard text content.
     * @param cText Text to copy to clipboard.
     * @return Self for method chaining.
     */
    func clipboardSet cText
        slint_clipboard_set(cText)
        return self

    /**
     * Clears the clipboard contents.
     * @return Self for method chaining.
     */
    func clipboardClear
        slint_clipboard_clear()
        return self

    /*
     * ========================================
     * Global Hotkey Functions
     * ========================================
     */

    /**
     * Registers a global hotkey that works even when the app is not focused.
     * Uses the global-hotkey crate internally.
     * Desktop only (not available on Android).
     * @param cModifiers Modifier keys: "ctrl", "alt", "shift", "super"/"meta"/"win".
     *                   Can combine with "+": "ctrl+shift", "ctrl+alt".
     * @param cKey Key code name (e.g., "KeyA", "KeyB", "F1", "Space", "Enter").
     *             Uses standard key code names from global-hotkey crate.
     * @param cCallback Ring function to call when hotkey is pressed.
     * @return Hotkey ID for use with hotkeyUnregister.
     */
    func hotkeyRegister cModifiers, cKey, cCallback
        return slint_hotkey_register(cModifiers, cKey, cCallback)

    /**
     * Unregisters a hotkey.
     * @param nId Hotkey ID from hotkeyRegister.
     * @return Self for method chaining.
     */
    func hotkeyUnregister nId
        slint_hotkey_unregister(nId)
        return self

    /**
     * Unregisters all hotkeys.
     * @return Self for method chaining.
     */
    func hotkeyUnregisterAll
        slint_hotkey_unregister_all()
        return self

    /**
     * Polls for hotkey events.
     * Call this periodically (e.g., in a timer) to check for hotkey presses.
     * @return Self for method chaining.
     */
    func hotkeyPoll
        slint_hotkey_poll()
        return self

    /*
     * ========================================
     * System Tray Functions
     * ========================================
     * Platform implementations:
     * - Linux/BSD: Uses ksni (D-Bus/SNI protocol, pure Rust, no GTK conflict)
     * - Windows/macOS: Uses tray-icon (native APIs)
     * - Android: Not supported
     */

    /**
     * Creates a system tray icon with a tooltip.
     * On Linux, uses ksni with D-Bus/SNI protocol.
     * On Windows/macOS, uses native tray-icon APIs.
     * Not supported on Android.
     * @param cTooltip Tooltip text shown on hover.
     * @return Self for method chaining.
     */
    func trayCreate cTooltip
        slint_tray_create(cTooltip)
        return self

    /**
     * Creates a system tray icon with tooltip and custom icon.
     * On Windows/macOS, loads the icon from the file path.
     * On Linux with ksni, custom icons may have limited support.
     * Not supported on Android.
     * @param cTooltip Tooltip text.
     * @param cIconPath Path to icon file (PNG recommended).
     * @return Self for method chaining.
     */
    func trayCreateWithIcon cTooltip, cIconPath
        slint_tray_create_with_icon(cTooltip, cIconPath)
        return self

    /**
     * Sets the tray icon image.
     * @param cIconPath Path to icon file.
     * @return Self for method chaining.
     */
    func traySetIcon cIconPath
        slint_tray_set_icon(cIconPath)
        return self

    /**
     * Sets the tray icon tooltip.
     * @param cTooltip Tooltip text.
     * @return Self for method chaining.
     */
    func traySetTooltip cTooltip
        slint_tray_set_tooltip(cTooltip)
        return self

    /**
     * Adds a menu item to the tray context menu.
     * @param cLabel Menu item label.
     * @param cCallback Ring function to call when clicked.
     * @return Menu item ID.
     */
    func trayAddItem cLabel, cCallback
        return slint_tray_add_item(cLabel, cCallback)

    /**
     * Adds a separator line to the tray menu.
     * @return Self for method chaining.
     */
    func trayAddSeparator
        slint_tray_add_separator()
        return self

    /**
     * Destroys the tray icon and menu.
     * @return Self for method chaining.
     */
    func trayDestroy
        slint_tray_destroy()
        return self

    /**
     * Polls for tray menu events and dispatches callbacks.
     * Must be called periodically (e.g., in a timer) to process menu item clicks.
     * On Linux/ksni: Checks pending activation queue.
     * On Windows/macOS: Receives events from MenuEvent channel.
     * @return Self for method chaining.
     */
    func trayPoll
        slint_tray_poll()
        return self
