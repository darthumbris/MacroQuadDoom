pub struct Game {
    //TODO
    pub game_info: GameInfo,
}

impl Game {
    //TODO
    pub fn new() -> Game {
        Game { game_info: GameInfo::new() }
    }
}

pub struct GameInfo {
    //TODO
    pub game_type: GameType
}

impl GameInfo {
    //TODO
    pub fn new() -> GameInfo {
        GameInfo { game_type: GameType::Doom }
    }
}

pub enum GameType {
    Any = 0,
    Doom = 1,
    Heretic = 2,
    Hexen = 4,
    Strife = 8,
    Chex = 16,

    Raven = 6,
    DoomChex = 17,
    DoomStrifeChex = 25
}

//TODO initgame where it fills the gameinfo, parsed the wad etc
//TODO some of the things only need to begin when the game is at the menu screen (loading level etc)


/*
 * D_DoomMain_Internal {
 * 
 *  D_InitGame();
 *  D_DoomLoop();
 * }
 * 
 * D_DoomLoop() {
 *    G_Ticker();
 * }
 * 
 * G_Ticker() {
 *    G_DoLoadGame();
 * }
 * 
 * G_DoLoadGame() {
 *    G_InitNew();
 * }
 * 
 * G_InitNew() {
 *    G_DoLoadLevel();
 * }
 * 
 * G_DoLoadLevel() {
 *    primarylevel.DoLoadLevel();
 * }
 * 
 * LevelLocals::DoLoadLevel() {
 *    P_SetupLevel();
 * }
 * 
 * P_SetupLevel() {
 *    MapLoader loader(level);
 *    loader.LoadLevel();
 * }
 * 
 * MapLoader::LoadLevel() {
 *      LoadBehavior();
 *      T_LoadScripts();
 *      Level->Behaviors.LoadDefaultModules();
 *      LoadMapinfoACSLump();
 *      LoadStrifeConversations();
 * 
 *      if (!textmap) {
 *          LoadVertexes();
 *          LoadLineDefs();
 *          LoadSideDefs2();
 *          FinishLoadingLineDefs();
 *          LoadThings();
 *      }
 *      else {
 *          ParseTextMap();
 *      }
 * 
 *      CalcIndices();
 *      PostProcessLevel();
 * 
 *      LoopSidedefs();
 * 
 *      if (something)  {
 *           LoadExtendedNodes();
 *           if (!NodesLoaded) {
 *                LoadGLNodes();
 *           }
 *      }
 * 
 * 
 *      LoadBlockMap();
 *      LoadReject();
 *      GroupLines();
 *      FloodZones();
 *      SetRenderSector();
 *      FixMiniSegReferences();
 *      FixHoles();
 *      CalcIndices();
 * 
 *      CreateSections();
 * 
 *      SpawnSlopeMakers();
 * 
 *      Spawn3DFloors();
 * 
 *      SpawnThings();
 * 
 *      if (someasd) {
 *           LoadLightMap();
 *      }
 * 
 *      SpawnSpecials();
 * }
 * 
 */