from requests import get, post, put, delete
from os import getenv
from sys import argv
from datetime import datetime, timedelta
from pprint import pprint

SERVICE_BASE_URL = getenv("DANUBIT_API_URL", "http://localhost:2345")

NOW = datetime.now()


def normalize_date(dt: datetime):
    return dt.replace(microsecond=0, second=0, minute=0).isoformat().replace("T", " ")


BASE_ASOCIATIONS = {
    "acm": {
        "short_name": "ACM",
        "long_name": "Capitulo de Estudiantes de la 'Asociation for Computer Machinery' de la UPM",
        "email": "acm@alumnos.fi.upm.es",
        "description": (
            "ACM-UPM es un capítulo de estudiantes de la asociación internacional "
            "ACM (Association for Computer Machinery), una de las más ilustres asociaciones "
            "en el mundo de la informática. Dicho capítulo fue fundado en nuestra escuela "
            "(actual ETSIINF, antigua Facultad de Informática, también conocida como FI) "
            "en el año 1983, y desde entonces no se ha cedido en el empeño de ampliar los "
            "conocimientos adquiridos en el grado y llevar más allá nuestras propias ambiciones."
        ),
        "is_public_joinable": True,
        "info": {
            "links": {
                "twitter": "acmupm",
                "instagram": "acmupm",
                "gmail": "acmfiupm@gmail.com",
                "telegram": "acmupm",
            }
        },
    },
    "ascfi": {
        "short_name": "ASCFI",
        "long_name": "Asociación Socio-Cultural de la Facultad de Informática",
        "email": "ascfi@alumnos.fi.upm.es",
        "description": (
            "Agrupación Sociocultural de la Facultad de Informática de la UPM. "
            "Somos gente tolerante, amistosa y temerosa de Dios. Torneos, videojuegos y risas inside!"
        ),
        "is_public_joinable": True,
        "info": {
            "links": {
                "twitter": "ascfiupm",
                "instagram": "ascfi.asoc.upm",
            }
        },
    },
    "rugfi": {
        "short_name": "Club Deportivo",
        "long_name": "Club Deportivo de la ETSIINF",
        "email": "cdi@alumnos.fi.upm.es",
        "description": ("Rugby, principalmente. Voley tambien."),
        "is_public_joinable": True,
        "info": {
            "links": {
                "twitter": "ClubDeportivoFI",
                "instagram": "rugfiupm",
            }
        },
    },
    "tuna": {
        "short_name": "Tuna",
        "long_name": "La Tuna de Informática",
        "email": "tuna@alumnos.fi.upm.es",
        "description": ("¿Donde encontrarnos? En los DMs de las alumnas de primero."),
        "is_public_joinable": True,
        "info": {},
    },
    "histrion": {
        "short_name": "Histrión",
        "long_name": "Histrión: Club de teatro",
        "email": "histrion@alumnos.fi.upm.es",
        "description": ("Club de teatro de la ETSIINF"),
        "is_public_joinable": True,
        "info": {
            "links": {
                "twitter": "HistrionUPM",
                "instagram": "histrionupm",
            }
        },
    },
}

BASE_USER = {
    "username": "admin",
    "email": "admin@danubit.com",
    "password": "admin",
}

