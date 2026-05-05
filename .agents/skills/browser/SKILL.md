---
name: browser
description: Control a real browser over Chrome DevTools Protocol (CDP). Use when user asks to open a page, click an element, take a screenshot, or any browser automation. Requires Chrome remote debugging enabled.
license: MIT
allowed-tools: chrome-devtools_*
---

# Browser Control via CDP

Control a real browser over Chrome DevTools Protocol (CDP).

## Enable Remote Debugging

**Chrome/Chromium**: Open `chrome://inspect/#remote-debugging` or launch with `--remote-debugging-port=9222`

## Workflow

1. **Navigate**: `chrome-devtools_navigate_page` to open URL
2. **Inspect**: `chrome-devtools_take_snapshot` to see clickable elements (uid)
3. **Interact**: Use element uid from snapshot to click, hover, fill, etc.
4. **Verify**: Take snapshot or screenshot to confirm

## Key Tools

| Tool | Purpose |
|------|---------|
| `chrome-devtools_list_pages` | List open tabs |
| `chrome-devtools_navigate_page` | Navigate to URL |
| `chrome-devtools_take_snapshot` | Get clickable elements with uids |
| `chrome-devtools_click` | Click element by uid |
| `chrome-devtools_fill` | Type into input |
| `chrome-devtools_take_screenshot` | Screenshot |
| `chrome-devtools_hover` | Hover over element |
| `chrome-devtools_press_key` | Send keyboard keys |
| `chrome-devtools_select_page` | Switch to tab by pageId |

## Example

```
chrome-devtools_navigate_page(url="https://github.com")
chrome-devtools_take_snapshot()  # returns elements with uids
chrome-devtools_click(uid="abc123")  # click a link/button
```

## Tips

- Always take snapshot first to get element uids
- Use `type_text` after clicking an input to focus it first
- For screenshots, use `format="jpeg"` or `format="webp"` for smaller output
- Check `chrome-devtools_list_network_requests` for debugging