# Details

Date : 2024-04-27 14:20:14

Directory c:\\Users\\crazy\\Desktop\\Programming\\Rust\\discord-rs\\src

Total : 66 files,  3571 codes, 585 comments, 537 blanks, all 4693 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [src/builders/client/mod.rs](/src/builders/client/mod.rs) | Rust | 5 | 0 | 1 | 6 |
| [src/builders/embed/mod.rs](/src/builders/embed/mod.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/builders/mod.rs](/src/builders/mod.rs) | Rust | 8 | 0 | 1 | 9 |
| [src/builders/slash_command/mod.rs](/src/builders/slash_command/mod.rs) | Rust | 297 | 58 | 60 | 415 |
| [src/lib.rs](/src/lib.rs) | Rust | 33 | 0 | 2 | 35 |
| [src/managers/cache/mod.rs](/src/managers/cache/mod.rs) | Rust | 67 | 1 | 18 | 86 |
| [src/managers/channel/mod.rs](/src/managers/channel/mod.rs) | Rust | 44 | 6 | 12 | 62 |
| [src/managers/channel/types.rs](/src/managers/channel/types.rs) | Rust | 18 | 0 | 4 | 22 |
| [src/managers/client/mod.rs](/src/managers/client/mod.rs) | Rust | 24 | 5 | 5 | 34 |
| [src/managers/guild/mod.rs](/src/managers/guild/mod.rs) | Rust | 43 | 1 | 11 | 55 |
| [src/managers/guild/types.rs](/src/managers/guild/types.rs) | Rust | 18 | 1 | 4 | 23 |
| [src/managers/mod.rs](/src/managers/mod.rs) | Rust | 11 | 0 | 1 | 12 |
| [src/structs/application/mod.rs](/src/structs/application/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/application/types.rs](/src/structs/application/types.rs) | Rust | 50 | 1 | 7 | 58 |
| [src/structs/application_command/mod.rs](/src/structs/application_command/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/application_command/types.rs](/src/structs/application_command/types.rs) | Rust | 87 | 0 | 8 | 95 |
| [src/structs/attachment/mod.rs](/src/structs/attachment/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/attachment/types.rs](/src/structs/attachment/types.rs) | Rust | 16 | 0 | 1 | 17 |
| [src/structs/channel/deserializers.rs](/src/structs/channel/deserializers.rs) | Rust | 34 | 0 | 5 | 39 |
| [src/structs/channel/enums.rs](/src/structs/channel/enums.rs) | Rust | 21 | 0 | 2 | 23 |
| [src/structs/channel/impls.rs](/src/structs/channel/impls.rs) | Rust | 47 | 3 | 14 | 64 |
| [src/structs/channel/mod.rs](/src/structs/channel/mod.rs) | Rust | 7 | 1 | 3 | 11 |
| [src/structs/channel/types.rs](/src/structs/channel/types.rs) | Rust | 58 | 35 | 8 | 101 |
| [src/structs/client/enums.rs](/src/structs/client/enums.rs) | Rust | 112 | 12 | 6 | 130 |
| [src/structs/client/mod.rs](/src/structs/client/mod.rs) | Rust | 185 | 59 | 44 | 288 |
| [src/structs/client/structs.rs](/src/structs/client/structs.rs) | Rust | 134 | 3 | 9 | 146 |
| [src/structs/embed/mod.rs](/src/structs/embed/mod.rs) | Rust | 228 | 156 | 39 | 423 |
| [src/structs/embed/types.rs](/src/structs/embed/types.rs) | Rust | 72 | 0 | 9 | 81 |
| [src/structs/emoji/mod.rs](/src/structs/emoji/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/emoji/types.rs](/src/structs/emoji/types.rs) | Rust | 14 | 0 | 2 | 16 |
| [src/structs/guild/deserializers.rs](/src/structs/guild/deserializers.rs) | Rust | 209 | 16 | 16 | 241 |
| [src/structs/guild/enums.rs](/src/structs/guild/enums.rs) | Rust | 96 | 0 | 11 | 107 |
| [src/structs/guild/mod.rs](/src/structs/guild/mod.rs) | Rust | 8 | 16 | 6 | 30 |
| [src/structs/guild/types.rs](/src/structs/guild/types.rs) | Rust | 160 | 2 | 8 | 170 |
| [src/structs/locale/mod.rs](/src/structs/locale/mod.rs) | Rust | 133 | 0 | 6 | 139 |
| [src/structs/member/enums.rs](/src/structs/member/enums.rs) | Rust | 6 | 0 | 0 | 6 |
| [src/structs/member/mod.rs](/src/structs/member/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/structs/member/types.rs](/src/structs/member/types.rs) | Rust | 17 | 3 | 2 | 22 |
| [src/structs/message/enums.rs](/src/structs/message/enums.rs) | Rust | 42 | 0 | 2 | 44 |
| [src/structs/message/mod.rs](/src/structs/message/mod.rs) | Rust | 35 | 43 | 15 | 93 |
| [src/structs/message/types.rs](/src/structs/message/types.rs) | Rust | 101 | 46 | 14 | 161 |
| [src/structs/message_payload/mod.rs](/src/structs/message_payload/mod.rs) | Rust | 29 | 0 | 6 | 35 |
| [src/structs/message_payload/types.rs](/src/structs/message_payload/types.rs) | Rust | 38 | 9 | 5 | 52 |
| [src/structs/nonce/mod.rs](/src/structs/nonce/mod.rs) | Rust | 26 | 0 | 3 | 29 |
| [src/structs/permissions/enums.rs](/src/structs/permissions/enums.rs) | Rust | 49 | 1 | 1 | 51 |
| [src/structs/permissions/mod.rs](/src/structs/permissions/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/presence/mod.rs](/src/structs/presence/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/presence/types.rs](/src/structs/presence/types.rs) | Rust | 124 | 2 | 14 | 140 |
| [src/structs/reaction/mod.rs](/src/structs/reaction/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/reaction/types.rs](/src/structs/reaction/types.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/structs/role/mod.rs](/src/structs/role/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/role/types.rs](/src/structs/role/types.rs) | Rust | 24 | 0 | 2 | 26 |
| [src/structs/snowflake/mod.rs](/src/structs/snowflake/mod.rs) | Rust | 96 | 8 | 17 | 121 |
| [src/structs/sticker/mod.rs](/src/structs/sticker/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/structs/sticker/types.rs](/src/structs/sticker/types.rs) | Rust | 30 | 1 | 4 | 35 |
| [src/structs/timestamp/mod.rs](/src/structs/timestamp/mod.rs) | Rust | 72 | 6 | 8 | 86 |
| [src/structs/user/mod.rs](/src/structs/user/mod.rs) | Rust | 16 | 0 | 4 | 20 |
| [src/structs/user/types.rs](/src/structs/user/types.rs) | Rust | 107 | 0 | 8 | 115 |
| [src/structs/webhook/errors.rs](/src/structs/webhook/errors.rs) | Rust | 34 | 2 | 7 | 43 |
| [src/structs/webhook/mod.rs](/src/structs/webhook/mod.rs) | Rust | 177 | 64 | 40 | 281 |
| [src/structs/webhook/types.rs](/src/structs/webhook/types.rs) | Rust | 14 | 0 | 2 | 16 |
| [src/test.rs](/src/test.rs) | Rust | 90 | 14 | 17 | 121 |
| [src/util/env/mod.rs](/src/util/env/mod.rs) | Rust | 19 | 0 | 5 | 24 |
| [src/util/rest/mod.rs](/src/util/rest/mod.rs) | Rust | 51 | 7 | 13 | 71 |
| [src/util/socket/mod.rs](/src/util/socket/mod.rs) | Rust | 33 | 3 | 8 | 44 |
| [src/util/threadpool/mod.rs](/src/util/threadpool/mod.rs) | Rust | 72 | 0 | 13 | 85 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)