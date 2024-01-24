# Endpoints

## Regular API
- (x) GET     /api/managers

- (x) GET     /api/asociations
- (x) GET     /api/asociations/{id}
- (x) GET     /api/asociations/byName/{name}
- (x) PUT     /api/asociations/{id}
- (x) POST    /api/asociations

- (x) GET     /api/asociations/{id}/membershipRequests
- (-) POST    /api/asociations/{id}/membershipRequests
- (-) PATCH   /api/asociations/{id}/membershipRequests/{id}
- (-) DELETE  /api/asociations/{id}/membershipRequests/{id}

- (-) GET     /api/asociations/{id}/members
- (-) PUT     /api/asociations/{id}/members/{id}
- (-) DELETE  /api/asociations/{id}/members/{id}

- (-) GET     /api/asociations/{id}/publicDocuments
- (-) GET     /api/asociations/{id}/documents
- (-) POST    /api/asociations/{id}/documents
- (-) PATCH   /api/asociations/{id}/documents/{id}

- (-) GET     /api/asociations/{id}/materials
- (-) GET     /api/asociations/{id}/lendableMaterials
- (-) POST    /api/asociations/{id}/materials
- (-) PUT     /api/asociations/{id}/materials/{id}
- (-) DELETE  /api/asociations/{id}/materials/{id}

- (-) GET     /api/publicActivities
- (-) GET     /api/memberActivities
- (-) GET     /api/boardActivities

- (-) POST    /api/activities
- (-) GET     /api/activities/{id}
- (-) PUT     /api/activities/{id}
- (-) DELETE  /api/activities/{id}

- (-) POST    /api/activities/{id}/media
- (-) DELETE  /api/activities/{id}/media/{id}

- (?) POST    /api/activities/{id}/registration
- (?) DELETE  /api/activities/{id}/registration


## Auth API

- ( ) `/auth/standins`
- ( ) `/auth/signup`
- ( ) `/auth/activate`
- ( ) `/auth/login`
- ( ) `/auth/logout`
- ( ) `/auth/change_username`
- ( ) `/auth/change_password`



