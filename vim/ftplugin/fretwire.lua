if vim.b.did_fretwire_ftplugin then
  return
end
vim.b.did_fretwire_ftplugin = true
require("fretwire").attach(vim.api.nvim_get_current_buf())
vim.b.undo_ftplugin = table.concat({
  vim.b.undo_ftplugin or "",
  "call v:lua.require'fretwire'.detach(bufnr())",
  "unlet! b:did_fretwire_ftplugin",
}, "\n")
