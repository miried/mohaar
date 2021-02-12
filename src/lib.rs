//extern crate libc;
use libc::{c_int, intptr_t};

mod ui;

/// When loading the library, the engine will first call dllEntry
/// So that we know the syscallptr to call functions from the library.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn dllEntry( syscallptr : intptr_t ) {
	ui::syscalls::set_syscallptr(syscallptr)
}

/// This is the gateway function for the engine to trigger events in the library.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn vmMain( command: c_int, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int, arg10: c_int, arg11: c_int) -> intptr_t {
	let cmd : uiExport_t = unsafe { std::mem::transmute(command) };

	let result = match cmd {
		uiExport_t::UI_GETAPIVERSION => ui::UI_APIVERSION,
		uiExport_t::UI_INIT => ui::init(arg0 != 0),
		uiExport_t::UI_SHUTDOWN => ui::shutdown(),
		uiExport_t::UI_KEY_EVENT => ui::key_event(arg0, arg1 != 0),
		uiExport_t::UI_MOUSE_EVENT => ui::mouse_event(arg0, arg1),
		uiExport_t::UI_REFRESH => ui::refresh(arg0),
		uiExport_t::UI_IS_FULLSCREEN => ui::is_fullscreen(),
		uiExport_t::UI_SET_ACTIVE_MENU => {
			let menu : ui::uiMenuCommand_t = unsafe { std::mem::transmute(arg0) };
			ui::set_active_menu(menu)
		},
		uiExport_t::UI_CONSOLE_COMMAND => ui::console_command(arg0),
		uiExport_t::UI_DRAW_CONNECT_SCREEN => ui::draw_connect_screen(arg0 != 0),
		uiExport_t::UI_HASUNIQUECDKEY => ui::has_unique_cdkey(),
	};
	result as intptr_t
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum uiExport_t {
	UI_GETAPIVERSION = 0,	// system reserved

	UI_INIT,
//	void	UI_Init( void );

	UI_SHUTDOWN,
//	void	UI_Shutdown( void );

	UI_KEY_EVENT,
//	void	UI_KeyEvent( int key );

	UI_MOUSE_EVENT,
//	void	UI_MouseEvent( int dx, int dy );

	UI_REFRESH,
//	void	UI_Refresh( int time );

	UI_IS_FULLSCREEN,
//	qboolean UI_IsFullscreen( void );

	UI_SET_ACTIVE_MENU,
//	void	UI_SetActiveMenu( uiMenuCommand_t menu );

	UI_CONSOLE_COMMAND,
//	qboolean UI_ConsoleCommand( int realTime );

	UI_DRAW_CONNECT_SCREEN,
//	void	UI_DrawConnectScreen( qboolean overlay );
	UI_HASUNIQUECDKEY
// if !overlay, the background will be drawn, otherwise it will be
// overlayed over whatever the cgame has drawn.
// a GetClientState syscall will be made to get the current strings
}
