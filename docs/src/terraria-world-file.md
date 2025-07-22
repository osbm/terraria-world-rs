

This is the terraria (1.4.4.9) world file standard as i understand it. Please correct me if i am wrong.

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

Tiles have upto 4 flag bytes before them.

- Flag Byte 1
    - 1.0: Has Flag Byte 2
    - 1.1: Has Block
    - 1.2: Has Wall
    - 1.3: Has water
    - 1.4: Has lava (if 1.3 is also true it means the block has honey)
    - 1.5: Has extended block id
    - 1.6: Used For RLE calculation
    - 1.7: Used For RLE calculation
- Flag Byte 2
    - 2.0: Has Flag Byte 3
    - 2.1: Has red wire
    - 2.2: Has blue wire
    - 2.3: Has green wire
    - 2.4: Used for block shape calculation
    - 2.5: Used for block shape calculation
    - 2.6: Used for block shape calculation
    - 2.7: ????
- Flag Byte 3
    - 3.0: Has Flag Byte 4
    - 3.1: Has yellow wire
    - 3.2: Is Block Passive
    - 3.3: Has block paint
    - 3.4: Has wall paint
    - 3.5: Has actuator
    - 3.6: Has extended wall id
    - 3.7: Has shimmer
- Flag Byte 4
    - 4.0: (i believe this is being left empty for possible Flag Byte 5)
    - 4.1: Is block echo
    - 4.2: Is wall echo
    - 4.3: Is block Illuminated
    - 4.4: Is wall Illuminated
    - 4.5:
    - 4.6:
    - 4.7:


For example in the empty world, the first column is only 3 bytes (80 AF 04):

- 80 = binary 1000 0000

Flags are as follows:

| 1.0 | 1.1 | 1.2 | 1.3 | 1.4 | 1.5 | 1.6 | 1.7 |
|-----|-----|-----|-----|-----|-----|-----|-----|
|  0  |  0  |  0  |  0  |  0  |  0  |  0  |  1  |

Which means That there is no block no wall no 2nd other flag bytes are present and we need to read 2 bytes to learn how many times we need to repeat this tile. Which is (AF 04 = 1199) 1200 if you count the current tile.


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

### 12. Footer

Just has world name and id to indicate the end of the file.
