local test = require("tests.support")
local equal = test.equal
local plugin = require("fretwire")

-- Unconfigured behavior matches the Rust defaults, then accepts environment settings.
local buf = test.new_buffer()
test.set_lines(
  buf,
  { "a @ 2000-01-01", "b @ 2026-01-01", "move :> destination", "delete :>" }
)
equal({}, test.groups(buf, 0))
equal(
  { "fretwireMovedLine", "fretwireMoveMarker", "fretwireDestination" },
  test.groups(buf, 2)
)
equal({ "fretwireDeletedLine", "fretwireMoveMarker" }, test.groups(buf, 3))
vim.env.FRETWIRE_TIMESTAMP_MARKER = " @ "
-- The Rust ICU pattern must not override the plugin's strftime default.
vim.env.FRETWIRE_TIMESTAMP_PATTERN = "dd/MM/yyyy"
vim.cmd("FretwireRefresh")
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))

-- Reconfiguration refreshes existing buffers, preserves overrides, and resets omissions.
plugin.setup({ timestamp_marker = "" })
equal({}, test.groups(buf, 0))
plugin.setup({ timestamp_marker = " @ ", timestamp_pattern = "%d/%m/%Y" })
test.set_lines(buf, { "a @ 01/01/2000", "b @ 01/01/2026" })
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
vim.b.fretwire_timestamp_pattern = "%d/%m/%Y"
plugin.setup({ timestamp_marker = " @ " })
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
vim.b.fretwire_timestamp_pattern = nil
plugin.setup({ timestamp_marker = " @ " })
equal({}, test.groups(buf, 0))
test.set_lines(buf, { "a @ 2000-01-01", "b @ 2026-01-01" })
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
vim.env.FRETWIRE_TIMESTAMP_MARKER = " # "
test.set_lines(buf, { "a # 2000-01-01", "b # 2026-01-01" })
equal({}, test.groups(buf, 0))
plugin.setup()
equal({ "fretwireAge1", "Comment" }, test.groups(buf, 0))
