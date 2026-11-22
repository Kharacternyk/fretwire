local api = vim.api
local test = require("tests.support")
local equal = test.equal

require("fretwire").setup({ timestamp_marker = " @ " })

-- Disk reloads and unloading/reopening a buffer restore highlighting.
local path = vim.fn.tempname() .. ".few"
vim.fn.writefile({ "a @ 2000-01-01", "b @ 2026-01-01" }, path)
vim.cmd("edit " .. vim.fn.fnameescape(path))
local buf = api.nvim_get_current_buf()
equal({ "fretwireAge1" }, test.groups(buf, 0))
vim.fn.writefile({ "a @ 2026-01-01", "b @ 2026-01-01" }, path)
vim.cmd("edit!")
test.wait_for_refresh()
equal({ "fretwireAge5" }, test.groups(buf, 0))
vim.cmd("bunload")
vim.cmd("buffer " .. buf)
test.wait_for_refresh()
equal({ "fretwireAge5" }, test.groups(buf, 0))

-- Undo restores the range, not just the text.
vim.cmd("let &undolevels = &undolevels")
api.nvim_buf_set_lines(buf, 0, 1, false, { "a @ 2000-01-01" })
test.wait_for_refresh()
equal({ "fretwireAge1" }, test.groups(buf, 0))
vim.cmd("undo")
test.wait_for_refresh()
equal({ "fretwireAge5" }, test.groups(buf, 0))
vim.fn.delete(path)
