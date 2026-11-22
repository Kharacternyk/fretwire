local api = vim.api

local function blend(fg, bg, opacity)
  local color = 0
  for _, place in ipairs({ 0x10000, 0x100, 1 }) do
    local foreground = math.floor(fg / place) % 256
    local background = math.floor(bg / place) % 256
    local channel = math.floor(background + (foreground - background) * opacity + 0.5)
    color = color + channel * place
  end
  return color
end

return function()
  for group, link in pairs({
    fretwireMovedLine = "DiagnosticSignWarn",
    fretwireDeletedLine = "DiagnosticSignError",
    fretwireMoveMarker = "Keyword",
    fretwireDestination = "Constant",
  }) do
    api.nvim_set_hl(0, group, { default = true, link = link })
  end
  local normal = api.nvim_get_hl(0, { name = "Normal", link = false })
  local dark = vim.o.background == "dark"
  local fg = normal.fg or (dark and 0xffffff or 0x000000)
  local bg = normal.bg or (dark and 0x000000 or 0xffffff)
  if normal.reverse then
    fg, bg = bg, fg
  end
  for bucket = 1, 4 do
    local opacity = 0.35 + (bucket - 1) * 0.65 / 4
    api.nvim_set_hl(0, "fretwireAge" .. bucket, { fg = blend(fg, bg, opacity) })
  end
  api.nvim_set_hl(0, "fretwireAge5", { fg = fg })
end
