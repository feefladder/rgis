#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub struct LibraryEntry {
    pub name: &'static str,
    pub url: &'static str,
    pub crs: u16,
}

pub const ENTRIES: [LibraryEntry; 9] = [LibraryEntry {
    name: "USA States",
    url: "https://raw.githubusercontent.com/PublicaMundi/MappingAPI/master/data/geojson/us-states.json",
    crs: 4326,
}, LibraryEntry {
    name: "Tectonic Plate Boundaries",
    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/tectonic_plates_2002/PB2002_boundaries.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "Countries",
    url: "https://raw.githubusercontent.com/andrea-ballatore/open-geo-data-education/main/datasets/world_country_boundaries_2018/natural_earth_world_boundaries_50m_2018.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "Amtrak Stations",
    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-stations.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "Amtrak Tracks",
    url: "https://raw.githubusercontent.com/datanews/amtrak-geojson/master/amtrak-track.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "World lakes",
    url: "https://raw.githubusercontent.com/simonepri/geo-maps/master/previews/earth-lakes.geo.json",
    crs: 4326,
}, LibraryEntry {
    name: "Russia",
    url: "https://raw.githubusercontent.com/hugoledoux/BIGpolygons/master/russia.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "Washington DC street centerlines",
    url: "https://raw.githubusercontent.com/benbalter/dc-maps/master/maps/street-centerlines.geojson",
    crs: 4326,
}, LibraryEntry {
    name: "NYC Street Centerline (CSCL)",
    url: "https://storage.googleapis.com/rgis-library/NYC%20Street%20Centerline%20(CSCL).geojson",
    crs: 4326,
}];
