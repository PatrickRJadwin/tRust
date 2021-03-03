mod bencode;
mod contruct_url;

extern crate crypto;

use bencode::{BencodeTorrent, get_bencode_from_file};
use contruct_url::{get_url};

fn main() {
    let bencode_torrent: BencodeTorrent = get_bencode_from_file();
    let bencode_torrent_url: String = get_url(bencode_torrent);
    println!("{}", bencode_torrent_url);
}
