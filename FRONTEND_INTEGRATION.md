# Frontend Integration Guide

## React ProtectRoute Example

```tsx
import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

interface User {
  id: string;
  email: string;
  role: string;
}

interface ProtectRouteProps {
  children: React.ReactNode;
  requiredRole?: string;
}

const ProtectRoute: React.FC<ProtectRouteProps> = ({ children, requiredRole }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const token = localStorage.getItem('token');
        
        if (!token) {
          navigate('/login');
          return;
        }

        // Function 3: Get current user from API
        const response = await fetch('/api/auth/me', {
          headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        });

        if (!response.ok) {
          localStorage.removeItem('token');
          navigate('/login');
          return;
        }

        const userData: User = await response.json();
        
        // Check role if required
        if (requiredRole && userData.role !== requiredRole) {
          navigate('/unauthorized');
          return;
        }

        setUser(userData);
      } catch (error) {
        console.error('Auth check failed:', error);
        navigate('/login');
      } finally {
        setLoading(false);
      }
    };

    checkAuth();
  }, [navigate, requiredRole]);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (!user) {
    return null;
  }

  return <>{children}</>;
};

export default ProtectRoute;
```

## Usage Examples

### Admin Only Page
```tsx
<ProtectRoute requiredRole="admin">
  <AdminDashboard />
</ProtectRoute>
```

### Creator Only Page
```tsx
<ProtectRoute requiredRole="creator">
  <CreatorDashboard />
</ProtectRoute>
```

### Any Authenticated User
```tsx
<ProtectRoute>
  <UserProfile />
</ProtectRoute>
```

## Login Component Example

```tsx
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

const Login: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        const data = await response.json();
        localStorage.setItem('token', data.token);
        localStorage.setItem('user', JSON.stringify(data.user));
        
        // Redirect based on role
        if (data.user.role === 'admin') {
          navigate('/admin');
        } else if (data.user.role === 'creator') {
          navigate('/creator');
        } else {
          navigate('/dashboard');
        }
      } else {
        alert('Login failed');
      }
    } catch (error) {
      console.error('Login error:', error);
      alert('Login failed');
    }
  };

  return (
    <form onSubmit={handleLogin}>
      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <input
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button type="submit">Login</button>
    </form>
  );
};

export default Login;
```

## API Calls with Auth

```tsx
// Helper function for authenticated API calls
const apiCall = async (url: string, options: RequestInit = {}) => {
  const token = localStorage.getItem('token');
  
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
      ...options.headers,
    },
  });

  if (response.status === 401) {
    localStorage.removeItem('token');
    window.location.href = '/login';
    return;
  }

  return response;
};

// Usage
const createGame = async (gameData: any) => {
  const response = await apiCall('/api/games', {
    method: 'POST',
    body: JSON.stringify(gameData),
  });
  
  return response.json();
};
```

## Middleware Functions Summary

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