BASE_ACTIVITIES = [
    {
        "asociations": ["ASCFI"],
        "people_in_charge": ["user1"],
        "activity": {
            "name": "Torneo de Smash",
            "description": ("Torneo de Smash Bros. Ultimate"),
            "room": "Sala de Asociaciones",
            "initial_date": normalize_date(NOW + timedelta(days=3)),
            "is_multi_session": False,
            "is_creditable": False,
            "is_external": False,
            "is_accepted": True,
            "is_room_accepted": True,
            "is_media_accepted": True,
            "is_registration_needed": True,
            "access": "Public",
            "additional_info": {},
        },
    },
    {
        "asociations": ["ACM"],
        "people_in_charge": ["user1"],
        "activity": {
            "name": "Charla de Python",
            "description": ("Charla de introducción a Python"),
            "room": "Hemiciclo H1003",
            "initial_date": normalize_date(NOW + timedelta(days=7)),
            "is_multi_session": False,
            "is_creditable": False,
            "is_external": False,
            "is_accepted": True,
            "is_room_accepted": True,
            "is_media_accepted": True,
            "is_registration_needed": False,
            "access": "Public",
            "additional_info": {},
        },
    },
    {
        "asociations": ["ACM"],
        "people_in_charge": ["user1"],
        "activity": {
            "name": "Charla de Git",
            "description": ("Charla de introducción a Git"),
            "room": "Hemiciclo 1002",
            "initial_date": normalize_date(NOW + timedelta(days=9)),
            "is_multi_session": False,
            "is_creditable": False,
            "is_external": False,
            "is_accepted": True,
            "is_room_accepted": True,
            "is_media_accepted": True,
            "is_registration_needed": False,
            "access": "Public",
            "additional_info": {},
        },
    },
    {
        "asociations": ["ASCFI"],
        "people_in_charge": ["user1"],
        "activity": {
            "name": "Torneo por parejas de Pokemon",
            "description": ("Torneo de Smash Bros. Ultimate"),
            "room": "Sala de Asociaciones",
            "initial_date": normalize_date(NOW + timedelta(days=9)),
            "is_multi_session": False,
            "is_creditable": False,
            "is_external": False,
            "is_accepted": True,
            "is_room_accepted": True,
            "is_media_accepted": True,
            "is_registration_needed": True,
            "access": "Public",
            "additional_info": {},
        },
    },
    {
        "asociations": ["Club Deportivo"],
        "people_in_charge": ["user1"],
        "activity": {
            "name": "Torneo Rugby Femenino",
            "description": ("Torneo de Smash Bros. Ultimate"),
            "room": "Sala de Asociaciones",
            "initial_date": normalize_date(NOW + timedelta(days=13)),
            "is_multi_session": False,
            "is_creditable": False,
            "is_external": False,
            "is_accepted": True,
            "is_room_accepted": True,
            "is_media_accepted": True,
            "is_registration_needed": True,
            "access": "Members",
            "additional_info": {},
        },
    },
]


def loginAsAdmin():
    login_response = post(
        f"{SERVICE_BASE_URL}/auth/login",
        json={"email": BASE_USER["email"], "password": BASE_USER["password"]},
    )
    if login_response.status_code != 200:
        print(login_response)
        return

    print(login_response)
    return login_response.json()


def createBaseAsociations():
    asociations = {}
    user = loginAsAdmin()
    for key, asociation in BASE_ASOCIATIONS.items():
        response = post(
            f"{SERVICE_BASE_URL}/api/asociations",
            json=asociation,
            headers={"Authorization": f"Bearer {user["token"]}"},
        )
        if response.ok:
            asociations[key] = response.json()
        else:
            print(f"asoc failed: {response.reason}")
    return asociations


def createBaseActivities():
    user = loginAsAdmin()
    asociations = {
        a["short_name"]: a for a in get(f"{SERVICE_BASE_URL}/api/asociations").json()
    }
    activities = []

    for activity in BASE_ACTIVITIES:
        response = post(
            f"{SERVICE_BASE_URL}/api/activities",
            json={
                "people_in_charge": [user["id"] for _ in activity["people_in_charge"]],
                "organizers": [asociations[a]["id"] for a in activity["asociations"]],
                "activity": activity["activity"],
            },
            headers={"Authorization": f"Bearer {user["token"]}"},
        )
        if response.ok:
            activities.append(response.json())
        else:
            print(activity, response.content, sep="\n")
            print("======")

    return activities


def addAdminToAsosBoards():
    user = loginAsAdmin()
    asociations = {
        a["id"]: a for a in get(f"{SERVICE_BASE_URL}/api/asociations").json()
    }
    members = []

    for asociation in asociations:
        print(f"{SERVICE_BASE_URL}/api/asociations/{asociation}/membershipRequests")
        post(
            f"{SERVICE_BASE_URL}/api/asociations/{asociation}/membershipRequests",
            json={
                "user_id": user["id"],
                "asociation": asociation,
            },
            headers={"Authorization": f"Bearer {user["token"]}"},
        ).ok
        member = put(
            f"{SERVICE_BASE_URL}/api/asociations/{asociation}/membershipRequests/{user["id"]}",
            headers={"X-API-Key": user["token"]},
        ).json()

        member["board_status"] = "Chair"
        response = put(
            f"{SERVICE_BASE_URL}/api/asociations/{asociation}/board/{user["id"]}",
            json=member,
            headers={"Authorization": f"Bearer {user["token"]}"},
        )
        if response.ok:
            members.append(response.json())

    return members


if __name__ == "__main__":
    try:
        print(f"Result: \n{locals()[argv[1]](*argv[2:])}")
    except Exception as e:
        print("Options are: ")
        print("\n".join(f"\t{n}" for n, f in locals().items() if callable(f)))
        raise
