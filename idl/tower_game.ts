export type TowerGame = {
  "version": "0.1.0",
  "name": "tower_game",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "serverAddress",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "participateTournament",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "server",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tournamentId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tap",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "rewardAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "buyChance",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReward",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tournamentId",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "userTournamentState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "health",
            "type": "u64"
          },
          {
            "name": "rewardsEarned",
            "type": "u64"
          },
          {
            "name": "sessions",
            "type": "u64"
          },
          {
            "name": "taps",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "gameState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isInit",
            "type": "bool"
          },
          {
            "name": "serverAddress",
            "type": "publicKey"
          },
          {
            "name": "tournamentId",
            "type": "u64"
          },
          {
            "name": "tournamentStartAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tournamentState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topPlayer",
            "type": "publicKey"
          },
          {
            "name": "topScore",
            "type": "u64"
          },
          {
            "name": "isRewarded",
            "type": "bool"
          },
          {
            "name": "prizePool",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "GameAlreadyInit",
      "msg": "Game already init!"
    },
    {
      "code": 6001,
      "name": "TournamentNotOngoing",
      "msg": "Given tournament is not currently on-going"
    },
    {
      "code": 6002,
      "name": "TournamentFinished",
      "msg": "Tournament already finished"
    },
    {
      "code": 6003,
      "name": "NotEnoughChances",
      "msg": "Not enough chances left to play this tournament buy it to continue"
    },
    {
      "code": 6004,
      "name": "RewardClaimed",
      "msg": "Reward already claimed!!"
    },
    {
      "code": 6005,
      "name": "NotWinner",
      "msg": "You are not a winner!"
    }
  ]
};

export const IDL: TowerGame = {
  "version": "0.1.0",
  "name": "tower_game",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "serverAddress",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "participateTournament",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "server",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tournamentId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tap",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "rewardAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "buyChance",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReward",
      "accounts": [
        {
          "name": "gameState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tournamentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tournamentId",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "userTournamentState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "health",
            "type": "u64"
          },
          {
            "name": "rewardsEarned",
            "type": "u64"
          },
          {
            "name": "sessions",
            "type": "u64"
          },
          {
            "name": "taps",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "gameState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isInit",
            "type": "bool"
          },
          {
            "name": "serverAddress",
            "type": "publicKey"
          },
          {
            "name": "tournamentId",
            "type": "u64"
          },
          {
            "name": "tournamentStartAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tournamentState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topPlayer",
            "type": "publicKey"
          },
          {
            "name": "topScore",
            "type": "u64"
          },
          {
            "name": "isRewarded",
            "type": "bool"
          },
          {
            "name": "prizePool",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "GameAlreadyInit",
      "msg": "Game already init!"
    },
    {
      "code": 6001,
      "name": "TournamentNotOngoing",
      "msg": "Given tournament is not currently on-going"
    },
    {
      "code": 6002,
      "name": "TournamentFinished",
      "msg": "Tournament already finished"
    },
    {
      "code": 6003,
      "name": "NotEnoughChances",
      "msg": "Not enough chances left to play this tournament buy it to continue"
    },
    {
      "code": 6004,
      "name": "RewardClaimed",
      "msg": "Reward already claimed!!"
    },
    {
      "code": 6005,
      "name": "NotWinner",
      "msg": "You are not a winner!"
    }
  ]
};
