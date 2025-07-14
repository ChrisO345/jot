local M = {}

M.opts = {
  enabled = true,                                                                 -- Whether to enable the plugin
  treesitter = {
    enabled = true,                                                               -- Whether to enable treesitter support
    grammar_path = vim.fn.stdpath("cache") .. "/jot.nvim",                        -- Path to the treesitter grammer. Recommended to use a custom path
    queries_path = vim.fn.stdpath("data") .. "/lazy/nvim-treesitter/queries/jot", -- Path to the treesitter queries. Recommended to use a custom path
  },
  filetype = "jot",                                                               -- This shouldn't ever need to be changed
}

M.setup = function(opts)
  M.opts = vim.tbl_deep_extend("force", M.opts, opts or {})

  if not M.opts.enabled then
    return
  end

  vim.filetype.add({
    pattern = {
      ['.*/jotfile'] = M.opts.filetype,
    },
  })

  if M.opts.treesitter.enabled then
    M.configure_treesitter()
  end

  vim.api.nvim_create_autocmd("FileType", {
    pattern = M.opts.filetype,
    callback = function()
      vim.bo.commentstring = "# %s"
      vim.bo.comments = ":#"
    end,
  })

  local usercmd = vim.api.nvim_create_user_command
  usercmd("JotTSUpdate", M.update_jot, {})
end

M.configure_treesitter = function()
  local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
  ---@diagnostic disable-next-line: inject-field
  parser_config.jot = {
    install_info = {
      url = M.opts.treesitter.grammar_path,
      files = { "src/parser.c" },
      requires_generate_from_grammar = true,
    },
    filetype = M.opts.filetype,
  }
end

M.update_jot = function()
  vim.cmd("TSUpdate jot")

  local queries_path = M.opts.treesitter.queries_path
  local grammar_path = M.opts.treesitter.grammar_path

  local command = string.format(
    "mkdir -p %s && cp -rv %s/queries/* %s/",
    queries_path,
    grammar_path,
    queries_path
  )

  local result = vim.fn.system(command)
  print(result)
end

return M
