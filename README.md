# Blog Backend

This project is the backend API for a personal blog, developed using Nest.js, MongoDB, and Docker.  
It supports operations such as authentication, authorization, and CRUD operations for comments.

## Features

- Authentication
- Comment CRUD

## Prerequisites

- Nest.js
- Docker
- MongoDB

### Choosing version of MongoDB

MongoDB 5.0+ requires a CPU with AVX support.  

```bash
cat /proc/cpuinfo | grep -i avx
```

Check your cpu with above command, and add 4.x version in `MONGO_TAG` in `.env` file.
