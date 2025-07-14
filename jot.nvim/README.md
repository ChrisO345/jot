# jot.nvim

`jot.nvim` is a basic Neovim plugin that provides basic filetype support for jotfiles, allowing for tree-sitter syntax highlighting.

---

## Installation

As this plugin is bundled with the `jot` repository, it can be installed and hooked into Neovim using a plugin manager.

Using [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
  dir = "path/to/jot.nvim",  -- Path to the cloned jot.nvim directory
  dependencies = {
    "nvim-treesitter/nvim-treesitter" -- If treesitter_path is set, then this is required
  },
  opts = {
    enabled = true,                                                                 -- Whether to enable the plugin
    treesitter = {
      enabled = true,                                                               -- Whether to enable treesitter support
      grammar_path = vim.fn.stdpath("cache") .. "/jot.nvim",                        -- Path to the treesitter grammer. Recommended to use a custom path
      queries_path = vim.fn.stdpath("data") .. "/lazy/nvim-treesitter/queries/jot", -- Path to the treesitter queries. Recommended to use a custom path
    },
    filetype = "jot",                                                               -- This shouldn't ever need to be changed
  }
}
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
