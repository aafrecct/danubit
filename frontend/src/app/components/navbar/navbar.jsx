'use client'

import Link from 'next/link';
import { useSession } from '@/app/session';

function UserHeader() {
  const session = useSession();
  if (session && session.username) {
    return (
      <div>
        <Link id="bar-accountbutton" href="/dashboard" className="button">{session.username}</Link>
      </div>
    );
  }

  return (
    <div>
      <Link id="bar-login-bt" href="/login" className="button">Login</Link>
      <Link id="bar-signup-bt" href="/signup" className="button">Regístrate</Link>
    </div>
  );
}

export default function NavBar() {


  return (
    <header id="bar">
      <div id="bar-left">
        <Link id="bar-logo" href="/">
          <img
            src="/logo.svg"
            alt="Danubit Logo"
          />
        </Link>
        <ul id="bar-nav">
          <li>
            <Link href="/asociations">Asociaciones</Link>
          </li>
          <li>
            <Link href="/activities">Actividades</Link>
          </li>
        </ul>
      </div>
      <div id="bar-right">
        <UserHeader />
      </div>
    </header>
  );
}