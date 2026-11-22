local marks = require("fretwire.marks")
local timestamp = require("fretwire.timestamp")
local M = {}

function M.collect(entries, row, line, marker, pattern)
  if marker == "" then
    return
  end
  local _, marker_end = line:find(marker, 1, true)
  local time = marker_end and timestamp.parse(pattern, line:sub(marker_end + 1))
  if time then
    entries[#entries + 1] = { row = row, length = #line, time = time }
  end
end

function M.highlight(buf, entries)
  if #entries == 0 then
    return
  end
  local oldest, newest = entries[1].time, entries[1].time
  for _, entry in ipairs(entries) do
    oldest = math.min(oldest, entry.time)
    newest = math.max(newest, entry.time)
  end
  for _, entry in ipairs(entries) do
    local bucket = 5
    if oldest ~= newest then
      local fraction = (entry.time - oldest) / (newest - oldest)
      bucket = math.min(5, math.floor(fraction * 5) + 1)
    end
    marks.add(buf, entry.row, 0, entry.length, "fretwireAge" .. bucket)
  end
end

return M
