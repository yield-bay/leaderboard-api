# leaderboard-api

## Endpoints

### GET /leaderboard

Response:

```json
[
  {
    "address": "<user_wallet_address>",
    "users_brought": 3,
    "created_at": "2022-12-03 11:03:37.923813 UTC",
    "last_user_brought_at": "2022-12-03 11:04:58.673697 UTC"
  }
]
```

### POST /user

Request:

```json
{
  "address": "<user_wallet_address>"
}
```

Response:

```json
{
  "address": "<user_wallet_address>",
  "hash": "2c835e6173bb69cbd0734351566f60eb4d985be57616053fdb1c49fbaa57915c",
  "users_brought": 0,
  "created_at": "2022-12-03 11:03:37.923813 UTC",
  "last_user_brought_at": "",
  "owns_nft": false
}
```

### POST /update

Request:

```json
{
  "hash": "2c835e6173bb69cbd0734351566f60eb4d985be57616053fdb1c49fbaa57915c"
}
```

Response:

```json
{
  "address": "<user_wallet_address>",
  "users_brought": 1,
  "created_at": "2022-12-03 11:03:37.923813 UTC",
  "last_user_brought_at": "2022-12-03 11:04:23.298134 UTC"
}
```
