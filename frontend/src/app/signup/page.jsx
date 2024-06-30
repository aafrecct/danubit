'use client'

import { useState } from "react";
import { useSessionDispatch } from "@/app/session"
import process from "process";
import { home } from "@/app/actions";

function SignupButton({ signupInfo }) {
  const authUrl = `http://${process.env.apiHost}/auth/signup`;

  function trySignup() {
    if (signupInfo.password !== signupInfo.repeat_password) {
      return
    }

    let { repeat_password, ...signup } = signupInfo;
    console.log(JSON.stringify(signup));

    fetch(authUrl, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "POST",
      body: JSON.stringify(signup)
    })
      .then((response) => {
        if (!response.ok) {
          console.log("Wrong signup info")
          console.log(response)
          return
        }
        home()
      })
  }

  return (
    <button onClick={trySignup}> Registrarse </button>
  )
}

export default function LoginForm() {

  const [signupInfo, setSignupInfo] = useState({
    email: "",
    password: ""
  })

  function handleField(field) {
    function handler(event) {
      setSignupInfo({
        ...signupInfo,
        [field]: event.target.value,
      })
    };
    return handler;
  }

  return (
    <main id="signup-backdrop">
      <div id="signup-box">
        <span>Nombre de usuario</span>
        <input name="email" onChange={handleField("username")} />

        <span>Nombre</span>
        <input name="email" onChange={handleField("name")} />

        <span>Apellidos</span>
        <input name="email" onChange={handleField("surname")} />

        <span>Correo</span>
        <input name="email" onChange={handleField("email")} />

        <span>Contraseña</span>
        <input name="email" onChange={handleField("password")} type="password" />

        <span>Repetir contraseña</span>
        <input name="email" onChange={handleField("repeat_password")} type="password" />

        <SignupButton signupInfo={signupInfo} />
      </div>
    </main>
  );
}