'use client'

import Link from "next/link";
import { useEffect, useState } from "react";
import process from "process";
import { shortNameToLogoPath } from "@/app/utils/normalization";
import { useSession } from "@/app/session";

export default function Page({ params }) {
    const apiUrl = `http://${process.env.apiHost}/api/asociations/${params.id}`;
    const [data, setData] = useState({
        state: "loading",
        asociation: []
    })

    useEffect(() => {
        console.log(params.id);
        if (data.state == "loading") {
            fetch(apiUrl)
                .then((response) => {
                    if (!response.ok) {
                        console.log(response);
                        setData({
                            state: "error",
                            asociation: {}
                        });
                    }
                    return response.json()
                })
                .then((data) => {
                    console.log(data);
                    setData({
                        state: "loaded",
                        asociation: data
                    });
                })
                .catch((error) => {
                    console.log(error);
                    setData({
                        state: "error",
                        asociation: {}
                    });
                })
        }
    }, []);

    return (
        <main id="asociation-backdrop">
            {data.state == "loaded" && (
                <div id="asociation-backdrop">
                    <div id="asociation-logo">
                        <img src={shortNameToLogoPath(data.asociation.short_name)} alt="No Media" />
                        <JoinButton></JoinButton>
                    </div>
                    <div id="asociation-info">
                        <span id="asociation-name">{data.asociation.short_name}</span>
                        <span id="asociation-lname">{data.asociation.long_name}</span>
                        <span id="asociation-desc">{data.asociation.description}</span>
                        <span id="asociation-link">Links</span>
                        {Object.keys(data.asociation.info.links).map((linkName) => (
                            <div key={linkName}>
                                <span className="asociation-link-name">{linkName}: </span>
                                <span className="asociation-link">{data.asociation.info.links[linkName]}</span>
                            </div>
                        ))}
                    </div>
                </div>
            )}
        </main>
    )
}

function JoinButton() {
    const session = useSession();

    function handleClick() {
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
 }