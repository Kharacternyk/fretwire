local marks = require("fretwire.marks")
local timestamp = require("fretwire.timestamp")
local M = {}

function M.collect(entries, row, line, marker, pattern)
  if marker == "" then
    return
  end
  local marker_start, marker_end = line:find(marker, 1, true)
  local time = marker_end and timestamp.parse(pattern, line:sub(marker_end + 1))
  if time then
    entries[#entries + 1] =
      { row = row, marker_col = marker_start - 1, length = #line, time = time }
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
    marks.add(buf, entry.row, 0, entry.marker_col, "fretwireAge" .. bucket)
    marks.add(buf, entry.row, entry.marker_col, entry.length, "Comment")
  end
end

return M
