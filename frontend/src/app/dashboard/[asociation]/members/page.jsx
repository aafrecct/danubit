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
        <Link className="active" href={`/dashboard/${params.asociation}/members`}>
          <span>Miembros</span>
        </Link>
        <Link href={`/dashboard/${params.asociation}/membershipRequests`}>
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
      get(`api/asociations/${asociation_id}/members`,
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
          <span>Error cargando tus miembros.</span>
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
            <div key={mem.id}>
              <span className="listitem-name">{mem.user.name} {mem.surname}</span>
              <span className="listitem-desc">{mem.user.username}</span>
              <span className="listitem-date">Desde {mem.accepted_date}</span>
              <RemoveButton member={mem} />
            </div>
          ))
        }
      </div>)
  }
}

function RemoveButton({ member }) {

}