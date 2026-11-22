local api = vim.api
local plugin = require("fretwire")
local M = {}

function M.equal(expected, actual)
  assert(
    vim.deep_equal(expected, actual),
    vim.inspect({ expected = expected, actual = actual })
  )
end

function M.new_buffer()
  vim.cmd("edit " .. vim.fn.fnameescape(vim.fn.tempname() .. ".few"))
  M.equal("fretwire", vim.bo.filetype)
  return api.nvim_get_current_buf()
end

function M.setup_markers()
  plugin.setup({
    timestamp_marker = " @ ",
    move_marker = "[%]",
    timestamp_pattern = "%Y-%m-%d",
  })
end

function M.wait_for_refresh()
  local done = false
  vim.schedule(function()
    done = true
  end)
  assert(vim.wait(1000, function()
    return done
  end))
end

local namespace = api.nvim_get_namespaces().fretwire
function M.groups(buf, row)
  local result = {}
  for _, item in
    ipairs(
      api.nvim_buf_get_extmarks(buf, namespace, { row, 0 }, { row, -1 }, { details = true })
    )
  do
    result[#result + 1] = item[4].hl_group
  end
  return result
end
function M.set_lines(buf, lines)
  api.nvim_buf_set_lines(buf, 0, -1, false, lines)
  M.wait_for_refresh()
end

return M
