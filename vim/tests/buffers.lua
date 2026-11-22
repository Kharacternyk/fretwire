local api = vim.api
local test = require("tests.support")
local equal = test.equal
local plugin = require("fretwire")

test.setup_markers()
local buf = test.new_buffer()
local namespace = api.nvim_get_namespaces().fretwire

-- Buffer settings take precedence and markers are literal, including UTF-8.
vim.b.fretwire_move_marker = "→.*"
vim.b.fretwire_timestamp_marker = " [%] "
vim.b.fretwire_timestamp_pattern = "%d/%m/%Y"
test.set_lines(buf, {
  "старий [%] 31/12/2025",
  "новий [%] 01/01/2026",
  "перенести →.* файл",
  " [%] 01/01/2026",
})
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
equal({ "fretwireAge5", "Comment" }, test.groups(buf, 1))
equal({ "Comment" }, test.groups(buf, 3))
local timestamp_marks = api.nvim_buf_get_extmarks(
  buf,
  namespace,
  { 0, 0 },
  { 0, -1 },
  { details = true }
)
equal(#"старий", timestamp_marks[1][4].end_col)
equal(#"старий", timestamp_marks[2][3])
equal(#"старий [%] 31/12/2025", timestamp_marks[2][4].end_col)
equal(
  { "fretwireMovedLine", "fretwireMoveMarker", "fretwireDestination" },
  test.groups(buf, 2)
)
local marks = api.nvim_buf_get_extmarks(
  buf,
  namespace,
  { 2, 0 },
  { 2, -1 },
  { details = true }
)
equal(#"перенести ", marks[2][3])
equal(#"перенести →.*", marks[2][4].end_col)

vim.b.fretwire_move_marker = ""
vim.b.fretwire_timestamp_marker = ""
vim.cmd("FretwireRefresh")
equal({}, api.nvim_buf_get_extmarks(buf, namespace, 0, -1, {}))
vim.b.fretwire_timestamp_marker = " [%] "
vim.cmd("FretwireRefresh")
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))

-- Independent buffer settings and filetype cleanup.
vim.cmd("enew")
local other = api.nvim_get_current_buf()
vim.bo.filetype = "fretwire"
api.nvim_buf_set_lines(other, 0, -1, false, { "other @ 2026-01-01" })
plugin.refresh(other)
equal(
  "fretwireAge5",
  api.nvim_buf_get_extmarks(other, namespace, 0, -1, { details = true })[1][4].hl_group
)
api.nvim_set_current_buf(buf)
vim.bo.filetype = "text"
equal({}, api.nvim_buf_get_extmarks(buf, namespace, 0, -1, {}))
test.set_lines(buf, { "a [%] 01/01/2000", "b [%] 01/01/2026" })
equal({}, test.groups(buf, 0))
vim.bo.filetype = "fretwire"
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
