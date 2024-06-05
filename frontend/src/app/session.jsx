'use client'

import { createContext, useContext, useReducer } from "react";

const SessionContext = createContext(null);
const SessionDispatchContext = createContext(null);

export function SessionProvider({ children }) {
  const [session, setSession] = useReducer(sessionReducer, initSession);

  return (
    <SessionContext.Provider value={session}>
      <SessionDispatchContext.Provider value={setSession}>
        {children}
      </SessionDispatchContext.Provider>
    </SessionContext.Provider>
  );
}

export function useSession() {
  return useContext(SessionContext);
}

export function useSessionDispatch() {
  return useContext(SessionDispatchContext);
}

function sessionReducer(_session, action) {
  console.log(action);
  switch (action.type) {
    case 'login': {
      console.log("logging in");
      return action.session
    }
    case 'logout': {
      console.log("logging out");
      return null
    }
  }
}

const initSession = {
  username: null,
  token: null
};