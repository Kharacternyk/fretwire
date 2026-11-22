local api = vim.api
local test = require("tests.support")
local equal = test.equal

vim.api.nvim_set_hl(0, "Normal", { fg = "#c8c8c8", bg = "#000000" })
test.setup_markers()
local buf = test.new_buffer()

test.set_lines(buf, {
  "oldest @ 2026-01-01",
  "older @ 2026-01-03",
  "middle @ 2026-01-05",
  "recent @ 2026-01-07",
  "newest @ 2026-01-09",
  "moved @ 1900-01-01 [%] destination",
  "deleted @ 2100-01-01 [%]   ",
  "unstamped",
  "invalid @ 2026-02-30",
  " \t ",
  "one @ 2020-01-01",
  "same @ 2020-01-01",
  "",
  "newer @ 2025-01-01",
  "older @ 2000-01-01",
})
for row = 0, 4 do
  equal({ "fretwireAge" .. row + 1, "Comment" }, test.groups(buf, row))
end
equal(
  { "fretwireMovedLine", "fretwireMoveMarker", "fretwireDestination" },
  test.groups(buf, 5)
)
equal({ "fretwireDeletedLine", "fretwireMoveMarker" }, test.groups(buf, 6))
for _, row in ipairs({ 7, 8, 9, 12 }) do
  equal({}, test.groups(buf, row))
end
for _, row in ipairs({ 10, 11, 13 }) do
  equal({ "fretwireAge5", "Comment" }, test.groups(buf, row))
end
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 14))
equal(0x464646, api.nvim_get_hl(0, { name = "fretwireAge1", link = false }).fg)
equal(0xc8c8c8, api.nvim_get_hl(0, { name = "fretwireAge5", link = false }).fg)

-- Editing an extremum rebuckets the rest of its block automatically.
api.nvim_buf_set_lines(buf, 4, 5, false, { "newest @ 2026-01-21" })
test.wait_for_refresh()
equal({ "fretwireAge2", "Comment" }, test.groups(buf, 3))
-- Removing separators merges ranges; inserting them restores independent ranges.
test.set_lines(buf, { "a @ 2000-01-01", "", "b @ 2026-01-01" })
equal({ "fretwireAge5", "Comment" }, test.groups(buf, 0))
api.nvim_buf_set_lines(buf, 1, 2, false, {})
test.wait_for_refresh()
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
api.nvim_buf_set_lines(buf, 1, 1, false, { " \t " })
test.wait_for_refresh()
equal({ "fretwireAge5", "Comment" }, test.groups(buf, 0))
equal({ "fretwireAge5", "Comment" }, test.groups(buf, 2))

api.nvim_set_hl(0, "Normal", { fg = "#000000", bg = "#c8c8c8" })
api.nvim_exec_autocmds("ColorScheme", {})
equal(0x828282, api.nvim_get_hl(0, { name = "fretwireAge1", link = false }).fg)
equal(0, api.nvim_get_hl(0, { name = "fretwireAge5", link = false }).fg)
