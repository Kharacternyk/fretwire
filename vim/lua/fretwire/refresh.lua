local api = vim.api
local age = require("fretwire.age")
local blocks = require("fretwire.blocks")
local config = require("fretwire.config")
local highlight_move = require("fretwire.highlight_move")
local marks = require("fretwire.marks")

return function(buf, first, last)
  buf = buf or api.nvim_get_current_buf()
  if not api.nvim_buf_is_loaded(buf) then
    return
  end
  if first then
    first, last = blocks.range(buf, first, last)
  else
    first, last = 0, -1
  end
  marks.clear(buf, first, last)
  local move_marker = config.option(buf, "move_marker")
  local timestamp_marker = config.option(buf, "timestamp_marker")
  local pattern = config.option(buf, "timestamp_pattern")
  local entries = {}
  for index, line in ipairs(api.nvim_buf_get_lines(buf, first, last, false)) do
    local row = first + index - 1
    if blocks.is_separator(line) then
      age.highlight(buf, entries)
      entries = {}
    elseif not highlight_move(buf, row, line, move_marker) then
      age.collect(entries, row, line, timestamp_marker, pattern)
    end
  end
  age.highlight(buf, entries)
end
