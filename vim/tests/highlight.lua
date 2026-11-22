vim.opt.runtimepath:prepend(vim.fn.getcwd() .. "/vim")
vim.cmd("filetype plugin on")
vim.cmd("runtime ftdetect/fretwire.vim")
vim.o.termguicolors = true
package.path = "vim/?.lua;" .. package.path

for _, scenario in ipairs({
  "timestamp",
  "shading",
  "incremental",
  "buffers",
  "config",
  "reload",
}) do
  for _, name in ipairs({ "move_marker", "timestamp_marker", "timestamp_pattern" }) do
    vim.env["FRETWIRE_" .. name:upper()] = nil
  end
  require("fretwire").setup()
  dofile("vim/tests/" .. scenario .. ".lua")
end

print("Fretwire highlighting tests passed")
