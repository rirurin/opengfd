local debug_metaphor = function()
    local att_dap = require('dap')
    local attach_program_path = os.getenv("DEBUG_ATTACH_XRD759")
    local att_dap_config = {
	name = "Attach to Metaphor: Refantazio",
	type = "rt_lldb",
	request = "attach",
	program = attach_program_path
    }
    att_dap.run(att_dap_config)
end
if os.getenv("DEBUG_ATTACH_XRD759") == nil then
    error('\
Environment variable "DEBUG_ATTACH_XRD759" has not been set. \
Please point this to your Metaphor: Refantazio executable (METAPHOR.exe)')
else
    vim.keymap.set('n', '<C-S-F5>', debug_metaphor, {})
end