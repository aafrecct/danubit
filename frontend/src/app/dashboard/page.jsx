'use client'

import Link from "next/link";
import { useEffect, useState } from "react";
import { get } from '@/app/utils/api';
import { useSession } from '@/app/session';


export default function Dashboard() {
  const session = useSession();

  return (
    <main id="dashboard">
      <AsociationSidepanel session={session} />
      <div id="dashboard-content">
        <span>Elige una asocición en el panel de la izquierda.</span>
      </div>
    </main>
  )
}

function AsociationSidepanel({ session }) {
  const [state, setState] = useState({ state: "loading" });

  useEffect(() => {
    if (state.state == "loading") {
      get("api/session/board_of",
        session.token,
        (data) => {
          setState({
            state: "loaded",
            asociations: data
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
        <div id="dashboard-nav">
          <span>Error cargando tus asociaciones.</span>
        </div>
      );
    case "loading":
      return (
        <div id="dashboard-nav">
          <span>Cargando...</span>
        </div>
      );
    default:
      return (<div id="dashboard-nav">
        {
          state.asociations.map((asoc) => (
            <Link key={asoc.id} href={`dashboard/${asoc.id}/activities`}>
              <span>{asoc.short_name}</span>
            </Link>
          ))
        }
      </div>)
  }
}