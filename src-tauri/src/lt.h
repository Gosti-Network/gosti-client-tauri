
#pragma once

#include "rust/cxx.h"

#include <libtorrent/version.hpp>
#include <libtorrent/create_torrent.hpp>
#include <libtorrent/session.hpp>
#include <libtorrent/magnet_uri.hpp>

#include <memory>

char const* version();

namespace libtorrent {

std::unique_ptr<lt::session> lt_create_session();
std::unique_ptr<lt::add_torrent_params> lt_parse_magnet_uri(rust::Str uri, rust::Str path);
std::unique_ptr<lt::torrent_handle> lt_session_add_torrent(lt::session &ses, lt::add_torrent_params &params);
void lt_session_remove_torrent(lt::session &ses, const lt::torrent_handle &hdl);
void lt_session_pause(lt::session &ses);
bool lt_torrent_has_metadata(const lt::torrent_handle &hdl);
rust::Str lt_torrent_get_name(const lt::torrent_handle &hdl);
rust::Slice<const uint8_t> lt_torrent_bencode(const lt::torrent_handle &hdl);

}