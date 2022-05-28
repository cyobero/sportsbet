# Sportsbet API
Sportsbet is an analytics tool that helps bettors find an edge against the sportsbook
by providing an easy, intuitive way to quickly retrieve insights from historical data.

## Resources
=========================================================================================
### Season
Use this API to retrieve a team's season averages and league rank data.
-----------------------------------------------------------------------------------------
endpoint: `seasons/v1`

requests:
    - `GET /{year}`

requests:
    - `GET /`

`GET /{year}/{team}/`
returns:
Season {
    team: BOS,
    year: 2021,
}

### Results
Use this API to retrieve a team's game results for a given season.
-----------------------------------------------------------------------------------------
endpoint: '/results/v1'

Requests:
- `GET /{year}/{team}``



# User Stories
=========================================================================================
- As a user, I want to see the odds for different sports bets so I can decide which event
I should punt on.
- As a user, I want to see a team's season stats so I can 
