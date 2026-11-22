local M = {}
local defaults =
  { move_marker = ":>", timestamp_marker = "", timestamp_pattern = "%Y-%m-%d" }
local config = {}

function M.option(buf, name)
  local value = vim.b[buf]["fretwire_" .. name]
  if value == nil then
    value = config[name]
  end
  -- The formatter's timestamp pattern uses ICU notation, not strftime.
  if value == nil and name ~= "timestamp_pattern" then
    value = vim.env["FRETWIRE_" .. name:upper()]
  end
  return value == nil and defaults[name] or value
end

function M.replace(opts)
  opts = opts or {}
  assert(type(opts) == "table", "fretwire.setup expects a table")
  local updated = {}
  for name, value in pairs(opts) do
    assert(
      defaults[name] ~= nil and type(value) == "string",
      "fretwire.setup: unknown option or non-string value: " .. tostring(name)
    )
    updated[name] = value
  end
  config = updated
end

return M
