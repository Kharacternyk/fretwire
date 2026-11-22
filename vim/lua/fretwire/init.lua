local api = vim.api
local buffers = require("fretwire.buffers")
local config = require("fretwire.config")
local palette = require("fretwire.palette")
local refresh = require("fretwire.refresh")

local function setup(opts)
  config.replace(opts)
  buffers.refresh_all()
end

local group = api.nvim_create_augroup("fretwire", { clear = true })
api.nvim_create_autocmd("ColorScheme", { group = group, callback = palette })
api.nvim_create_autocmd("OptionSet", {
  group = group,
  pattern = "background",
  callback = palette,
})
api.nvim_create_user_command("FretwireRefresh", function()
  palette()
  refresh()
end, { desc = "Refresh Fretwire markers and timestamp shading" })

return {
  setup = setup,
  attach = buffers.attach,
  detach = buffers.detach,
  refresh = refresh,
}
