# Registry Tree Dependency

Each dependency level is shown with progressive bullet points: * for components, ** for their dependencies, *** for nested dependencies.

```
* use_breadcrumb (hooks)
* use_can_scroll (hooks)
* use_can_scroll_vertical (hooks)
* use_card_carousel (hooks)
  ** cargo: wasm_bindgen
  ** cargo: web_sys
* use_cell_edit (hooks)
  ** data_grid
* use_cell_selection (hooks)
  ** data_grid
* use_click_outside (hooks)
  ** cargo: wasm_bindgen
* use_column_state (hooks)
  ** cargo: strum
  ** data_grid
* use_copy_clipboard (hooks)
  ** cargo: wasm_bindgen
* use_data_grid_state (hooks)
  ** data_grid
  ** use_cell_selection (hooks)
    *** data_grid
  ** use_click_outside (hooks)
    *** cargo: wasm_bindgen
  ** use_drag_selection (hooks)
    *** data_grid
* use_data_scrolled (hooks)
  ** cargo: wasm_bindgen
* use_drag_selection (hooks)
  ** data_grid
* use_form (hooks)
  ** cargo: serde
* use_handle_day_click (hooks)
  ** cargo: time
* use_history (hooks)
  ** cargo: wasm_bindgen
  ** cargo: web_sys
* use_history_stack (hooks)
* use_horizontal_scroll (hooks)
  ** cargo: strum
  ** cargo: wasm_bindgen
* use_input_otp (hooks)
  ** cargo: wasm_bindgen
  ** cargo: web_sys
* use_is_mobile (hooks)
* use_lock_body_scroll (hooks)
* use_lock_body_scroll_dialog (hooks)
  ** cargo: wasm_bindgen
* use_lock_body_scroll_popover (hooks)
  ** cargo: wasm_bindgen
* use_locks (hooks)
* use_media_query (hooks)
  ** cargo: wasm_bindgen
* use_pagination (hooks)
* use_press_hold (hooks)
  ** cargo: wasm_bindgen
* use_random (hooks)
* use_scroll_lock (hooks)
  ** cargo: wasm_bindgen
* use_stepper (hooks)
* use_theme_mode (hooks)
* use_virtual_scroll (hooks)
  ** cargo: wasm_bindgen
* use_workflow (hooks)
  ** cargo: serde
  ** cargo: strum
```
