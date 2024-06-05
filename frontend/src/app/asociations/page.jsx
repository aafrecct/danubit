'use client'

import Link from "next/link";
import { useEffect, useState } from "react";
import process from "process";
import { shortNameToLogoPath } from '@/app/utils/normalization';
import { get } from '@/app/utils/api'

export default function Asociations() {
    const apiUrl = `http://${process.env.apiHost}/api/asociations`;
    const [asociations, setAsociations] = useState({
        state: "loading",
        asociations: []
    })

    useEffect(() => {
        if (asociations.state == "loading") {
            get("api/asociations",
                (data) => {
                    setAsociations({
                        state: "loaded",
                        asociations: data
                    });
                },
                (error) => {
                    console.log(error);
                    setAsociations({
                        state: "error",
                        asociations: []
                    });
                }
            )
        }
    }, []);

    return (
        <main id="asociations-backdrop">
            <div id="asociations">
                <AsociationsPanel state={asociations.state} asociations={asociations.asociations} />
            </div>
        </main>
    )
}

function AsociationsPanel({ state, asociations }) {
    switch (state) {
        case "error":
            return (<span>Error loading asociations.</span>);
        case "loading":
            return (<span>Loading asociations...</span>);
        default:
            return asociations.map((asoc) => (
                <Link key={asoc.id} href={`/asociations/${asoc.id}`}>
                    <div>
                        <img src={shortNameToLogoPath(asoc.short_name)} alt="No Media" />
                    </div>
                    <span>{asoc.short_name}</span>
                </Link>
            ));
    }
}