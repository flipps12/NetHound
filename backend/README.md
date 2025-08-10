# NetHound - Backend

Este es el backend de NetHound, desarrollado en Node.js con Express.  
Se encarga de manejar la API y la comunicación con la base de datos.

---

## 📦 Instalación

##### La instalacion por los scripts es automatica, esto no es necesario.

```bash
cd backend
npm install
````

---

## ▶️ Ejecución

```bash
npm run dev   # modo desarrollo
npm start     # modo producción
```

---

## 🌐 Rutas disponibles

redirects.ts
* `GET / & GET /generate_204` → Redirección automatica a login.]

users.ts
* `GET /api/reload_firewall` → Recarga el firewall. No protegida
* `POST /api/adduser` → Crear usuarios. No protegida
* `GET /api/getallusers` → Ver usuarios. No protegida 
* `GET /api/user/:user` → Ver un usuario. No protegida
* `GET /api/userip` → Ver ip de usuario - usar query con la IP y MAC. No protegida
* `DELETE /api/delete/:id` → Eliminar usuarios. No protegida
* `POST /api/login` → Iniciar sesión.
* `GET /api/verify` → Verificar autorización de IP y MAC - usar query. No protegida
* `GET /api/onlyip` → Verificar solo por IP. No protegida

PacketAnalizer.ts
* `GET /api/daily-traffic/:date` → Muestra el trafico de un dia especificado. No protegida
* `GET /api/resolve` → Busca en el DNS el hostname usando la IP. No protegida

---
