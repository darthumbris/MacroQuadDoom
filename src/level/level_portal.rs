

#[derive(Clone)]
pub struct PortalCoverage {
    subsectors: Vec<u32>, //pointer to subsectors
    subsector_count: i32
}

#[derive(Default)]
pub struct PortalInfo {
    displacements: DisplacementTable,
    portal_block_map: PortalBlockMap,
    linked_portals: Vec<Box<LinePortal>>,
    portal_groups: Vec<Box<SectorPortalGroup>>,
    line_portal_spans: Vec<LinePortalSpan>,
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