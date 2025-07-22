# Game Creator API
![Rust](https://img.shields.io/badge/Rust-Actix--Web-orange)
![ORM](https://img.shields.io/badge/ORM-SeaORM-green)
![DB](https://img.shields.io/badge/DB-PostgreSQL-blue)
![Docker](https://img.shields.io/badge/Docker-Compose-blue)


Rust Actix Web API สำหรับจัดการข้อมูลผู้สร้างเกมและเกม โดยใช้ SeaORM และ PostgreSQL

## Tech Stack
- 🔧 Rust (Actix Web, SeaORM)
- 🐘 PostgreSQL
- 🐳 Docker

## 🚀 Getting Started

1. Clone repository:
https://github.com/Enberkay/game-creator-api.git

2. สร้าง .env
DATABASE_URL=postgres://myuser:mypassword@localhost:5432/mydatabase

3. Start services
docker-compose up -d

4. Run migrations
cargo install sea-orm-cli
sea-orm-cli migrate up

5. Run project
RUST_LOG=debug cargo run




## โครงสร้างโปรเจค

```
├── src/
│   ├── main.rs              # Entry point
│   ├── database.rs          # Database connection
│   ├── routes.rs            # Route configuration
|   |
|   ├── dtos/
│   │   ├── mod.rs
│   │   ├── creator_dto.rs
│   │   └── game_dto.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── creator.rs       # Creator model
│   │   └── game.rs          # Game model
│   └── controllers/
│       ├── mod.rs
│       ├── creator_controller.rs
│       └── game_controller.rs
├── migrations/
│   ├──src/
|   |   ├── lib.rs
|   |   ├── mxxxxxxxx_xxxxxx_create_table.rs  #example migrate version
|   |   ├── main.rs
|   └── target/|
|
|
├── docker-compose.yml       # PostgreSQL + pgAdmin
├── .env                     # Environment variables
└── README.md
```

## 🔗 API Endpoints

### Creators

- `POST /api/creators` - สร้าง creator ใหม่
- `GET /api/creators` - ดู creator ทั้งหมด
- `GET /api/creators/{id}` - ดู creator ตาม id
- `PUT /api/creators/{id}` - แก้ไข creator
- `DELETE /api/creators/{id}` - ลบ creator
- `GET /api/creators/{id}/games` - ดูเกมทั้งหมดของ creator

| Method | URL                        | รายละเอียด                 |
| ------ | -------------------------- | -------------------------- |
| POST   | `/api/creators`            | สร้าง Creator ใหม่      |
| GET    | `/api/creators`            | ดู Creator ทั้งหมด      |
| GET    | `/api/creators/{id}`       | ดู Creator รายตัว       |
| PUT    | `/api/creators/{id}`       | แก้ไขข้อมูล Creator     |
| DELETE | `/api/creators/{id}`       | ลบ Creator              |
| GET    | `/api/creators/{id}/games` | ดูเกมทั้งหมดของ Creator |

### Games

- `POST /api/games` - สร้างเกมใหม่
- `GET /api/games` - ดูเกมทั้งหมด
- `GET /api/games/{id}` - ดูเกมตาม id
- `PUT /api/games/{id}` - แก้ไขเกม
- `DELETE /api/games/{id}` - ลบเกม
- `GET /api/games/{id}/with-creator` - ดูเกมพร้อมข้อมูล creator

| Method | URL                            | รายละเอียด                  |
| ------ | ------------------------------ | --------------------------- |
| POST   | `/api/games`                   | สร้างเกมใหม่             |
| GET    | `/api/games`                   | ดูเกมทั้งหมด             |
| GET    | `/api/games/{id}`              | ดูเกมรายตัว              |
| PUT    | `/api/games/{id}`              | แก้ไขข้อมูลเกม           |
| DELETE | `/api/games/{id}`              | ลบเกม                    |
| GET    | `/api/games/{id}/with-creator` | ดูเกมพร้อมข้อมูล Creator |

## 📊 Database Schema

### Creators Table

```sql
CREATE TABLE creators (
    id UUID PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Games Table

```sql
CREATE TABLE games (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    genre VARCHAR NOT NULL,
    creator_id UUID NOT NULL REFERENCES creators(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 🌐 Services

- **API Server**: http://localhost:8080
- **pgAdmin**: http://localhost:5050
  - Email: admin@admin.com
  - Password: admin
- **PostgreSQL**: localhost:5432
  - Database: mydatabase
  - User: myuser
  - Password: mypassword

## 📝 ตัวอย่างการใช้งาน

### สร้าง Creator

```bash
1. สร้าง Creator ใหม่
curl -X POST http://localhost:8080/api/creators \
  -H "Content-Type: application/json" \
  -d '{
        "first_name": "Hideo",
        "last_name": "Kojima",
        "email": "hideo.kojima@example.com"
    }'

2. ดู Creator ทั้งหมด
curl -X GET http://localhost:8080/api/creators

3. ดู Creator ตาม ID
curl -X GET http://localhost:8080/api/creators/{creator-id}

4. แก้ไขข้อมูล Creator
curl -X PUT http://localhost:8080/api/creators/{creator-id} \
  -H "Content-Type: application/json" \
  -d '{
        "first_name": "Hideo2",
        "last_name": "Kojima2",
        "email": "kojima.studios@example.com"
    }'

5. ลบ Creator
curl -X DELETE http://localhost:8080/api/creators/{creator-id}

ดูเกมทั้งหมดของ Creator
curl -X GET http://localhost:8080/api/creators/{creator-id}/games
```

### สร้าง Game

```bash
1. สร้างเกมใหม่
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Death Stranding",
    "description": "เกมแนวแอคชั่นผสมจำลองการเดินทางในโลกหลังภัยพิบัติ",
    "genre": "Action",
    "creator_id": "{creator-id}"
  }'
หมายเหตุ: ให้แทน {creator-id} ด้วย UUID ของผู้สร้างที่มีอยู่ในระบบ

2. ดูเกมทั้งหมด
curl -X GET http://localhost:8080/api/games

3. ดูเกมตาม ID
curl -X GET http://localhost:8080/api/games/{game-id}
แทน {game-id} ด้วย UUID ของเกมที่ต้องการดูรายละเอียด

4. แก้ไขข้อมูลเกม
curl -X PUT http://localhost:8080/api/games/{game-id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Death Stranding: Director's Cut",
    "description": "เวอร์ชันพิเศษพร้อมเนื้อหาเพิ่มเติม",
    "genre": "Action-Adventure",
    "creator_id": "{creator-id}"
  }'
สามารถส่งเฉพาะ field ที่ต้องการอัปเดตได้ เช่น อัปเดตแค่ description อย่างเดียวก็ได้

5. ลบเกม
curl -X DELETE http://localhost:8080/api/games/{game-id}

6. ดูเกมพร้อมข้อมูลผู้สร้าง
curl -X GET http://localhost:8080/api/games/{game-id}/with-creator
Endpoint นี้จะคืนข้อมูลเกมพร้อมกับข้อมูลของ creator ที่เกี่ยวข้องในรูปแบบ JSON เดียวกัน
```