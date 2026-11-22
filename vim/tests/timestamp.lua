local equal = require("tests.support").equal
local timestamp = require("fretwire.timestamp")
local function parse(text)
  return timestamp.parse("%Y-%m-%d", text)
end
equal(86400, parse("2024-03-01") - parse("2024-02-29"))
equal(2 * 86400, parse("2024-03-01") - parse("2024-02-28"))
for _, value in ipairs({
  "2023-02-29",
  "2024-04-31",
  "2024-00-10",
  "0000-01-01",
  "2024-13-01",
  "2024-01-00",
  "junk2024-01-01",
  "2024-01-01junk",
  "2024-1-1",
  "",
}) do
  equal(nil, parse(value))
end
equal(parse("2024-01-01"), parse("2024-01-01 \t"))
local function reordered(text)
  return timestamp.parse("%d/%m/%Y", text)
end
equal(86400, reordered("01/01/2025") - reordered("31/12/2024"))
local function timed(text)
  return timestamp.parse("%Y-%m-%dT%H:%M:%S", text)
end
equal(90, timed("2026-01-02T00:00:30") - timed("2026-01-01T23:59:00"))
equal(nil, timed("2026-01-01T24:00:00"))
equal(nil, timed("2026-01-01T23:60:00"))
equal(nil, timed("2026-01-01T23:59:00.125"))
equal(parse("2026-02-01"), timestamp.parse("%d day's %m %Y %%", "01 day's 02 2026 %"))
equal(nil, timestamp.parse("yyyy-MM-dd", "2026-01-01"))
equal(nil, timestamp.parse("%Y-%m-%d %Q", "2026-01-01 Q"))
equal(nil, timed(vim.fn.strftime("%Y-%m-%dT%H:%M:%S", 0)))
