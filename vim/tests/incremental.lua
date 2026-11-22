local api = vim.api
local test = require("tests.support")
local equal = test.equal
local plugin = require("fretwire")
local timestamp = require("fretwire.timestamp")

test.setup_markers()
local buf = test.new_buffer()
local namespace = api.nvim_get_namespaces().fretwire

-- A local edit must neither read/parse the whole file nor recreate distant marks.
local paragraphs = {}
for _ = 1, 100 do
  vim.list_extend(paragraphs, { "a @ 2026-01-01", "b @ 2026-01-05", "c @ 2026-01-09", "" })
end
test.set_lines(buf, paragraphs)
local distant = api.nvim_buf_get_extmarks(
  buf,
  namespace,
  { 0, 0 },
  { 2, -1 },
  { details = true }
)
local original_parse, original_get_lines = timestamp.parse, api.nvim_buf_get_lines
local parsed, read = 0, 0
timestamp.parse = function(...)
  parsed = parsed + 1
  return original_parse(...)
end
api.nvim_buf_get_lines = function(...)
  local lines = original_get_lines(...)
  read = read + #lines
  return lines
end
api.nvim_buf_set_text(buf, 201, 0, 201, #"b @ 2026-01-05", { "b @ 2026-01-02" })
test.wait_for_refresh()
timestamp.parse, api.nvim_buf_get_lines = original_parse, original_get_lines
equal(3, parsed)
assert(read < 20, "local edit read " .. read .. " lines")
equal(
  distant,
  api.nvim_buf_get_extmarks(buf, namespace, { 0, 0 }, { 2, -1 }, { details = true })
)

-- Compare incremental extmarks with a full refresh, ignoring generated mark IDs.
local function snapshot()
  local result = {}
  for _, item in ipairs(api.nvim_buf_get_extmarks(buf, namespace, 0, -1, { details = true })) do
    local detail = item[4]
    result[#result + 1] =
      { item[2], item[3], detail.end_row, detail.end_col, detail.hl_group }
  end
  table.sort(result, function(a, b)
    return vim.inspect(a) < vim.inspect(b)
  end)
  return result
end
local function matches_full_refresh()
  test.wait_for_refresh()
  local incremental = snapshot()
  plugin.refresh(buf)
  equal(snapshot(), incremental)
end

-- Queued changes before, inside, and after the pending range shift its coordinates.
api.nvim_buf_set_lines(buf, 201, 202, false, { "b @ 2030-01-01" })
api.nvim_buf_set_lines(buf, 0, 0, false, { "inserted @ 1990-01-01", "" })
api.nvim_buf_set_lines(buf, 2, 10, false, {})
api.nvim_buf_set_lines(buf, 300, 301, false, { "", "split @ 2026-01-03", "" })
matches_full_refresh()

-- Repeated mixed edits exercise split/merge boundaries, deletion, and EOF marks.
math.randomseed(42)
local choices = {
  "",
  " \t ",
  "plain",
  "old @ 2000-01-01",
  "new @ 2026-01-01",
  "middle @ 2010-01-01",
  "invalid @ 2026-02-30",
  "move [%] destination",
  "delete [%]",
}
test.set_lines(buf, { "a @ 2000-01-01", "b @ 2026-01-01", "", "single @ 2010-01-01" })
for _ = 1, 100 do
  for _ = 1, 4 do
    local count = api.nvim_buf_line_count(buf)
    local first = math.random(0, count)
    local last = math.min(count, first + math.random(0, 6))
    local replacement = {}
    for _ = 1, math.random(0, 5) do
      replacement[#replacement + 1] = choices[math.random(#choices)]
    end
    api.nvim_buf_set_lines(buf, first, last, false, replacement)
  end
  matches_full_refresh()
end
api.nvim_buf_set_lines(buf, 0, -1, false, {})
matches_full_refresh()
equal({}, snapshot())
