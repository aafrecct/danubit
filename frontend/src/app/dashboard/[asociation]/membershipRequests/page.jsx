'use client'

import Link from "next/link";
import { useEffect, useState } from "react";
import { get } from '@/app/utils/api';
import { useSession } from '@/app/session';


export default function Dashboard({ params }) {
  const session = useSession();

  return (
    <main id="dashboard">
      <div id="dashboard-nav">
        <Link href={`/dashboard/${params.asociation}/activities`}>
          <span>Actividades</span>
        </Link>
        <Link href={`/dashboard/${params.asociation}/members`}>
          <span>Miembros</span>
        </Link>
        <Link className="active" href={`/dashboard/${params.asociation}/membershipRequests`}>
          <span>Solicitudes</span>
        </Link>
      </div>
      <div id="dashboard-content">
        <MemberTable session={session} asociation_id={params.asociation} />
      </div>
    </main>
  )
}

function MemberTable({ session, asociation_id }) {
  const [state, setState] = useState({ state: "loading" });

  useEffect(() => {
    if (state.state == "loading") {
      get(`api/asociations/${asociation_id}/membershipRequests`,
        session.token,
        (data) => {
          setState({
            state: "loaded",
            members: data
          });
        },
        (error) => {
          console.log(error);
          setState({
            state: "error",
          });
        }
      )
    }
  }, []);

  switch (state.state) {
    case "error":
      return (
        <div id="dashboard-list">
          <span>Error cargando las peticiones de membresía.</span>
        </div>
      );
    case "loading":
      return (
        <div id="dashboard-list">
          <span>Cargando...</span>
        </div>
      );
    default:
      return (<div id="dashboard-list">
        {
          state.members.map((mem) => (
            <ListItem key={mem.id} asociation_id={asociation_id} member={mem} session={session} />
          ))
        }
      </div>)
  }
}
function ListItem({ asociation_id, member, session }) {
  const [active, setActive] = useState(true);

  if (active) {
    return (
      <div className="membership-req">
        <span className="listitem-name">{member.user.name} {member.surname}</span>
        <span className="listitem-desc">{member.user.username}</span>
        <RemoveButton asociation={asociation_id} member={member} session={session} setActive={setActive} />
        <AcceptButton asociation={asociation_id} member={member} session={session} setActive={setActive} />
      </div>
    )
  }
}


function RemoveButton({ asociation, member, session, setActive }) {
  return (
    <button id="deny-button">Rechazar</button>
  )
}

function AcceptButton({ asociation, member, session, setActive }) {
  const acceptUrl = `http://${process.env.apiHost}/api/asociations/${asociation}/membershipRequests/${member.user.id}`

  function handleClick() {
    fetch(acceptUrl, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'X-API-Key': session.token
      },
      method: "PUT"
    })
      .then((response) => {
        if (!response.ok) {
          console.log(response);
        }
        setActive(false);
      })
  }

  return (
    <button id="accept-button" onClick={handleClick}>Aceptar</button>
  )
}