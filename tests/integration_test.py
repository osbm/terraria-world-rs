#!/usr/bin/env python3
"""
Integration test script for terraria-world-parser-rust

This script uses the lihzahrd Python library to parse Terraria world files
and generate reference data for comparison with the Rust implementation.
"""

import lihzahrd
import json
import sys
import os
import argparse
from pathlib import Path
from typing import Dict, Any, List, Optional


def parse_world_file(world_file: str) -> Dict[str, Any]:
    """
    Parse a Terraria world file using lihzahrd and return structured data.
    
    Args:
        world_file: Path to the .wld file
        
    Returns:
        Dictionary containing parsed world data
    """
    try:
        world = lihzahrd.World.create_from_file(world_file)
        
        # Extract metadata
        metadata = {
            "version": str(world.version),
            "name": world.name,
            "size": {"width": world.size.x, "height": world.size.y},
            "difficulty": str(world.difficulty),
            "is_hardmode": world.is_hardmode,
            "is_drunk_world": world.is_drunk_world,
            "is_for_the_worthy": world.is_for_the_worthy,
            "is_tenth_anniversary": world.is_tenth_anniversary,
            "is_the_constant": world.is_the_constant,
            "is_bee_world": world.is_bee_world,
            "is_upside_down": world.is_upside_down,
            "is_trap_world": world.is_trap_world,
            "is_zenith_world": world.is_zenith_world,
            "spawn_point": {"x": world.spawn.x, "y": world.spawn.y},
            "dungeon_point": {"x": world.dungeon.x, "y": world.dungeon.y},
            "underground_level": world.underground_level,
            "cavern_level": world.cavern_level,
            "tile_frame_important": world.tile_frame_important,
        }
        
        # Extract sample tiles for comparison
        sample_tiles = []
        if hasattr(world.tiles, 'tiles') and world.tiles.tiles:
            # Sample tiles from different areas of the world
            sample_positions = [
                (0, 0),  # Top-left
                (world.size.x // 4, world.size.y // 4),  # Quarter way
                (world.size.x // 2, world.size.y // 2),  # Center
                (world.size.x // 2, world.size.y // 4),  # Middle-top
                (world.size.x // 4, world.size.y // 2),  # Middle-left
                (world.size.x - 1, world.size.y - 1),  # Bottom-right
                (world.spawn.x, world.spawn.y),  # Spawn point
                (world.dungeon.x, world.dungeon.y),  # Dungeon point
            ]
            
            for x, y in sample_positions:
                if (x < len(world.tiles.tiles) and 
                    y < len(world.tiles.tiles[x]) if world.tiles.tiles[x] else False):
                    tile = world.tiles.tiles[x][y]
                    tile_data = extract_tile_data(tile, x, y)
                    sample_tiles.append(tile_data)
            
            # Also sample some random tiles for better coverage
            import random
            random.seed(42)  # For reproducible results
            for _ in range(10):
                x = random.randint(0, min(len(world.tiles.tiles) - 1, 100))
                if world.tiles.tiles[x]:
                    y = random.randint(0, min(len(world.tiles.tiles[x]) - 1, 100))
                    tile = world.tiles.tiles[x][y]
                    tile_data = extract_tile_data(tile, x, y)
                    sample_tiles.append(tile_data)
        
        return {
            "metadata": metadata,
            "tiles": {
                "sample_tiles": sample_tiles,
                "total_tiles": sum(len(col) for col in world.tiles.tiles) if hasattr(world.tiles, 'tiles') else 0
            }
        }
        
    except Exception as e:
        raise RuntimeError(f"Failed to parse world file {world_file}: {e}")


def extract_tile_data(tile, x: int, y: int) -> Dict[str, Any]:
    """
    Extract detailed tile data for comparison.
    
    Args:
        tile: lihzahrd Tile object
        x: X coordinate
        y: Y coordinate
        
    Returns:
        Dictionary containing tile data
    """
    tile_data = {
        "position": {"x": x, "y": y},
        "has_block": tile.block is not None,
        "has_wall": tile.wall is not None,
        "has_liquid": tile.liquid is not None,
        "wiring": {
            "red": tile.wiring.red,
            "blue": tile.wiring.blue,
            "green": tile.wiring.green,
            "yellow": tile.wiring.yellow,
        }
    }
    
    # Extract block data
    if tile.block:
        block_data = {
            "type_id": tile.block.type_.value,
            "is_active": tile.block.is_active,
            "has_paint": tile.block.paint is not None,
            "is_illuminant": tile.block.is_illuminant,
            "is_echo": tile.block.is_echo,
        }
        
        if tile.block.paint is not None:
            block_data["paint_id"] = tile.block.paint.value
        
        if tile.block.frame is not None:
            block_data["frame"] = {
                "x": tile.block.frame.x,
                "y": tile.block.frame.y
            }
        
        tile_data["block"] = block_data
    
    # Extract wall data
    if tile.wall:
        wall_data = {
            "type_id": tile.wall.type_.value,
            "has_paint": tile.wall.paint is not None,
            "is_illuminant": tile.wall.is_illuminant,
            "is_echo": tile.wall.is_echo,
        }
        
        if tile.wall.paint is not None:
            wall_data["paint_id"] = tile.wall.paint.value
        
        tile_data["wall"] = wall_data
    
    # Extract liquid data
    if tile.liquid:
        liquid_data = {
            "type_id": tile.liquid.type_.value,
            "volume": tile.liquid.volume,
        }
        tile_data["liquid"] = liquid_data
    
    return tile_data


def find_world_files(directory: str = ".") -> List[str]:
    """
    Find all .wld files in the given directory.
    
    Args:
        directory: Directory to search in
        
    Returns:
        List of .wld file paths
    """
    world_files = []
    for file in Path(directory).glob("*.wld"):
        world_files.append(str(file))
    return sorted(world_files)


def main():
    parser = argparse.ArgumentParser(
        description="Generate reference data for terraria-world-parser-rust integration tests"
    )
    parser.add_argument(
        "world_file", 
        nargs="?", 
        help="Path to .wld file to parse (if not provided, searches for .wld files in current directory)"
    )
    parser.add_argument(
        "-o", "--output", 
        help="Output JSON file path (default: {world_file}.lihzahrd_reference.json)"
    )
    parser.add_argument(
        "--all", 
        action="store_true", 
        help="Process all .wld files in current directory"
    )
    parser.add_argument(
        "--verbose", "-v", 
        action="store_true", 
        help="Verbose output"
    )
    
    args = parser.parse_args()
    
    if args.verbose:
        print(f"lihzahrd version: {lihzahrd.__version__}")
    
    world_files = []
    
    if args.all:
        world_files = find_world_files()
        if not world_files:
            print("No .wld files found in current directory")
            sys.exit(1)
    elif args.world_file:
        if not os.path.exists(args.world_file):
            print(f"World file not found: {args.world_file}")
            sys.exit(1)
        world_files = [args.world_file]
    else:
        # Default behavior: look for common test world names
        common_names = [
            "small_corruption.wld",
            "small_crimson.wld",
            "medium_corruption.wld", 
            "medium_crimson.wld",
            "large_corruption.wld",
            "large_crimson.wld",
        ]
        
        for name in common_names:
            if os.path.exists(name):
                world_files.append(name)
                break
        
        if not world_files:
            print("No world files found. Please specify a .wld file or use --all to process all .wld files.")
            sys.exit(1)
    
    for world_file in world_files:
        print(f"Processing world file: {world_file}")
        
        try:
            # Parse the world file
            world_data = parse_world_file(world_file)
            
            # Determine output file
            if args.output:
                output_file = args.output
            else:
                output_file = f"{world_file}.lihzahrd_reference.json"
            
            # Write reference data
            with open(output_file, 'w') as f:
                json.dump(world_data, f, indent=2)
            
            print(f"Reference data written to: {output_file}")
            print(f"World: {world_data['metadata']['name']}")
            print(f"Size: {world_data['metadata']['size']['width']}x{world_data['metadata']['size']['height']}")
            print(f"Sample tiles: {len(world_data['tiles']['sample_tiles'])}")
            print(f"Total tiles: {world_data['tiles']['total_tiles']}")
            
        except Exception as e:
            print(f"Error processing {world_file}: {e}")
            if args.verbose:
                import traceback
                traceback.print_exc()
            sys.exit(1)


if __name__ == "__main__":
    main() 