

#[derive(Clone)]
pub struct PortalCoverage {
    _subsectors: Vec<u32>, //pointer to subsectors
    _subsector_count: i32
}

#[derive(Default)]
pub struct PortalInfo {
    _displacements: DisplacementTable,
    _portal_block_map: PortalBlockMap,
    _linked_portals: Vec<Box<LinePortal>>,
    _portal_groups: Vec<Box<SectorPortalGroup>>,
    _line_portal_spans: Vec<LinePortalSpan>,
}

#[derive(Default)]
struct PortalBlockMap {}

pub struct LinePortal {}

#[derive(Default, Clone, Copy)]
pub struct SectorPortalGroup {}

struct LinePortalSpan {}

#[derive(Default)]
pub struct PortalBits {}

#[derive(Default)]
struct DisplacementTable {}

pub struct SectorPortal {}