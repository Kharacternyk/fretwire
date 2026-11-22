local marks = require("fretwire.marks")

return function(buf, row, line, marker)
  if marker == "" then
    return false
  end
  local marker_start, marker_end = line:find(marker, 1, true)
  if not marker_start then
    return false
  end

  local destination = line:find("%S", marker_end + 1)
  local prefix = line:sub(1, marker_start - 1):gsub("%s+$", "")
  marks.add(
    buf,
    row,
    0,
    #prefix,
    destination and "fretwireMovedLine" or "fretwireDeletedLine"
  )
  marks.add(buf, row, marker_start - 1, marker_end, "fretwireMoveMarker")
  if destination then
    marks.add(buf, row, destination - 1, #line, "fretwireDestination")
  end
  return true
end
