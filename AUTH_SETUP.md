# Authentication & Authorization Setup

## Environment Variables

สร้างไฟล์ `.env` ในโฟลเดอร์หลักและเพิ่ม:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/game_creator_db
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
PORT=8080
```

## การใช้งานระบบ Auth

### 1. Register User
```bash
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123",
  "role": "creator"
}
```

### 2. Login
```bash
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

### 3. Get Current User (Protected)
```bash
GET /api/auth/me
Authorization: Bearer <your-jwt-token>
```

## Middleware Functions

### Function 1: JWT Decode Middleware
- ตรวจสอบ token จาก Authorization header
- Decode JWT และแนบข้อมูลไปกับ `req.user`
- ถ้าไม่มี token จะ reject ทันที

### Function 2: Role Check Middleware
- ตรวจสอบ role ของ user ว่าตรงกับที่กำหนดไว้หรือไม่
- ใช้กับ endpoints ที่ต้องการ role เฉพาะ

### Function 3: Current User Function
- ใช้ `req.user` เพื่อ query ข้อมูล user จาก database
- ตรวจสอบ role อีกครั้งจาก database
- เหมาะสำหรับ frontend ProtectRoute

## Protected Routes

### Admin Routes (role: "admin")
- `/api/creators/*` - จัดการ creators

### Creator Routes (role: "creator")
- `/api/games/*` - จัดการ games

### Public Routes
- `/api/auth/register` - ลงทะเบียน
- `/api/auth/login` - เข้าสู่ระบบ

## Frontend Integration

สำหรับ frontend สามารถใช้ `/api/auth/me` เพื่อ:
1. ตรวจสอบว่า user login อยู่หรือไม่
2. ดึงข้อมูล user ปัจจุบัน
3. ตรวจสอบ role เพื่อ ProtectRoute 