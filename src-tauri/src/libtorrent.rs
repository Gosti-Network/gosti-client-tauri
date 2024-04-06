
#[cxx::bridge(namespace = "libtorrent")]

pub mod ffi {
    unsafe extern "C++" {
	include!("src/lt.h");

	type session;
	type add_torrent_params;
	type torrent_handle;

	/// This function return a struct of type lt::session
	pub fn lt_create_session() -> UniquePtr<session>;

	/// This function return a struct of type lt::add_torrent_params
	///
	/// lt::add_torrent_params is return by lt::parse_magnet_uri,
	/// then it configured with the given magnet string, and the current path
	pub fn lt_parse_magnet_uri(uri: &str, path: &str) -> UniquePtr<add_torrent_params>;

	/// This function return a struct of type lt::torrent_handle
	///
	/// Call the function add_torrent() using the current session and add_torrent_params
	/// given in parameters
	pub fn lt_session_add_torrent(ses: Pin<&mut session>, params: Pin<&mut add_torrent_params>) -> UniquePtr<torrent_handle>;

	/// This function remove the given torrent from session
	pub fn lt_session_remove_torrent(ses: Pin<&mut session>, hdl: &torrent_handle);

	/// This function call pause() for the given session
	pub fn lt_session_pause(ses: Pin<&mut session>);

	/// This function return true if torrent has metadata
	pub fn lt_torrent_has_metadata(hdl: &torrent_handle) -> bool;

	/// This function return the torrent's name
	///
	/// Name is found in torrent_info return by function torrent_file()
	pub fn lt_torrent_get_name(hdl: &torrent_handle) -> &str;

	/// This function return bencoded data by lt::bencode()
	pub fn lt_torrent_bencode(hdl: &torrent_handle) -> &[u8];

	/// This function call libtorrent::version() and return libtorrent version
	pub fn version() -> *const c_char;
    }
}
