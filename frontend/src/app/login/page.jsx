'use client'

import { useState } from "react";
import { useSessionDispatch } from "@/app/session"
import process from "process";
import { home } from "@/app/actions";

function LoginButton({ loginInfo }) {
  const setSession = useSessionDispatch();
  const authUrl = `http://${process.env.apiHost}/auth/login`;

  function tryLogin() {
    console.log({ email: loginInfo.email, password: loginInfo.password });
    fetch(authUrl, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "POST",
      body: JSON.stringify({ email: loginInfo.email, password: loginInfo.password })
    })
      .then((response) => {
        if (!response.ok) {
          console.log("Wrong login info")
        }
        return response.json()
      })
      .then((session) => {
        setSession({
          type: "login",
          session: session
        });

        home()
      })
  }

  return (
    <button onClick={tryLogin}> Login </button>
  )
}

export default function LoginForm() {

  const [loginInfo, setLoginInfo] = useState({
    email: "",
    password: ""
  })

  function handleEmail(event) {
    setLoginInfo({
      email: event.target.value,
      password: loginInfo.password
    })
  }

  function handlePassword(event) {
    setLoginInfo({
      email: loginInfo.email,
      password: event.target.value
    })
  }

  return (
    <main id="login-backdrop">
      <div id="login-box">
        <span>Correo</span>
        <input name="email" onChange={handleEmail} />
        <span>Contraseña</span>
        <input name="email" onChange={handlePassword} type="password" />
        <LoginButton loginInfo={loginInfo} />
      </div>
    </main>
  );
}