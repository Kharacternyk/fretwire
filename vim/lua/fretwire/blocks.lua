local api = vim.api
local M = {}

function M.is_separator(line)
  return line:match("^%s*$") ~= nil
end

function M.range(buf, first, last)
  local count = api.nvim_buf_line_count(buf)
  -- Neighbors cover both sides of inserted/deleted paragraph separators.
  first, last = math.max(0, first - 1), math.min(count, last + 1)
  local function blank(row)
    return M.is_separator(api.nvim_buf_get_lines(buf, row, row + 1, false)[1])
  end
  while first > 0 and not blank(first) do
    first = first - 1
  end
  while last < count and not blank(last - 1) do
    last = last + 1
  end
  return first, last == count and -1 or last
end

return M
