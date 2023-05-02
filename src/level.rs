use std::collections::HashMap;

//TODO split this up in multiple structs (level stats, music, lights etc)
//TODO give everything types
//TODO see what is needed etc

struct LevelLocals {
    level,

    process_mask: PortalBits,
    found_portals: Vec<LinePortal>,
    groups_to_check: Vec<i32>,

    //level elements
    elements: LevelElements,

    //lightmaps
    lm: LightMaps,
    lp: LightProbes,

    //portal info
    portal_info: PortalInfo,
    sections: SectionContainer,
    canvas_tex_info: CanvasTextureInfo,
    local_event_manager: EventManager,
    aabb_tree: AABBTree,
    level_mesh: LevelMesh,

    health_groups: HashMap<i32, HealthGroup>,

    block_map: BlockMap,
    poly_block_map: Vec<PolyBlock>,
    udmf_keys: [HashMap<i32, UDMFKeys>;4],


    load_sectors: Vec<Sector>,
    load_lines: Vec<Line>,
    load_sides: Vec<Side>,

    death_match_starts: Vec<PlayerStart>,
    player_starts: [PlayerStart; 8], //8 maxplayers
    all_player_starts: Vec<PlayerStart>,

    behaviors: BehaviorContainer,

    tid_hash: [AActor; 128],

    strife_dialogues: Vec<StrifeDialogueNode>,
    dialogue_roots: HashMap<i32, i32>,
    class_roots: HashMap<String, i32>, //Fname?
    bot_info: CajunMaster,

    ii_compatflags: i32,
	ii_compatflags2: i32,
	ib_compatflags: i32,
	i_compatflags: i32,
	i_compatflags2: i32,

	sector_marker: SectorMarker,

	md5: [u8; 16],			// for savegame validation. If the MD5 does not match the savegame won't be loaded.
	time: i32,			// time in the hub
	maptime: i32,			// time in the map
	totaltime: i32,		// time in the game
	starttime: i32,
	partime: i32,
	sucktime: i32,
	spawnindex: u32,

	info: LevelInfo,
	cluster: i32,
	cluster_flags: i32,
	level_num: i32,
	lump_num: i32,
    level_name: String,
    map_name: String,
    next_map: String,
    next_secret_map: String,
    author_name: String,
    f1_pic: String,
	translator: Translator,
	map_type: MapType,
	tag_manager: TagManager,
    interpolator: Interpolator,

	shader_start_time: u64,

	body_que,
	automap,
	body_que_slot: i32,

	players: [Player;8], //8 max players


    num_map_sections: i32,

    flags: u32,
    flags2: u32,
    flags3: u32,

    fade_to_color: u32,
    outside_fog_color: u32,

    hazard_color: u32,
    hazard_flash_color: u32,

    music: String,
    music_order: i32,
    cd_track: i32,
    cd_id: u32,
    
    sky_texture1: TextureID,
    sky_texture2: TextureID,

    sky_speed1: f32,
    sky_speed2: f32,

    sky_pos1: f64,
    sky_pos2: f64,

    hw_sky_pos1: f32,
    hw_sky_pos2: f32,

    sky_stretch: bool,

    total_secrets: i32,
    found_secrets: i32,

    total_items: i32,
    found_items: i32,

    total_monster: i32,
    killed_monsters: i32,

    map_velocity: f64,
    avg_velocity: f64,

    gravity: f64,
    air_control: f64,
    air_friction: f64,
    air_supply: i32,
    default_environment: i32,

    sequence_list_head: SequenceNode,

    //particles
    oldest_particle: u32,
    active_particles: u32,
    inactive_particles: u32,
    particles: Vec<Particle>,
    particles_in_subsec: Vec<u16>,
    thinkers: ThinkerCollection,

    scrolls: Vec<Vector2<f64>>,

    wall_vert_light: i8,
    wall_hori_light: i8,

    from_snapshot: bool,
    has_height_sectors: bool,
    has_dynamic_lights: bool,
    frozen_state: i32,

    team_damage: f64,

    fog_density: i32,
    outside_fog_density: i32,
    sky_fog: i32,

    death_sequence: String, //fname?
    pixel_stretch: f32,
    music_volume: f32,

    light_mode: LightMode,
    bright_fog: bool,
    light_additive_surfaces: bool,
    no_texture_fill: bool,
    impact_decal_count: i32,

    lights: DynamicLights,

    corpse_queue,
    fraggle_script_thinker,
    acs_thinker,

    spot_state,
}

