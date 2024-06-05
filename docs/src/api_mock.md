# Endpoints

## Regular API
- (x) GET     /api/managers

- (x) GET     /api/asociations
- (x) GET     /api/asociations/{id}
- (x) GET     /api/asociations/byName/{name}
- (x) PUT     /api/asociations/{id}
- (x) POST    /api/asociations

- (x) GET     /api/asociations/{id}/membershipRequests
- (x) POST    /api/asociations/{id}/membershipRequests
- (x) PATCH   /api/asociations/{id}/membershipRequests/{id}
- (x) DELETE  /api/asociations/{id}/membershipRequests/{id}

- (x) GET     /api/asociations/{id}/members
- (x) PUT     /api/asociations/{id}/members/{id}
- (x) DELETE  /api/asociations/{id}/members/{id}

- (x) GET     /api/asociations/{id}/publicDocuments
- (x) GET     /api/asociations/{id}/documents
- (-) POST    /api/asociations/{id}/documents
- (-) PATCH   /api/asociations/{id}/documents/{id}

- (x) GET     /api/asociations/{id}/materials
- (x) GET     /api/asociations/{id}/lendableMaterials
- (x) POST    /api/asociations/{id}/materials
- (x) PUT     /api/asociations/{id}/materials/{id}
- (x) DELETE  /api/asociations/{id}/materials/{id}

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



## Web paths`

- ( ) `/`
- ( ) `/asociations`
- ( ) `/asociation/[asociation]`
- ( ) `/activities`
- ( ) `/activities/[activity]`
- ( ) `/activities/[activity]/edit`
- ( ) `/login`
- ( ) `/register`
- ( ) `/dashboard/`
- ( ) `/dashboard/[asociation]`
- ( ) `/dashboard/[asociation]/members`
- ( ) `/dashboard/[asociation]/activity`
