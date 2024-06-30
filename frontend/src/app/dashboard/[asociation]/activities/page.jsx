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
        <Link className="active" href={`/dashboard/${params.asociation}/activities`}>
          <span>Actividades</span>
        </Link>
        <Link href={`/dashboard/${params.asociation}/members`}>
          <span>Miembros</span>
        </Link>
        <Link href={`/dashboard/${params.asociation}/membershipRequests`}>
          <span>Solicitudes</span>
        </Link>
      </div>
      <div id="dashboard-content">
        <ActivityTable asociation_id={params.asociation} />
        <Link id="dashboard-new" href={"/activities/new/edit"}>
          <span>Nueva actividad</span>
        </Link>
      </div>
    </main>
  )
}

function ActivityTable({ asociation_id }) {
  const [state, setState] = useState({ state: "loading" });

  useEffect(() => {
    if (state.state == "loading") {
      get(`api/publicActivities?asociation_filter=${asociation_id}`,
        null,
        (data) => {
          setState({
            state: "loaded",
            activities: data
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
          <span>Error cargando tus asociaciones.</span>
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
          state.activities.map((act) => (
            <Link key={act.activity.id} href={`/activities/${act.activity.id}/edit`}>
              <span className="listitem-name">{act.activity.name}</span>
              <span className="listitem-desc">{act.activity.description}</span>
              <span className="listitem-date">{act.activity.initial_date}</span>
              <span className="listitem-room">{act.activity.room}</span>
            </Link>
          ))
        }
      </div>)
  }
}