local api = vim.api
local namespace = api.nvim_create_namespace("fretwire")
local M = {}

function M.clear(buf, first, last)
  api.nvim_buf_clear_namespace(buf, namespace, first, last)
end

function M.add(buf, row, first, last, group)
  if last > first then
    api.nvim_buf_set_extmark(buf, namespace, row, first, {
      end_col = last,
      hl_group = group,
      priority = 100,
    })
  end
end

return M
