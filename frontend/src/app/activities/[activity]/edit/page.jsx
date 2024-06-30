'use client'

import { useEffect, useState } from "react";
import { get } from "@/app/utils/api"
import { home } from "@/app/actions"


export default function ActivityEdit({ params }) {
  const activeFields = {
    name: ["Nombre", "string"],
    description: ["Descripción", "string"],
    room: ["Sala", "string"],
    initial_date: ["Fecha", "datetime"],
    is_registration_needed: ["Necesita registro", "boolean"]
  }

  const [state, setState] = useState({
    state: "loading",
    activity: {
      name: "",
      description: "",
      room: "",
      is_multi_session: false,
      is_creditable: false,
      is_external: true,
      is_accepted: true,
      is_room_accepted: true,
      is_media_accepted: true,
      is_registration_needed: true,
      access: "Public",
      initial_date: "",
      is_registration_needed: false
    },
    organizers: [],
    people_in_charge: []
  });

  const isNew = params.activity == "new";

  useEffect(() => {
    if (state.state == "loading") {
      if (!isNew) {
        get(`api/activities/${params.activity}`,
          null,
          (data) => {
            setState({
              state: "loaded",
              activity: data.activity,
              organizers: data.organizers,
              people_in_charge: data.people_in_charge
            });
          },
          (error) => {
            console.log(error);
            setState({
              ...state,
              state: "error",
            });
          }
        )
      } else {
        setState({
          ...state,
          state: "loaded",
        })
      }
    }
  })

  if (state.state === "loading") {
    return (
      <main id="activity-edit">
        <span>Loading...</span>
      </main>
    )
  }

  return (
    <main id="activity-edit">
      <div id="activity-form">
        {Object.keys(activeFields).map((key) =>
          <FormItem name={key} displayname={activeFields[key][0]} type={activeFields[key][1]} state={state} setState={setState} />
        )}
        <SubmitButton state={state} />
      </div>

    </main>
  )
}

function FormItem({ name, displayname, type, state, setState }) {
  let input;
  switch (type) {
    case "string":
      input = (
        <input type="text" value={state.activity[name]} onChange={(event) =>
          setState({ ...state, activity: { ...state.activity, [name]: event.target.value } })}>
        </input>);
      break;
    case "boolean":
      input = (
        <input type="checkbox" checked={state.activity[name] ? "checked" : ""} onChange={(event) =>
          setState({ ...state, activity: { ...state.activity, [name]: event.target.checked } })}>
        </input>);
      break;
    case "datetime":
      input = (
        <input type="datetime-local" value={state.activity[name]} onChange={(event) =>
          setState({ ...state, activity: { ...state.activity, [name]: event.target.value } })}>
        </input>);
      break;
  }

  return (
    <div>
      <span>{displayname}: </span>
      {input}
    </div>
  )
}

function SubmitButton({ state }) {
  return (
    <button onClick={() => home()}>
      Crear
    </button>
  )
}
