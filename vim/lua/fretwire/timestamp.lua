local M = {}

function M.parse(pattern, text)
  text = text:gsub("%s+$", "")
  local ok, time = pcall(vim.fn.strptime, pattern, text)
  -- strptime uses zero for failure, which also excludes the Unix epoch itself.
  if not ok or time == 0 then
    return nil
  end
  -- Reject normalized invalid dates, partial matches, and noncanonical input.
  local formatted, result = pcall(vim.fn.strftime, pattern, time)
  if formatted and result == text then
    return time
  end
end

return M
