# Terraria World Parser

This is a simple terraria world parser library for the rust ecosystem.

## World File Format Documentation

Terraria world files (.wld) are structured into 11 distinct sections, each containing specific data about the world. The file begins with a header containing metadata and a pointer table that indicates where each section begins.

### 1. File Header

| Variable Name | Bytes | Data Type | Possible Values | Explanation |
|---------------|-------|-----------|-----------------|-------------|
| `version_integer` | 4 | i32 | 279 (1.4.4.9), 278 (1.4.4.8), etc. | Terraria version number |
| `magic` | 7 | String |"relogic" | Magic string identifier |
| `savefile_type` | 1 | u8 | 0 | File type identifier |
| `revision` | 4 | u32 | 0 | Revision number |
| `is_favorite` | 8 | u64 | 0 or 1 | Whether world is marked as favorite (Why so big?) |
| `pointer_count` | 2 | u16 | 11 | Number of section pointers. This seems to be constant on all 1.4.4.9 worlds |
| `pointer_vector` | `pointer_count` * 4 | Vec\<u32> | Depends on the worlds contents | Vector of section pointers, a map for this file |
| `tile_frame_important_count` | 2 | i16 | ?? | Number of bits for the tile_frame_important vector |
| `tile_frame_important` | ceil(`tile_frame_important_count`/8) | Vec\<bool> | ?? | ?? |


### 2. World Header

The world metadata. Defeated bosses, biome styles,

| Variable Name | Bytes | Data Type | Possible Values | Explanation |
|---------------|-------|-----------|-----------------|-------------|
| `world_name` | length of string | String | * | Name of the world |

This is a bit too long i got lazy.

### 3. Tile Data

All the information related to blocks, walls, liquids etc.

### 4. Chest Data

All the information related to chests, their contents, and locations.

### 5. Sign Data

All signs with their text and locations.

### 6. NPC Data

All the information related to NPCs, their locations, and states.

### 7. Tile Entitiy Data

A TileEntity, such as a Training Dummy, an Item Frame or a Logic Sensor.

### 8. Pressure Plate Data

Information about Weighed Pressure Plates locations.

### 9. Town manager data

Information about the NPC rooms

### 10. Beastiary Data

Information about the Beastiary, such as which creatures have been encountered/killed.

### 11. Journey Powers Data

Journey mode powers.