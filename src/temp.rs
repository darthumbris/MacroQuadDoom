/*
function that setups the level
it will first call open_map to make the mapdata
then call load_level to create all the data needed for the game
 */
fn setup_level() {}


/*Will open the map (either using an external file or it will use the default file)
it needs to check for the type of map (doom/hexen/udmf)
create all the differnt lumps (Header,txtmap for udmf or header,things,linedefs..blockmap ->doom (if ends with behavior -> hexen))
*/
fn open_map() {}




/*
This function uses the mapdata and then parses that all according to the type of map
it needs to create all the leveldata

maybe have a struct for the maploader ?
 */
fn load_level() {}

/*
Also need to parse all the textures, music etc
 */