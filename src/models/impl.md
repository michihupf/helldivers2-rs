# Implemented Schema Model Structs
## Common
- [x] Position
- [x] WarId

## raw
### War
- [x] WarInfo
- [x] WarStatus
- [x] WarSummary

### Assignments
- [x] Assignment
- [x] Campaign
- [x] JointOperation
- [x] Reward
- [x] Setting
- [x] Task

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

### Dispatch
- [x] NewsFeedItem

## v1
### War
- [x] War

### Assignments
- [x] Assignment2 / MajorOrder
- [ ] Campaign2 (more information needed)
- [x] Reward2 / MajorOrderReward
- [x] Task2 / Major Order Task

### Planet
- [x] Biome
- [x] Event
- [x] Hazard
- [x] Planet
- [x] Position

### Dispatch
- [x] Dispatch
- [x] LocalizedMessage
- [x] SteamNews

### Stats
- [x] Statistics

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

