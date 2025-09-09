# Understanding and Using Hurricane Data

_Information to accompany my presentation at the Online News Association's ONA25 conference in New Orleans on September 10, 2025._

## There's a new hurricane. Now what?

Expect lots of interest, before it ever reaches land.

- Where is it going?
- When might it get here?
- How strong will it become?
- Where it is now?
- What hazards are ahead?

Also, maybe:

- Where it's been?
- What other storms have hit the area?

## Current Tracking Data

- National Hurricane Center [main page](https://www.nhc.noaa.gov/)
- The [CurrentStorms](https://www.nhc.noaa.gov/CurrentStorms.json) json file.
- Example NHC data in this repository's [examples/kiko](examples/kiko) folder.
  - `wsp34knt120hr_5km.shp`: the 5-day chances of 34-knot (tropical storm) winds
  - `wsp50knt120hr_5km.shp`: the 5-day chances of 50-knot ("damaging") winds
  - `wsp50knt120hr_5km.shp`: the 5-day chances of 64-knot (hurricane-force) winds
  - `5day_pts.shp`: the 5-day prediction of location and strength
  - `5day_lin.shp`: 5-day "path" (really just connecting the points)
  - Items in the "best track" folder are detail of where the storm _already has been_
- Models `https://web.uwm.edu/hurricane-models/models/{stormId}.kml` where `stormId` is a Hurricane Center ID in lower case like `ep112025`.

## Using this data to make maps

I made the this map using Mapshaper.org and Datawrapper. Details are at the bottom of this page.

![Hurricane Kiko](images/GU1hb-hurricane-kiko-expected-to-pass-north-of-hawaii.png)

### New Data Products

#### Warnings by county

During an active hurricane threatening the U.S., the hurricane center provides maps of watches and warnings by county, not just the shorelines.

#### Rip currents

Main [website](https://www.nhc.noaa.gov/templates/graphics_ripCurrents_inc.shtml) and latest map.

- Latest [data](https://www.nhc.noaa.gov/gis/rip_currents/rip_currents_composite_latest.geojson) (geojson)
- [One-day](https://www.nhc.noaa.gov/gis/rip_currents/rip_currents_day1_latest.geojson) forecast
- [Two-day](https://www.nhc.noaa.gov/gis/rip_currents/rip_currents_day2_latest.geojson) forecast

Pause to show how to very quickly make a current rip-currents map with Datawrapper.

## Previous Storms

### Hurricane tracks archive

Caution: This site gets _very slow_ during big hurricanes, especially close to landfall. Get your data here early.

- Historical hurricane tracks [site](https://coast.noaa.gov/hurricanes/#map=4/32/-80)
- Here, I've selected major hurricanes (Category 3, 4 and 5) that [made landfall near New Orleans](https://coast.noaa.gov/hurricanes/#map=7.24/29.864/-89.931&search=eyJzZWFyY2hTdHJpbmciOiJOZXcgT3JsZWFucywgT3JsZWFucyBQYXJpc2gsIExvdWlzaWFuYSwgVVNBIiwic2VhcmNoVHlwZSI6Imdlb2NvZGVkIiwib3NtSUQiOiIxMzE4ODUiLCJjYXRlZ29yaWVzIjpbIkg1IiwiSDQiLCJIMyJdLCJ5ZWFycyI6W10sIm1vbnRocyI6W10sImVuc28iOltdLCJwcmVzc3VyZSI6eyJyYW5nZSI6WzAsMTAzMF0sImluY2x1ZGVVbmtub3duUHJlc3N1cmUiOnRydWV9LCJ0YWJJbmRleCI6MSwiYnVmZmVyIjo2MCwiYnVmZmVyVW5pdCI6WyJOYXV0aWNhbCBNaWxlcyJdLCJzb3J0U2VsZWN0aW9uIjp7InZhbHVlIjoieWVhcnNfbmV3ZXN0IiwibGFiZWwiOiJZZWFyIChOZXdlc3QpIn0sImFwcGx5VG9BT0kiOnRydWUsImlzU3Rvcm1MYWJlbHNWaXNpYmxlIjp0cnVlfQ==)

Important note! This system will only show hurricanes that were one of those categories _while within the circle of interest_. For example, Katrina was a Category 5 hurricane earlier in its existence in the Gulf, but was a Category 3 at landfall so it doesn't appear in this:

- [Category 5 hurricanes](https://coast.noaa.gov/hurricanes/#map=7.24/29.864/-89.931&search=eyJzZWFyY2hTdHJpbmciOiJOZXcgT3JsZWFucywgT3JsZWFucyBQYXJpc2gsIExvdWlzaWFuYSwgVVNBIiwic2VhcmNoVHlwZSI6Imdlb2NvZGVkIiwib3NtSUQiOiIxMzE4ODUiLCJjYXRlZ29yaWVzIjpbIkg1Il0sInllYXJzIjpbXSwibW9udGhzIjpbXSwiZW5zbyI6W10sInByZXNzdXJlIjp7InJhbmdlIjpbMCwxMDMwXSwiaW5jbHVkZVVua25vd25QcmVzc3VyZSI6dHJ1ZX0sInRhYkluZGV4IjoxLCJidWZmZXIiOjYwLCJidWZmZXJVbml0IjpbIk5hdXRpY2FsIE1pbGVzIl0sInNvcnRTZWxlY3Rpb24iOnsidmFsdWUiOiJ5ZWFyc19uZXdlc3QiLCJsYWJlbCI6IlllYXIgKE5ld2VzdCkifSwiYXBwbHlUb0FPSSI6dHJ1ZSwiaXNTdG9ybUxhYmVsc1Zpc2libGUiOnRydWV9) that made landfall near New Orleans (really: hurricanes that were Category 5 near New Orleans)

### NHC data archive

- NHC's [geospatial data archive](https://www.nhc.noaa.gov/gis/)

### Model archive

- [Model tracks](https://web.uwm.edu/hurricane-models/models/archive/) from the University of Wisconsin-Milwaukee

## Tropycal Python Software

- [General documentation](https://tropycal.github.io/tropycal/index.html)
- [Example documentation](https://tropycal.github.io/tropycal/samples/tracks.storm.html)
- [Model plot documentation](https://tropycal.github.io/tropycal/api/generated/tropycal.realtime.RealtimeStorm.plot_ensembles.html)
- Collab notebook tk tk

## NYT Examples

- Hurricane [Erin 2025](https://www.nytimes.com/interactive/2025/weather/hurricane-erin-map-path-tracker.html)
- Hurriane [Helene 2024](https://www.nytimes.com/interactive/2024/09/24/weather/helene-map-path-tracker.html)

## Tips & Best Practices

### Basics

- _tropical disturbances_ — clusters of storms
- _tropical depressions_ — more organized; sustained winds of 38 miles per hour; National Hurricane Center begins issuing advisories
- _tropical storms_ — sustained winds of 39 m.p.h. or more; it gets a name
- _hurricanes_ — sustained winds at 74 m.p.h. or more
- _major hurricanes_ — sustained winds at least 111 m.p.h., making it a Category 3 or higher

### Landfall

When the _center_ of the storm — which is the center of the eye — reaches land. (Not when the eyewall reaches land.)

### Long-range track predictions

Hurricane experts repeatedly tell us that, at this point, any notion of where a hurricane will be more than 7 days out is not valid.

## Making the Datawrapper map

I used the files located in the [examples/kiko](examples/kiko) folder of this repository.

### Building the damaging winds file

Note that these files start as "shapefiles," which are actually a collection of files. To make sure Mapshaper gets all the data it needs, be sure to select all of the shape's files (they all have the same name, with different extentions), and drag them together into Mapshaper:

![selecting all of the files](images/select_files.png)

- Opened [mapshaper.org](https://mapshaper.org)
- Dragged all of the `_wsp50knt120hr_5km` files from the example folder Mapshaper to the main window
- Added `pct` value to each band (using lowest value available)
  - You do this by inspecting the area and then using the "edit" button in the little window
- Opened console with the Console button
- Typed `-classify pct colors=Oranges breaks=20,40,60,80`
- Typed `-style stroke-width=0`
- Used the Export button
- Saved as Geojson

### Building the forecast points file

- Reload [mapshaper.org](https://mapshaper.org)
- Dragged all of the `_5day_pts` files to the main window
- Want to [style the markers](https://academy.datawrapper.de/article/177-how-to-style-your-markers-before-importing-them-to-datawrapper) for Datawrapper ahead of time
- Opened console with the Console button
- Typed `-each "name=DATELBL"`
- Typed `-each "markerSymbol=SSNUM"`
- Used the Export button
- Saved as Geojson

### In Datawrapper

- Signed in
- New locator map
- Toggle "Import line and area markers" switch
- Dragged in the damaging winds geojson
- Dragged in the forecast points geojson
- Tinkered with the styles, including:
  - Made the areas 100% opacity
  - Changed the circles for the tropical storm points to little dots
  - Added carrage returns to the time/day labels
  - Added "Hurricane Kiko" as the first label
  - Added "Hawaii" as a label by searching for it in the box
  - Tinkered with the Hawaii text and formatting
- In "Annotate & Layout":
  - Toggled "Show key"
  - Added a title
  - Changed position to "Top"
  - Deleted the point icons
  - Re-labeled the area colors
