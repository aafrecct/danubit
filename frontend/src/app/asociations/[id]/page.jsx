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
                <div id="asociation">
                    <div id="asociation-logo">
                        <img src={shortNameToLogoPath(data.asociation.short_name)} alt="No Media" />
                        <JoinButton asociation={data.asociation.id}></JoinButton>
                    </div>
                    <div id="asociation-info">
                        <span id="asociation-name">{data.asociation.short_name}</span>
                        <span id="asociation-lname">{data.asociation.long_name}</span>
                        <span id="asociation-desc">{data.asociation.description}</span>
                        <span id="asociation-link">Links</span>
                        {Object.keys(data.asociation.info.links || []).map((linkName) => (
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

function JoinButton({ asociation }) {
    const session = useSession();
    const joinUrl = `http://${process.env.apiHost}/api/asociations/${asociation}/membershipRequests`;
    const [joined, setJoined] = useState(false);


    function handleClick() {
        fetch(joinUrl, {
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'X-API-Key': session.token
            },
            method: "POST",
            body: JSON.stringify({
                user_id: session.id,
                asociation: asociation
            })
        })
            .then((response) => {
                if (!response.ok) {
                    console.log(response);
                }
                setJoined(true);
            })
    }

    if (!session.username) {
        return (
            <span className="button">Logeate para apuntarte</span>
        )
    }

    if (joined) {
        return <span className="button">Esperando aprovación</span>
    }

    return (
        <button onClick={handleClick}>
            Apúntate
        </button>
    )
}