import type { Metadata } from "next";
import "./globals.scss";
import NavBar from "./components/navbar/navbar";
import { SessionProvider } from "./session";

export const metadata: Metadata = {
  title: "Danubit",
  description: "A hub and organization platform for student asociations.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <SessionProvider>
          <NavBar />
          {children}
        </SessionProvider>
      </body>
    </html>
  );
}
