# Testing Examples

## 1. Register User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "password123",
    "role": "admin"
  }'
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "admin@example.com",
    "role": "admin"
  }
}
```

## 2. Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "password123"
  }'
```

## 3. Get Current User (Protected)

```bash
curl -X GET http://localhost:8080/api/auth/me \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

## 4. Create Game (Creator Role Required)

```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -d '{
    "name": "My Game",
    "description": "A great game",
    "genre": "Action",
    "creator_id": "123e4567-e89b-12d3-a456-426614174000"
  }'
```

## 5. Create Creator (Admin Role Required)

```bash
curl -X POST http://localhost:8080/api/creators \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -d '{
    "name": "John Doe",
    "email": "john@example.com",
    "bio": "Game developer"
  }'
```

## Error Examples

### 1. No Token Provided
```bash
curl -X GET http://localhost:8080/api/auth/me
```
Response: `401 Unauthorized - No token provided`

### 2. Invalid Token
```bash
curl -X GET http://localhost:8080/api/auth/me \
  -H "Authorization: Bearer invalid-token"
```
Response: `401 Unauthorized - Invalid token`

### 3. Insufficient Permissions
```bash
# Try to access admin route with creator role
curl -X POST http://localhost:8080/api/creators \
  -H "Authorization: Bearer creator-token" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test"}'
```
Response: `401 Unauthorized - Insufficient permissions`

## Middleware Flow

### Function 1: JWT Decode Middleware
1. ตรวจสอบ Authorization header
2. ถ้าไม่มี token → reject ทันที
3. ถ้ามี token → decode JWT
4. แนบข้อมูล user ไปกับ `req.user`

### Function 2: Role Check Middleware
1. รับข้อมูล user จาก `req.user`
2. ตรวจสอบ role ว่าตรงกับที่กำหนดไว้หรือไม่
3. ถ้าไม่ตรง → reject
4. ถ้าตรง → ผ่านไปยัง endpoint

### Function 3: Current User Function
1. ใช้ `req.user` เพื่อ query ข้อมูลจาก database
2. ตรวจสอบ role อีกครั้งจาก database
3. ส่งข้อมูล user กลับไปให้ frontend
4. Frontend ใช้ข้อมูลนี้เพื่อ ProtectRoute 