struct LightMaps {
    surfaces: Vec<LightMapSurface>,
    tex_coords: Vec<f32>,
    tex_count: i32,
    tex_size: i32,
    tex_data: Vec<u16>,
}

struct LightMapSurface {
    type_: SurfaceType,
    subsector: SubSector,
    side: Side,
    control_sector: Sector,
    light_map_num: u32,
    tex_coords: &Vec<f32>
}

enum SurfaceType {
    STNull,
	STMiddleWall,
	STUpperWall,
	STLowerWall,
	STCeiling,
	STFloor
}

struct SubSector {
    sector: Sector,
    polys: PolyNode,
    bsp: MiniBSP,
    first_line: Seg,
    render_sector: Sector,
    section: Section,
    subsector_num: i32,
    line_count: u32,
    flags : u16,
    map_section: i16,
    
    valid_count: i32,
    hacked: u8,

    portal_coverage: Option<[PortalCoverage;2]>,
    light_map: [LightMapSurface;2]

    //fn buildpolybsp
    //fn index

    //TODO implement the functions for subsector

}

struct Line {
    v1: Vertex,
    v2: Vertex,
    delta: Vector2<f64>,

    flags: u32,
    flags2: u32,
    activation: u32,
    special: i32,
    args: [i32;5],
    alpha: f64,
    sidedef: [Side;2],
    bbox: [f64;4],
    front_sector: Sector,
    back_sector: Sector,
    valid_count: i32,
    lock_number: i32,
    portal_index: u32,
    portal_transfered: u32,
    auto_map_style: AutoMapLineStyle,
    health: i32,
    health_group: i32,
    line_num: i32,

    //TODO functions
}

struct Side {
    sector: Sector, //sector sidedef is facing
    attached_decals: BaseDecal,
    textures: [Part;3],
    linedef: Line,
    left_side: u32,
    right_side: u32,
    texel_length: u16,
    light: i16,
    tier_lights: [i16;3],
    flags: u16,
    udmf_index: i32,
    light_head: LightNode,
    lightmap: LightMapSurface,
    segs: Vec<&Seg>, //all segs in ascending order
    num_segs: i32,
    side_num: i32,

    //TODO functions, Part and BaseDecal struct
}

struct Vertex {
    p: Vector2<f64>,

    vertex_num: i32,
    view_angle: Angle,
    angle_time: i32,
    dirty: bool,
    num_heights: i32,
    num_sectors: i32,
    sectors: Vec<&Sector>,
    height_list: &[f32],

    //TODO functions and constructors etc
}

struct Sector {

}

struct Seg {

}

struct Section {

}


struct PolyNode {

}

struct MiniBSP {}



struct PortalCoverage {
    subsectors: Option<&[u32]>, //pointer to subsectors
    subsector_count: i32
}

struct LightProbes {
    light_probes: Vec<LightProbe>,
    min_x: i32,
    min_y: i32,
    width: i32,
    height: i32,
    cell_size: i32, // = 32
    cells: Vec<LightProbeCell>
}

struct LightProbe {
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32
}

struct LightProbeCell {
    first_probe: Option<&[LightProbe]>,
    probe_count: i32
}

struct PortalInfo {
    displacements: DisplacementTable,
    portal_block_map: PortalBlockMap,
    linked_portals: Vec<&LinePortal>,
    portal_groups: Vec<&SectorPortalGroup>,
    line_portal_spans: Vec<LinePortalSpan>,
}

struct DisplacementTable {}
struct PortalBlockMap {}
struct LinePortal {}
struct SectorPortalGroup {}
struct LinePortalSpan {}




struct PlayerStart {}
struct Particle {}

struct PortalBits {}

struct LevelElements {
    vertexes: Vec<Vertex>,
    sectors: Vec<Sector>,
    extsectors: Vec<ExtSector>,
    line_buffer: Vec<Line>,
    subsector_buffer: Vec<SubSector>,
    lines: Vec<Line>,
    sides: Vec<Side>,
    seg_buffer: Vec<Seg>,
    segs: Vec<Seg>,
    subsectors: Vec<SubSector>,
    nodes: Vec<Node>,
    game_subsectors: Vec<SubSector>,
    game_nodes: Vec<Node>,
    head_game_node: Node,
    reject_matrix: Vec<u8>,
    z_zones: Vec<Zone>,
    poly_objects: Vec<PolyObj>,
    sector_portals: Vec<SectorPortal>,
    line_portals: Vec<LinePortal>,
}

struct Vector2<T> {
    x: T,
    y: T
}

//TODO ExtSector, Sector, Seg
