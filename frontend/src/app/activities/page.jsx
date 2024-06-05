'use client'

import Link from "next/link";
import { useEffect, useState } from "react";
import { shortNameToLogoPath } from '@/app/utils/normalization';
import { get } from '@/app/utils/api'

export default function Activities() {
    const [activities, setActivities] = useState({
        state: "loading",
        activities: []
    })

    useEffect(() => {
        if (activities.state == "loading") {
            get("api/publicActivities",
                (data) => {
                    setActivities({
                        state: "loaded",
                        activities: data
                    });
                },
                (error) => {
                    console.log(error);
                    setActivities({
                        state: "error",
                        activities: []
                    });
                }
            )
        }
    }, []);

    return (
        <main id="activities-backdrop">
            <div id="activities">
                <ActivitiesList state={activities.state} activities={activities.activities} />
            </div>
        </main>
    )
}

function ActivitiesList({ state, activities }) {
    switch (state) {
        case "error":
            return (<span>Error loading activities.</span>);
        case "loading":
            return (<span>Loading activities...</span>);
        default:
            return activities.map((act) => (
                <div key={act.activity.id}>
                    <span id="activity-name">{act.activity.name}</span>
                    <span id="activity-desc">{act.activity.description}</span>
                    <span id="activity-room">{act.activity.room}</span>
                    <RegisterButton activity={act.activity} />
                </div>
            ));
    }
}

function RegisterButton({ activity }) {
    const [registered, setRegistered] = useState(false)
    if (activity.is_registration_needed) {
        return (<button onClick={() => setRegistered(true)}>
            {registered ? "Apuntado/a" : "Apuntarse"}
        </button>)
    } else {
        return;
    }
}