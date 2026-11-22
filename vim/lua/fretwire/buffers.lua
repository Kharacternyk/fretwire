local api = vim.api
local changed_range = require("fretwire.changed_range")
local marks = require("fretwire.marks")
local palette = require("fretwire.palette")
local refresh = require("fretwire.refresh")
local buffers = {}
local M = {}

function M.refresh_all()
  for buf in pairs(buffers) do
    refresh(buf)
  end
end

local function schedule(buf, state, first, old_last, new_last)
  if first == nil then
    state.first, state.last = 0, api.nvim_buf_line_count(buf)
  else
    state.first, state.last =
      changed_range(state.first, state.last, first, old_last, new_last)
  end
  if state.pending then
    return
  end
  state.pending = true
  vim.schedule(function()
    state.pending = false
    if buffers[buf] == state then
      local start, finish = state.first, state.last
      state.first, state.last = nil, nil
      refresh(buf, start, finish)
    end
  end)
end

function M.detach(buf)
  buffers[buf] = nil
  if api.nvim_buf_is_valid(buf) then
    marks.clear(buf, 0, -1)
  end
end

function M.attach(buf)
  if buffers[buf] then
    refresh(buf)
    return
  end
  local state = {}
  buffers[buf] = state
  api.nvim_buf_attach(buf, false, {
    on_lines = function(_, _, _, first, old_last, new_last)
      if buffers[buf] ~= state then
        return true
      end
      schedule(buf, state, first, old_last, new_last)
    end,
    on_reload = function()
      schedule(buf, state)
    end,
    on_detach = function()
      if buffers[buf] == state then
        buffers[buf] = nil
      end
    end,
  })
  palette()
  refresh(buf)
end

return M
