# Implemented Schema Model Structs
## Common
- [x] Position
- [x] WarId

## raw
### Assignments
- [x] Assignment
- [x] Reward
- [x] Setting
- [x] Task

### Campaign
- [x] Campaign

### Dispatch
- [x] NewsFeedItem

### Planet
- [x] HomeWorld
- [x] PlanetAttack
- [x] PlanetEvent
- [x] PlanetInfo
- [x] PlanetStatus
- [x] PlanetCoordinates

### Stats
- [x] GalaxyStats
- [x] PlanetStats

### War
- [x] WarInfo
- [x] WarStatus
- [x] WarSummary
- [x] JointOperation

## v1
### Assignments
- [x] Assignment2 / MajorOrder
- [x] Reward2 / MajorOrderReward
- [x] Task2 / Major Order Task

### Campaign
- [ ] Campaign2 (more information needed)

### Dispatch
- [x] Dispatch
- [x] LocalizedMessage
- [x] SteamNews

### Planet
- [x] Biome
- [x] Event
- [x] Hazard
- [x] Planet
- [x] Position

### Stats
- [x] Statistics

### War
- [x] War


# Implemented routes
## raw
- [x] `/raw/api/WarSeason/current/WarID`
- [x] `/raw/api/WarSeason/{war_id}/Status`
- [x] `/raw/api/WarSeason/{war_id}/WarInfo`
- [x] `/raw/api/Stats/war/{war_id}/801/summary`
- [x] `/raw/api/NewsFeed/{war_id}`
- [x] `/raw/api/v2/Assignment/War/{war_id}`

## v1
- [x] `/api/v1/war`
- [ ] `/api/v1/assignments`
- [ ] `/api/v1/assignments/{index}`
- [ ] `/api/v1/campaigns`
- [ ] `/api/v1/campaigns/{index}`
- [ ] `/api/v1/dispatches`
- [ ] `/api/v1/dispatches/{index}`
- [ ] `/api/v1/planets`
- [ ] `/api/v1/planets/{index}`
- [ ] `/api/v1/planet-events`
- [ ] `/api/v1/steam`
- [ ] `/api/v1/steam/{gid}`

