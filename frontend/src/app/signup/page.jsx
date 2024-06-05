'use client'

import { useState } from "react";
import { useSessionDispatch } from "@/app/session"
import process from "process";
import { home } from "@/app/actions";

function SignupButton({ loginInfo }) {
  const authUrl = `http://${process.env.apiHost}/auth/signup`;

  function tryLogin() {
    fetch(authUrl, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "POST",
      body: JSON.stringify(loginInfo)
    })
      .then((response) => {
        if (!response.ok) {
          console.log("Wrong login info")
        }
        return response.json()
      })
      .then((_) => {
        home()
      })
  }

  return (
    <button onClick={tryLogin}> Registrarse </button>
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

        <SignupButton signupInfo={signupInfo} />
      </div>
    </main>
  );
}