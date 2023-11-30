# Endpoints

## Regular API
```
GET     /api/asos
GET     /api/asos/{id}
PUT     /api/asos/{id}

GET     /api/asos/{id}/membershipRequests
POST    /api/asos/{id}/membershipRequests
POST    /api/asos/{id}/membershipRequests/{id}

GET     /api/asos/{id}/publicDocuments
GET     /api/asos/{id}/documents
POST    /api/asos/{id}/documents
PUT     /api/asos/{id}/documents/{id}

GET     /api/asos/{id}/materials
GET     /api/asos/{id}/lendableMaterials
POST    /api/asos/{id}/materials
POST    /api/asos/{id}/lendableMaterials
PUT     /api/asos/{id}/materials/{id}
DELETE  /api/asos/{id}/materials/{id}

GET     /api/managers

GET     /api/publicActivities
GET     /api/memberActivities
GET     /api/boardActivities

POST    /api/activities
GET     /api/activities/{id}
PUT     /api/activities/{id}

POST    /api/activities/{id}/media
DELETE  /api/activities/{id}/media/{id}

GET     /api/activities/{id}/registration
POST    /api/activities/{id}/registration
```

## Auth API

- `/auth/signup`
- `/auth/login`
- `/auth/logout`
- `/auth/change_username`
- `/auth/change_password`



