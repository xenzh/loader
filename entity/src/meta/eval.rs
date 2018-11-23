// eval: (conditions, meta) -> plan
//
// GET tracks WHERE \
// artist is a and (year is 2018 or year is 2015) and tag not in (pop, electro) and track.no is 1
//
// I would:
// 1. get Artist:Albums
// 2. for each of them get Album:Info
// 2. filter albums by year and tag from Album:Info
// 3. Return Album:Tracks for all albums
//
// static info: map_in  () like Album+Info -> [AlbumArtist, AlbumName]
//              map_out () like Album+Info -> [AlbumName, AlbumArtist, AlbumTracks, ...]
//
// tracks can be retrieved from album?
// -- yes, via AlbumTracks (album_out). need an album or albums
// okay, need album or albums. Where can I get them from?
// -- from places i have enough parameters for (album_in): -Album:Info, +Artist:Albums, +Chart:Albums, +Tag:Albums...
// what is the most restrictive one? (album_in)? (other options might be reviewed, least
//                                                complex one should be selected)
// -- Artist:Albums (1 param of mine in request, Album:Artist)
// will i be able to filter results with just this response?
// -- no
// okay, can I get limitation params from based on results?
// -- yes, tags and releasedate from Album:Info

use std::borrow::Borrow;
use crate::meta::method::{Target, Method};

pub struct MethodCache {
    dummy: Vec<Box<Method>>,
}

impl MethodCache {
    // methods return list / iter of methods fitting any of criteria, ordered by number of matching
    // criteria

    fn who_returns(&self, _tgts: &[Target]) -> impl Iterator<Item = &Method> {
        self.dummy.iter().map(|b| b.borrow())
    }

    fn who_takes(&self, _tgts: &[Target]) -> impl Iterator<Item = &Method> {
        self.dummy.iter().map(|b| b.borrow())
    }
}

// eval steps:
// 1. get all methods that return tracks
// 2. their input: does all my conditions fit?
//    * yes, one - add to plan
//    * yes, many - ?? add one that returns more info?..
//    * no, we have less then required - which request has required params?
//    * no, we have more than required
//
// GET tracks WHERE (album.year is 2018) and ("chamber pop" in album.tags) and (artist.name is "iamthemorning")
//
// 1. get all methods -> album.getinfo(artist, album), ...
// 2. artist.name - OK
//    album.name - MISS
//      how to get album.name by album.year, album.tags and artist.name?
//        * 
//
//      album.name -> artist.gettopalbums(artist.name) -> [album.name, artist{}, tags[]...]
//        
