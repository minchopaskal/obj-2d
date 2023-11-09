local dap = require("dap")

dap.configurations.rust = {
	{
		name = "Launch file",
		type = "codelldb",
		request = "launch",
		program = function()
			return vim.fn.getcwd() .. "/target/debug/obj-2d"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		args = { "--obj", "res/bugatti.obj" },
	},
}
