# Description

This is a small, rough around the edges, application that allows you to easily use Sonny's LiDAR models with Ortho4XP.

It achieves this by mapping out the expected 10x10 grid directories within Ortho4XP's 'Elevation_data' directory. The reason for this approach is convenience so that you don't have to manully specify the per-tile configuration to point to each HGT file.

# Usage

For now, this is a command line app. I may eventually turn this into a UI app to promote a better user-experience.

To use this, you must first have downloaded the files via [Sonny's LiDAR sources](https://sonny.4lima.de/). Afterwards, extract all HGT files into a single directory.

The application will prompt you for a full/absolute path to the LiDAR sources as well as the Ortho4XP Elevation_data directory. These should be fully qualified paths.
Example:

The following directory holds all HGT files at the root path.

Windows:
```
C:\Users\Me\Downloads\LiDAR\Extracted
```

Unix:
```
~/Downloads/LiDAR/Extracted
```


The following directory contains the 10x10 elevation grid subdirectories.

Windows
```
C:\Users\Me\Documents\Ortho4XP\_internal\Ortho4XP_Data\Elevation_data
```

Unix:
```
~/Documents/Ortho4XP/_internal/Ortho4XP_Data/Elevation_data
```

The process this app takes to map the data to the relevant directories is as follows:
1. Enumerate all HGT files to discover which are available
2. Map the files to the 10x10 grid format
3. Create the 10x10 grid subdirectories in the LiDAR path
4. Move all files into the 10x10 grid subdirectories
5. Symlink the subdirectories over to Ortho4XP's 'Elevation_data' path.